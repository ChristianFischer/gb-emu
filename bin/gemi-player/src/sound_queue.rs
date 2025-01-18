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

use libgemi::core::apu::audio_output::{AudioOutputSpec, SamplesReceiver};
use libgemi::core::apu::Apu;
use libgemi::core::apu::{audio_output, sample};
use sdl2::audio::*;
use sdl2::Sdl;


const SAMPLE_FREQ    : u32   = 48_000;
const CHANNEL_COUNT  : u8    = 2;
const BUFFER_SAMPLES : usize = audio_output::SAMPLE_BUFFER_SIZE;
const DEFAULT_VOLUME : f32   = 0.10;


/// SoundQueue to feed sound data into the audio device.
pub struct SoundQueue {
    /// The device for audio output
    audio_device:  AudioDevice<SoundQueueCallback>,

    /// Mirror value of the configured volume in the queue callback.
    /// Used to avoid unnecessary locking of the callback object.
    volume: f32,
}


/// SDL callback object to fetch audio samples.
struct SoundQueueCallback {
    /// Receiver object of the channel to receive audio samples from the backend.
    receiver: SamplesReceiver,

    /// The current volume.
    volume: f32,
}


impl SoundQueue {
    /// Creates a new SoundQueue
    pub fn create(sdl: &Sdl, apu: &mut Apu) -> Result<Self, String> {
        let sdl_audio = sdl.audio()?;

        let audio_spec = AudioSpecDesired {
            freq:     Some(SAMPLE_FREQ as i32),
            channels: Some(CHANNEL_COUNT),
            samples:  Some(BUFFER_SAMPLES as u16),
        };

        // open a channel to the APU backend to receive audio data
        let receiver = apu.get_audio_output().open_channel(
            AudioOutputSpec {
                sample_rate: SAMPLE_FREQ,
            }
        ).ok_or_else(
            || String::from("Cannot connect to emulator")
        )
        ?;

        let audio_device = sdl_audio.open_playback(
            None,
            &audio_spec,
            move |_| {
                SoundQueueCallback {
                    receiver,
                    volume: DEFAULT_VOLUME,
                }
            }
        )?;

        audio_device.resume();

        Ok (Self {
            audio_device,
            volume: DEFAULT_VOLUME
        })
    }


    /// Set the playback volume.
    pub fn set_volume(&mut self, volume: f32) {
        let volume_clamped = volume.clamp(0.0, 1.0);

        if self.volume != volume_clamped {
            self.volume = volume_clamped;

            self.audio_device.lock().volume = volume_clamped;
        }
    }


    /// Get the current playback volume.
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
}


impl Drop for SoundQueue {
    fn drop(&mut self) {
        self.audio_device.pause();
    }
}


impl AudioCallback for SoundQueueCallback {
    type Channel = sample::SampleType;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        let result = self.receiver.try_recv();

        match result {
            Ok(samples) => {
                for i in 0..BUFFER_SAMPLES {
                    out[i * 2 + 0] = self.volume * samples[i].left.get_value();
                    out[i * 2 + 1] = self.volume * samples[i].right.get_value();
                }
            }

            Err(_) => {
                out.iter_mut().for_each(|x| *x = 0.0);
            }
        }
    }
}
