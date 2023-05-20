/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use std::cmp::min;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::apu::hpf::StereoHighPassFilters;
use crate::apu::sample::{SampleResult, StereoSample};
use crate::cpu::cpu::CPU_CLOCK_SPEED;
use crate::gameboy::{Clock, DeviceConfig};


/// The size of the sample buffer to be transmitted to the receiver of generated audio data.
pub const SAMPLE_BUFFER_SIZE: usize = 1024;


/// Type alias for an array storing sample data generated by the APU.
pub type SampleBuffer = [StereoSample; SAMPLE_BUFFER_SIZE];

/// Sender part of the channel to transfer audio samples from the APU to the consumer.
pub type SamplesSender = Sender<Box<SampleBuffer>>;

/// Receiver part of the channel to transfer audio samples from the APU to the consumer.
pub type SamplesReceiver = Receiver<Box<SampleBuffer>>;


/// Stores the configuration to initialize the audio generation.
pub struct AudioOutputSpec {
    pub sample_rate: u32,
}


/// A buffer object receiving the audio data generated by the APU.
pub struct AudioOutput {
    /// The sample rate currently configured to generate audio data.
    sample_rate: u32,

    /// Records the time passed to check when to record a new sample.
    time_passed: Clock,

    /// Timestamp when the next sample has to be recorded.
    next_sample_time: Clock,

    /// The multiplier to be multiplied with the sample data
    /// in order to compute the average over n CPU cycles.
    sample_multiplier: f32,

    /// The current sample to be accumulated from sample data over n CPU cycles.
    current_sample: StereoSample,

    /// The buffer to record sample data and transfer them to the emulator frontend.
    buffer: Box<SampleBuffer>,

    /// The position where to insert the next sample.
    buffer_insert_pos: usize,

    /// Highpass filters for left and right channels to filter the output samples.
    high_pass_filter: StereoHighPassFilters,

    /// Sender part of the channel to transfer sample data to the emulator frontend.
    sender: Option<SamplesSender>
}


impl AudioOutput {
    pub const DEFAULT_SAMPLE_RATE: u32 = 48_000;


    pub fn new(device_config: DeviceConfig) -> Self {
        Self {
            sample_rate:        Self::DEFAULT_SAMPLE_RATE,
            time_passed:        0,
            next_sample_time:   0,
            sample_multiplier:  0.0,
            current_sample:     StereoSample::default(),
            buffer:             Box::new([StereoSample::default(); SAMPLE_BUFFER_SIZE]),
            buffer_insert_pos:  0,
            high_pass_filter:   StereoHighPassFilters::new(device_config),
            sender:             None,
        }
    }


    /// Get the sample rate configured for this output buffer.
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }


    /// Push a new sample into the buffer.
    /// Takes a sample as read from the APU channels and the number of cycles
    /// this sample was live.
    /// To produce sample data with the requested sample rate, the audio output will
    /// compute the average of all values within `CPU_CLOCK / sample rate` cycles.
    pub(crate) fn push(&mut self, sample: SampleResult<StereoSample>, cycles: Clock) {
        let mut remaining_cycles = cycles;

        while remaining_cycles > 0 {
            let time_to_next_sample = self.next_sample_time - self.time_passed;
            let run_cycles          = min(remaining_cycles, time_to_next_sample);

            // filter the sample data via high pass filter
            let sample_filtered = self.high_pass_filter.filter(sample);

            // sample data will be accumulated to produce the average value for one sample
            self.current_sample += sample_filtered * (run_cycles as f32) * self.sample_multiplier;

            // check if enough data collected to complete a sample
            self.time_passed += run_cycles;
            if self.time_passed >= self.next_sample_time {
                // write the sample into the buffer
                self.finish_sample();

                // wrap after a second has passed to prevent an overflow
                if self.next_sample_time > CPU_CLOCK_SPEED {
                    self.next_sample_time -= CPU_CLOCK_SPEED;
                    self.time_passed -= CPU_CLOCK_SPEED;
                }

                // compute the time when the next sample has to be generated
                let sample_rate           = self.sample_rate as Clock;
                let last_sample_time      = self.next_sample_time;
                let current_sample_number = (self.next_sample_time + 1) * sample_rate / CPU_CLOCK_SPEED;
                let next_sample_number    = current_sample_number + 1;
                let next_sample_time      = next_sample_number * CPU_CLOCK_SPEED / sample_rate;
                self.next_sample_time     = next_sample_time;

                // compute a multiplier, the sample has to be multiplied with in order to get
                // the average over all values until this sample has been completed.
                let time_diff  = next_sample_time - last_sample_time;
                let multiplier = 1.0 / (time_diff as f32);
                self.sample_multiplier = multiplier;
            }

            remaining_cycles -= run_cycles;
        }
    }


    /// After collecting data for one sample, this pushes the current sample into the samples buffer
    /// and, if reached the end of the buffer, sends it to a receiver object.
    fn finish_sample(&mut self) {
        // store the sample on the current position inside the sample buffer
        self.buffer[self.buffer_insert_pos] = self.current_sample;

        // reset the sample accumulator
        self.current_sample = StereoSample::default();

        // increment the write position
        self.buffer_insert_pos += 1;

        // if we reached the end of the buffer ...
        if self.buffer_insert_pos >= SAMPLE_BUFFER_SIZE {
            // send it to the receiver, if any channel was opened
            if let Some(sender) = &self.sender {
                let result = sender.send(self.buffer.clone());

                // disconnect on error
                if result.is_err() {
                    self.sender = None;
                }
            }

            // and reset the insert position
            self.buffer_insert_pos = 0;
        }
    }


    /// Open a channel in order to receive audio samples from the emulator backend.
    /// This function requires to specify a sample rate and returns a receiver object
    /// which will receive all samples generated by the APU.
    pub fn open_channel(&mut self, spec: AudioOutputSpec) -> Option<SamplesReceiver> {
        let (s, r) = channel::<Box<SampleBuffer>>();

        assert_ne!(spec.sample_rate, 0);

        if spec.sample_rate > 0 {
            self.sample_rate = spec.sample_rate;
            self.sender      = Some(s);

            Some(r)
        }
        else {
            None
        }
    }
}
