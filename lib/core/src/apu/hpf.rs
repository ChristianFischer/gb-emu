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

use crate::apu::sample::{Sample, SampleResult, StereoSample};
use crate::device_type::{DeviceConfig, DeviceType};


/// Charge factor per frame on classic GameBoy models.
const CAPACITOR_CHARGE_FACTOR_BASE_DMG : f32 = 0.999958;

/// Charge factor per frame on GameBoy Color models.
const CAPACITOR_CHARGE_FACTOR_BASE_GBC : f32 = 0.998943;


/// The Highpass Filter tries to pull an input value towards zero
/// to neutralize the offset of inactive but still enabled channels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HighPassFilter {
    capacitor: f32,
    charge_factor: f32,
}


/// A set of two [HighPassFilter], one for the left, one for the right channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StereoHighPassFilters {
    filter_left:  HighPassFilter,
    filter_right: HighPassFilter,
}


impl HighPassFilter {
    pub fn new(device_config: DeviceConfig) -> Self {
        let charge_factor = match device_config.device {
            DeviceType::GameBoyDmg => CAPACITOR_CHARGE_FACTOR_BASE_DMG,
            _                      => CAPACITOR_CHARGE_FACTOR_BASE_GBC,
        };

        Self {
            capacitor: 0.0,
            charge_factor,
        }
    }


    /// Filters the input value of a sample, which may be a valid audio signal or a silence value,
    /// if no DAC was enabled, and returns the filtered sample value.
    pub fn filter(&mut self, sample: SampleResult<Sample>) -> Sample {
        if let SampleResult::Audio(sample) = sample {
            let sample_value     = sample.get_value();
            let sample_out_value = sample_value - self.capacitor;

            self.capacitor = sample_value - (sample_out_value * self.charge_factor);

            Sample::new(sample_out_value)
        }
        else {
            Sample::default()
        }
    }
}


impl StereoHighPassFilters {
    pub fn new(device_config: DeviceConfig) -> Self {
        Self {
            filter_left:  HighPassFilter::new(device_config),
            filter_right: HighPassFilter::new(device_config),
        }
    }


    /// Filters the value of both left and right channels and returns a [StereoSample]
    /// with the filtered result of both channels.
    pub fn filter(&mut self, sample: SampleResult<StereoSample>) -> StereoSample {
        StereoSample {
            left:  self.filter_left.filter(sample.get_left()),
            right: self.filter_right.filter(sample.get_right()),
        }
    }
}
