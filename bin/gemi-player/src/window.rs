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

extern crate sdl2;

use crate::sound_queue::SoundQueue;
use libgemi::core::input::{Input, InputButton};
use libgemi::core::mmu::locations::MEMORY_LOCATION_SPRITES_BEGIN;
use libgemi::core::ppu::flags::LcdControlFlag;
use libgemi::core::ppu::graphic_data::{Color, DmgPalette, TileMap, TileSet};
use libgemi::core::ppu::ppu::{LcdBuffer, Ppu, SCREEN_H, SCREEN_W};
use libgemi::GameBoy;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, UpdateTextureError, WindowCanvas};


#[derive(PartialEq)]
pub enum DisplayMode {
    Game,
    Background,
    Objects,
}


pub enum State {
    Open,
    Closed,
}


/// A texture with a separate color buffer to store pixel data
/// until it will be transferred into the texture GPU memory.
pub struct BufferedTexture {
    width:      u32,
    height:     u32,
    buffer:     Vec<u8>,
    texture:    Texture,
}


/// A window to present the Gameboy's output.
pub struct Window {
    display_scale:      u32,
    event_pump:         sdl2::EventPump,
    canvas:             WindowCanvas,
    texture_game:       BufferedTexture,
    texture_background: BufferedTexture,
    texture_objects:    BufferedTexture,
    state:              State,
    display_mode:       DisplayMode,
    key_bindings:       KeyBindings,
    audio:              SoundQueue,
}


/// Alias type for the key bindings used by this window.
type KeyBindings = gemi_utils::keybindings::KeyBindings<Keycode>;

/// Create the keybindings used by this window.
fn make_keybindings() -> KeyBindings {
    KeyBindings::with_mapping(
        vec![
            (InputButton::DPadRight,    vec![Keycode::D,    Keycode::Right  ]),
            (InputButton::DPadLeft,     vec![Keycode::A,    Keycode::Left   ]),
            (InputButton::DPadUp,       vec![Keycode::W,    Keycode::Up     ]),
            (InputButton::DPadDown,     vec![Keycode::S,    Keycode::Down   ]),
            (InputButton::A,            vec![Keycode::E,    Keycode::X      ]),
            (InputButton::B,            vec![Keycode::Q,    Keycode::Y      ]),
            (InputButton::Select,       vec![Keycode::Num1, Keycode::LShift ]),
            (InputButton::Start,        vec![Keycode::Num2, Keycode::Return ]),
        ]
    )
}


impl BufferedTexture {
    /// Creates a new texture from a TextureCreator with a specific size.
    pub fn new<T>(texture_creator: &TextureCreator<T>, width: u32, height: u32) -> Result<BufferedTexture, String> {
        let size = (width * height * 4) as usize;

        let texture = texture_creator
            .create_texture_streaming(None, width, height)
            .map_err(|e| e.to_string())
            ?
        ;

        Ok(
            BufferedTexture {
                width,
                height,
                buffer: vec![0xff; size],
                texture,
            }
        )
    }

    /// Get the texture width.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the texture height.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Set a color value for any specific pixel.
    pub fn set_color(&mut self, x: u32, y: u32, color: &Color) {
        let offset = ((x + (y * self.width)) * 4) as usize;

        self.buffer[offset + 0] = color.b;
        self.buffer[offset + 1] = color.g;
        self.buffer[offset + 2] = color.r;
        self.buffer[offset + 3] = color.a;
    }

    /// Updates the texture with the pixel data in the current buffer.
    pub fn update_texture(&mut self) -> Result<(), UpdateTextureError> {
        self.texture.update(
            None,
            &self.buffer,
            (self.width * 4) as usize
        )
    }

    /// Copy the texture content into the given canvas.
    pub fn copy_to_canvas(&self, canvas: &mut WindowCanvas, display_scale: u32) -> Result<(), String> {
        canvas.copy(
            &self.texture,
            Rect::new(0, 0, self.width, self.height),
            Rect::new(0, 0, self.width * display_scale, self.height * display_scale)
        )
    }
}


impl Window {
    /// Creates a new window with a given size and title.
    pub fn create(title: &str, gb: &mut GameBoy) -> Result<Window, String> {
        let display_scale = 4;

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let event_pump = sdl.event_pump()?;

        let window = video
            .window(title, SCREEN_W * display_scale, SCREEN_H * display_scale)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            ?
        ;

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            ?
        ;

        let texture_creator    = canvas.texture_creator();
        let texture_game       = BufferedTexture::new(&texture_creator, SCREEN_W, SCREEN_H)?;
        let texture_background = BufferedTexture::new(&texture_creator, 256, 256)?;
        let texture_objects    = BufferedTexture::new(&texture_creator, 16*8, 24*8)?;

        let audio = SoundQueue::create(&sdl, gb.get_apu_mut())?;

        Ok(Window {
            display_scale,
            event_pump,
            canvas,
            texture_game,
            texture_background,
            texture_objects,
            state: State::Open,
            display_mode: DisplayMode::Game,
            key_bindings: make_keybindings(),
            audio,
        })
    }


    /// Checks whether the window is open.
    pub fn is_opened(&self) -> bool {
        match self.state {
            State::Open   => true,
            State::Closed => false,
        }
    }


    /// Close the window.
    pub fn close(&mut self) {
        self.state = State::Closed;
    }

    /// Polls and handles events of this window.
    pub fn poll_events(&mut self) {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    self.close();
                }

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.handle_key_down(keycode);
                }

                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.handle_key_up(keycode);
                }

                _ => { }
            }
        }
    }


    fn handle_key_down(&mut self, keycode: Keycode) {
        // set the emulator button states
        self.key_bindings.set_key_pressed(
            keycode,
            true
        );

        // handle key events for the player application itself
        match keycode {
            Keycode::Escape => { self.close(); },
            Keycode::F1     => { self.set_display_mode(DisplayMode::Game); }
            Keycode::F2     => { self.set_display_mode(DisplayMode::Background); }
            Keycode::F3     => { self.set_display_mode(DisplayMode::Objects); }

            Keycode::KpMinus => {
                let volume = self.audio.get_volume();
                self.audio.set_volume(volume - 0.05);
            }

            Keycode::KpPlus => {
                let volume = self.audio.get_volume();
                self.audio.set_volume(volume + 0.05);
            }

            _ => { }
        }
    }


    fn handle_key_up(&mut self, keycode: Keycode) {
        self.key_bindings.set_key_pressed(
            keycode,
            false
        );
    }


    /// Set the pressed state for gameboy buttons.
    pub fn apply_button_states(&self, input: &mut Input) {
        self.key_bindings.apply_button_states_to_input(input);
    }


    /// Switches the display mode to display the selected content.
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        if self.display_mode != mode {
            self.display_mode = mode;

            let target_texture = match self.display_mode {
                DisplayMode::Game       => &self.texture_game,
                DisplayMode::Background => &self.texture_background,
                DisplayMode::Objects    => &self.texture_objects,
            };

            self.canvas.window_mut().set_size(
                self.display_scale * target_texture.get_width(),
                self.display_scale * target_texture.get_height()
            ).unwrap();
        }
    }

    /// Presents the content of a LCD buffer on the window.
    pub fn present(&mut self, lcd: &LcdBuffer, ppu: &Ppu) {
        match self.display_mode {
            DisplayMode::Game       => self.present_game(lcd),
            DisplayMode::Background => self.present_background(ppu),
            DisplayMode::Objects    => self.present_objects(ppu),
        }
    }


    /// Present the current LCD buffer content on the screen.
    /// This will be the content as it would be displayed to the player.
    fn present_game(&mut self, lcd: &LcdBuffer) {
        // convert palette based image data into RGBA
        for y in 0..SCREEN_H {
            for x in 0..SCREEN_W {
                let color = lcd.get_pixel(x, y);
                self.texture_game.set_color(x, y, &color);
            }
        }

        // update texture
        self.texture_game.update_texture()
            .map_err(|e| e.to_string())
            .unwrap()
        ;

        // copy texture into framebuffer
        self.texture_game.copy_to_canvas(&mut self.canvas, self.display_scale).unwrap();

        // present the framebuffer
        self.canvas.present();
    }


    /// Present the whole background on the screen.
    /// This includes the whole content even outside of the scrolling viewport.
    pub fn present_background(&mut self, ppu: &Ppu) {
        let tilemap = TileMap::by_select_bit(ppu.check_lcdc(LcdControlFlag::BackgroundTileMapSelect));
        let tileset = TileSet::by_select_bit(ppu.check_lcdc(LcdControlFlag::TileDataSelect));
        let palette = DmgPalette::create_default();

        // convert palette based image data into RGBA
        for background_y in 0..255 {
            for background_x in 0..255 {
                let sprite_pixel = ppu.read_tilemap_pixel(
                    tilemap,
                    tileset,
                    background_x,
                    background_y
                );

                let pixel = palette.get_color(&sprite_pixel.value);
                let color = ppu.translate_dmg_color_index(&pixel);

                self.texture_background.set_color(background_x as u32, background_y as u32, color);
            }
        }

        // update texture
        self.texture_background.update_texture()
            .map_err(|e| e.to_string())
            .unwrap()
        ;

        // copy texture into framebuffer
        self.texture_background.copy_to_canvas(&mut self.canvas, self.display_scale).unwrap();

        // present the framebuffer
        self.canvas.present();
    }


    /// Presents the list of objects from the video memory.
    pub fn present_objects(&mut self, ppu: &Ppu) {
        let objects_per_row = 16;
        let objects_rows    = 24;
        let palette         = DmgPalette::create_default();

        for object_y in 0..objects_rows {
            for object_x in 0..objects_per_row {
                let object_index   = object_x + (object_y * objects_per_row);
                let sprite_address = MEMORY_LOCATION_SPRITES_BEGIN + (object_index * 16);

                for object_pixel_y in 0..8 {
                    for object_pixel_x in 0..8 {
                        let sprite_pixel = ppu.read_sprite_pixel_from_address(
                            sprite_address,
                            0,
                            object_pixel_x,
                            object_pixel_y
                        );

                        let pixel = palette.get_color(&sprite_pixel);

                        let pixel_color = ppu.translate_dmg_color_index(&pixel);
                        let texture_x   = (object_x as u32 * 8) + (object_pixel_x as u32);
                        let texture_y   = (object_y as u32 * 8) + (object_pixel_y as u32);

                        self.texture_objects.set_color(texture_x, texture_y, pixel_color);
                    }
                }
            }
        }

        // update texture
        self.texture_objects.update_texture()
            .map_err(|e| e.to_string())
            .unwrap()
        ;

        // copy texture into framebuffer
        self.texture_objects.copy_to_canvas(&mut self.canvas, self.display_scale).unwrap();

        // present the framebuffer
        self.canvas.present();
    }
}
