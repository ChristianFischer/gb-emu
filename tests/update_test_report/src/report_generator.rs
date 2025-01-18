/*
 * Copyright (C) 2022-2025 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use futures::{future, StreamExt};
use libgemi::core::device_type::DeviceType;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tests_shared::io_utils::{IteratorState, TestConfigVisitor, Workspace};
use tests_shared::runner::{run_test_case_safe, TestCaseError};
use tests_shared::test_config::{EmulatorTestCase, EmulatorTestConfig};


/// A result code indicating whether a test was successful or not
/// with some additional information on the reason of failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TestResult {
    Success,
    Failed,
    Error,
    Panic,
}


/// A struct containing the statistics of how much tests ended in each state.
#[derive(Default)]
pub struct TestStats {
    pub success:    u32,
    pub failed:     u32,
    pub errors:     u32,
    pub panicked:   u32,
}


/// An enum containing either a fixed string or a test case referenced by it's index.
enum StringOrTestCaseRef {
    /// A string to be added into the report file.
    String(String),

    /// A reference to a test case, which result will be added to the report file.
    TestCaseRef(usize),
}


/// A set of test cases which belong to a single test ROM.
/// Used to stack test cases of the same ROM, which are considered as
/// different test configurations otherwise.
pub struct TestCaseSet {
    rom: String,
    test_cases: HashMap<DeviceType, EmulatorTestCase>,
}


/// An object used to generate unit tests for the emulator based on the results of the file visitors.
pub struct TestReportGenerator {
    /// The workspace where to find test ROMs.
    workspace: Workspace,

    /// The pending headline of the latest module found.
    /// Will only be submitted, when followed by any test case.
    pending_headline: Option<String>,

    /// The last test cases found. Stored until the current module gets closed
    /// or other test cases will be found which do not belong to the same ROM file.
    pending_tests: Option<TestCaseSet>,

    /// All entries found, which will be either a fixed string or a test case to be run.
    entries: Vec<StringOrTestCaseRef>,

    /// The collection of test cases to be run.
    tests: Vec<EmulatorTestCase>,

    /// The results of all tests after they were being executed.
    test_results: Vec<TestResult>,
}


impl TestResult {
    /// Get an icon displaying the test result.
    pub fn get_icon(&self) -> &str {
        match self {
            TestResult::Success       => "✔️",
            TestResult::Failed        => "❌",
            TestResult::Error         => "⚠️",
            TestResult::Panic         => "☠️",
        }
    }
}



impl TestReportGenerator {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            workspace:          workspace.clone(),
            pending_headline:   None,
            pending_tests:      None,
            entries:            Vec::new(),
            tests:              Vec::new(),
            test_results:       Vec::new(),
        }
    }


    /// Helper function to create a string containing the table header
    /// with the name of each device being tested.
    pub fn create_devices_header() -> String {
        let mut line1 = String::new();
        let mut line2 = String::new();

        line1.push_str("|                                                                          |");
        line2.push_str("|--------------------------------------------------------------------------|");

        for device in DeviceType::ALL_DEVICES {
            line1.push_str(&format!(" {:4} |", device.get_abbreviation()));
            line2.push_str(":----:|");
        }

        format!("{}\n{}", line1, line2)
    }


    /// Get the number of test cases tracked by this generator.
    pub fn get_test_case_count(&self) -> usize {
        self.tests.len()
    }


    /// Adds an string entry immediately.
    pub fn add_str_entry(&mut self, s: &str) {
        self.add_string_entry(s.to_string());
    }


    /// Adds an string entry immediately.
    pub fn add_string_entry(&mut self, s: String) {
        self.entries.push(StringOrTestCaseRef::String(s));
    }


    /// Set the headline of the current module.
    /// The headline will be submitted once any tests were found.
    pub fn set_headline(&mut self, headline: String) {
        self.pending_headline = Some(headline);
    }


    /// Clear the currently pending headline, if any.
    pub fn clear_headline(&mut self) {
        self.pending_headline = None;
    }


    /// Submit the current headline, if any pending.
    /// The pending headline will be cleared after this.
    fn submit_headline_if_any(&mut self) {
        if let Some(headline) = self.pending_headline.take() {
            let headline_formatted = format!("**{}**", headline);
            let mut line = String::new();

            line.push_str(&format!("| {:72} |", headline_formatted));

            for _ in DeviceType::ALL_DEVICES {
                line.push_str("      |");
            }

            self.add_string_entry(format!("{}\n", line));

            self.clear_headline();
        }
    }


    /// Submits any pending set of test cases, if any.
    /// Clears the pending tests after submitting them.
    fn submit_tests(&mut self) {
        if let Some(pending_test_set) = std::mem::replace(&mut self.pending_tests, None) {
            let rom_file = Self::get_filename_from_path(&pending_test_set.rom);

            // any pending headline needs to be submitted first
            self.submit_headline_if_any();

            // add the string containing the ROM file name
            self.add_string_entry(format!("| {:72} |", rom_file));

            // check all possible device types
            for device_type in DeviceType::ALL_DEVICES {
                let entry = if let Some(test_case) = pending_test_set.test_cases.get(&device_type) {
                    // verify the test case contains the same rom file as our test set
                    assert_eq!(pending_test_set.rom, test_case.setup.cartridge_path);

                    // enlist the test case and remember its index
                    let index = self.tests.len();
                    self.tests.push(test_case.clone());

                    StringOrTestCaseRef::TestCaseRef(index)
                }
                else {
                    // produce an empty cell for each not covered device
                    let result_string = String::from("      |");
                    StringOrTestCaseRef::String(result_string)
                };

                self.entries.push(entry);
            }

            self.add_str_entry("\n");
        }
    }


    /// Checks whether any new tests may be stacked with the currently pending tests,
    /// submits the pending tests if stacking is not possible
    fn submit_tests_if_not_stackable(&mut self, rom: &String, devices: &HashSet<DeviceType>) {
        if !self.can_stack_tests(rom, devices) {
            self.submit_tests();
        }
    }


    /// Checks whether any new tests may be stacked with the currently pending tests.
    fn can_stack_tests(&self, rom: &String, devices: &HashSet<DeviceType>) -> bool {
        if let Some(pending_test_set) = &self.pending_tests {
            // cannot stack with different rom files
            if !pending_test_set.rom.eq(rom) {
                return false;
            }

            // check if any device is already present in the pending set
            for device in devices {
                if pending_test_set.test_cases.contains_key(&device) {
                    // cannot stack if any device is already added
                    return false;
                }
            }

        }

        true
    }


    /// Runs a test, return with a test result.
    pub fn run_test(workspace: &Workspace, test_case: &EmulatorTestCase) -> TestResult {
        match run_test_case_safe(workspace, test_case) {
            Ok(_)  => TestResult::Success,
            Err(e) => match e {
                TestCaseError::Failed(_)     => TestResult::Failed,
                TestCaseError::SetUpError(_) => TestResult::Error,
                TestCaseError::Panic(_)      => TestResult::Panic,
            },
        }
    }


    /// Get the ROM file name out of a path reference.
    pub fn get_filename_from_path(path: &str) -> String {
        let path_normalized = path.replace("\\", "/");

        if let Some(idx) = path_normalized.rfind("/") {
            path_normalized[idx + 1..].to_string()
        }
        else {
            path_normalized.to_string()
        }
    }


    /// Runs all tests which were found in parallel.
    /// As a result of this function, the list of test results will be filled.
    pub async fn run_all_tests(&mut self, bar: ProgressBar) {
        let workspace = Arc::new(self.workspace.clone());

        // tokio stream of test cases
        let test_results_stream =
            // build a tokio stream based on clones of each test case,
            // which can be sent to worker threads
            tokio_stream::iter(self.tests.clone().into_iter())

            // run all tests within a tokio task and return it's result
            .map(|test_case| {
                // create a clone of the Arc to be moved into the async closure
                let workspace = workspace.clone();
                let bar = bar.clone();

                tokio::spawn(async move {
                    // log the current test
                    bar.set_message(
                        PathBuf::from(
                            test_case
                            .setup.cartridge_path
                            .clone()
                        )
                        .file_name().unwrap()
                        .to_str().unwrap()
                        .to_string()
                    );

                    // run the actual test
                    let result = Self::run_test(&workspace, &test_case);

                    // increment progress bar
                    bar.inc(1);

                    // return the result
                    result
                })
            })
        ;

       // collect all test results
        let test_results = future::join_all(
            // take all futures from the stream and collect them into a vector
            test_results_stream
                .collect::<Vec<_>>().await
                .into_iter()
        )
            .await
            .into_iter()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>()
        ;

        self.test_results = test_results;
    }


    /// Collects statistics about the test results.
    /// Expects the tests to be run first.
    pub fn collect_stats(&self) -> TestStats {
        self.test_results
            .iter()
            .fold(
                TestStats::default(),
                |mut stats, result| {
                    match result {
                        TestResult::Success => stats.success  += 1,
                        TestResult::Error   => stats.errors   += 1,
                        TestResult::Failed  => stats.failed   += 1,
                        TestResult::Panic   => stats.panicked += 1,
                    }

                    stats
                }
            )
    }


    /// Generates the content of the test report file.
    /// Expects the tests to be run first.
    pub fn generate_file_content(&self, title: &str) -> String {
        format!(
            // language=markdown
            r#"## {} Test ROM Results

{}
{}
"#,
            title,
            Self::create_devices_header(),
            self.entries.iter()
                .map(|e| match e {
                    StringOrTestCaseRef::String(s) => s.clone(),

                    StringOrTestCaseRef::TestCaseRef(index) => {
                        let result = self.test_results[*index];
                        format!("  {}   |", result.get_icon())
                    }
                })
                .collect::<String>()
        )
    }
}


impl TestConfigVisitor for TestReportGenerator {
    fn on_open_module(&mut self, module_name: &str, state: &IteratorState) {
        let path = format!("{}/{}", state.path, module_name);
        self.set_headline(path);
    }


    fn on_close_module(&mut self, _module_name: &str, _state: &IteratorState) {
        // if any tests are pending, apply them before leaving the module
        self.submit_tests();
    }


    fn on_visit_test(&mut self, test_cfg: &EmulatorTestConfig, _state: &IteratorState) {
        let rom = &test_cfg.setup.cartridge_path;

        // submit any previous test, except if the new tests belong to the same ROM and the device
        // list can be stacked with the already pending tests
        self.submit_tests_if_not_stackable(&rom, &test_cfg.devices);

        // create test set, if none exists yet
        if self.pending_tests.is_none() {
            self.pending_tests = Some(TestCaseSet {
                rom:        rom.clone(),
                test_cases: HashMap::new(),
            })
        }

        // add all test cases into the existing set
        if let Some(pending_tests) = &mut self.pending_tests {
            for test_case in test_cfg.get_test_cases() {
                pending_tests.test_cases.insert(test_case.device, test_case);
            }
        }
    }
}
