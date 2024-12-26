/*
 * Copyright (C) 2022-2024 by Christian Fischer
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

use core::ops;

/// The data type to be used to store audio sample data.
pub type SampleType = f32;


/// A sample of a single channel as to be transmitted to the emulator frontend.
#[derive(Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sample {
    value: SampleType
}


/// A sample of left and right channel as to be transmitted to the emulator frontend.
#[derive(Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StereoSample {
    pub left: Sample,
    pub right: Sample,
}


/// The result of taking a sample from the APU channels.
/// This may be sound data or silence, if no DAC was enabled.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleResult<T>
    where T : Copy + Clone
{
    /// An audio sample was produced by the source.
    Audio(T),

    /// No audio data was produced and the audio source is silent.
    Silence,
}


impl Sample {
    pub fn new(value: SampleType) -> Self {
        Self {
            value
        }
    }


    /// Get this sample's value.
    pub fn get_value(&self) -> SampleType {
        self.value
    }
}


impl ops::Add<Sample> for Sample {
    type Output = Sample;

    fn add(self, rhs: Sample) -> Self::Output {
        Sample {
            value: self.value + rhs.value,
        }
    }
}


impl ops::AddAssign<Sample> for Sample {
    fn add_assign(&mut self, rhs: Sample) {
        self.value += rhs.value;
    }
}


impl ops::Mul<SampleType> for Sample {
    type Output = Sample;

    fn mul(self, rhs: SampleType) -> Self::Output {
        Sample {
            value: self.value * rhs,
        }
    }
}


impl ops::Div<SampleType> for Sample {
    type Output = Sample;

    fn div(self, rhs: SampleType) -> Self::Output {
        Sample {
            value: self.value / rhs,
        }
    }
}



impl ops::Add<StereoSample> for StereoSample {
    type Output = StereoSample;

    fn add(self, rhs: StereoSample) -> Self::Output {
        StereoSample {
            left:  self.left  + rhs.left,
            right: self.right + rhs.right,
        }
    }
}


impl ops::AddAssign<StereoSample> for StereoSample {
    fn add_assign(&mut self, rhs: StereoSample) {
        self.left  += rhs.left;
        self.right += rhs.right;
    }
}


impl ops::Mul<SampleType> for StereoSample {
    type Output = StereoSample;

    fn mul(self, rhs: SampleType) -> Self::Output {
        StereoSample {
            left:  self.left  * rhs,
            right: self.right * rhs,
        }
    }
}


/// Implementation for a [SampleResult] for a [StereoSample] to retrieve a SampleResult for each
/// left and right channels.
impl SampleResult<StereoSample> {
    /// Get the [SampleResult] for the left channel.
    pub fn get_left(&self) -> SampleResult<Sample> {
        match self {
            SampleResult::Audio(sample) => SampleResult::Audio(sample.left),
            SampleResult::Silence       => SampleResult::Silence
        }
    }


    /// Get the [SampleResult] for the right channel.
    pub fn get_right(&self) -> SampleResult<Sample> {
        match self {
            SampleResult::Audio(sample) => SampleResult::Audio(sample.right),
            SampleResult::Silence       => SampleResult::Silence
        }
    }
}
