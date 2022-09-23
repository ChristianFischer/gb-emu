/*
 * Copyright (C) 2022 by Christian Fischer
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
use std::fmt::{Display, Formatter};
use crate::cpu::Interrupt;
use crate::gameboy::clock_t;
use crate::memory::*;
use crate::utils::{change_bit, get_bit};

pub const SCREEN_W: u32 = 160;
pub const SCREEN_H: u32 = 144;

pub const SCREEN_PIXELS: usize = (SCREEN_W * SCREEN_H) as usize;

pub const CPU_CYCLES_PER_LINE:  clock_t =    456;
pub const CPU_CYCLES_PER_FRAME: clock_t = 70_224;

pub const LCD_CONTROL_BIT_BG_WINDOW_ENABLED:        u8 = 0;
pub const LCD_CONTROL_BIT_SPRITE_ENABLED:           u8 = 1;
pub const LCD_CONTROL_BIT_SPRITE_SIZE:              u8 = 2;
pub const LCD_CONTROL_BIT_BG_TILE_MAP_SELECT:       u8 = 3;
pub const LCD_CONTROL_BIT_TILE_DATA_SELECT:         u8 = 4;
pub const LCD_CONTROL_BIT_WINDOW_ENABLED:           u8 = 5;
pub const LCD_CONTROL_BIT_WINDOW_TILE_MAP_SELECT:   u8 = 6;
pub const LCD_CONTROL_BIT_LCD_ENABLED:              u8 = 7;

pub const LCD_STATUS_BIT_PPU_MODE_0:                u8 = 0;
pub const LCD_STATUS_BIT_PPU_MODE_1:                u8 = 1;
pub const LCD_STATUS_BIT_FLAG_COINCIDENCE:          u8 = 2;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_0:         u8 = 3;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_1:         u8 = 4;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_2:         u8 = 5;
pub const LCD_STATUS_BIT_ENABLE_IRQ_LYC_EQ_LY:      u8 = 6;
pub const LCD_STATUS_BIT_UNUSED:                    u8 = 7;


type PixelBuffer160x144 = [u8; SCREEN_PIXELS];

pub struct LcdBuffer {
    pixels: PixelBuffer160x144,
}

#[derive(Copy, Clone)]
pub enum Mode {
    HBlank      = 0,
    VBlank      = 1,
    OamScan     = 2,
    DrawLine    = 3,
}

pub enum FrameState {
    Processing,
    FrameCompleted,
}


/// A list of possible tilesets the gameboy can handle.
#[derive(Copy, Clone)]
pub enum TileSet {
    /// The tileset is based on the 0x8000 address plus tile index as unsigned integer.
    H8000,

    /// The tileset is based on the 0x8800 address plus tile index as signed integer.
    H8800,
}

/// A list of possible tilemaps the gameboy can handle.
#[derive(Copy, Clone)]
pub enum TileMap {
    /// This tilemap is stored in the video memory at 0x9800 - 0x9bff
    H9800,

    /// This tilemap is stored in the video memory at 0x9c00 - 0x9fff
    H9C00,
}


/// Stores the data of a single sprite entry, how
/// it's stored in the OAM memory.
#[derive(Copy, Clone)]
pub struct Sprite {
    /// The sprites position on Y axis.
    pos_y: u8,

    /// The sprites position on X axis.
    pos_x: u8,

    /// The tile number containing the sprites image data to be displayed.
    tile: u8,

    /// Flags to control the sprites behaviour.
    flags: u8,
}


/// Delivers the result of obtaining pixel data from a sprite.
pub struct SpritePixelData {
    /// The color index of the pixel data.
    color_index: u8,

    /// The index of the palette to be used to obtain the pixel color.
    palette_index: u8,
}


/// An object storing data of any scanline to be processed by the PPU.
pub struct ScanlineData {
    /// The line number stored in this object.
    line: u8,

    /// Stores the sprites to be displayed within the current scanline.
    sprites: [Sprite; 10],

    /// The number of sprites found.
    sprites_found: u8,

    /// Stores if the window was enabled for this scanline.
    window_enabled: bool,
}


/// An object representing the gameboy's picture processing unit.
pub struct Ppu {
    clock: clock_t,

    mem: MemoryReadWriteHandle,

    /// The PPU's current mode.
    mode: Mode,

    /// The currently processed scanline.
    ly: u8,

    /// The currently processed pixel in the current scanline.
    current_line_pixel: u8,

    /// The number of cycles being consumed for the current scanline.
    current_line_cycles: clock_t,

    /// The cached data of the currently processed scanline.
    current_scanline: ScanlineData,

    /// Stores the current line being processed for a window.
    /// This in independent of the frame line counter (LY) and just updated
    /// when window pixels were drawn for the current scanline.
    window_line: u8,

    /// The data buffer to store the actual viewport content presented to the display.
    lcd_buffer: LcdBuffer,
}


impl LcdBuffer {
    pub fn alloc() -> LcdBuffer {
        LcdBuffer {
            pixels: [0x00; SCREEN_PIXELS]
        }
    }

    /// Get the width of the buffer image content.
    pub fn get_width(&self) -> u32 {
        SCREEN_W
    }

    /// Get the height of the buffer image content.
    pub fn get_height(&self) -> u32 {
        SCREEN_H
    }

    /// Get the value of a specific pixel.
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let index = x + (y * SCREEN_W);
        self.pixels[index as usize]
    }

    /// Set the value of a specific pixel.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let index = x + (y * SCREEN_W);
        self.pixels[index as usize] = value & 0x03;
    }

    /// Get the pixel data to be displayed.
    pub fn get_pixels(&self) -> &PixelBuffer160x144 {
        &self.pixels
    }
}


impl TileSet {
    /// Selects a TileSet based on the value of a selection bit from the LCD status register.
    pub fn by_select_bit(bit: bool) -> TileSet {
        match bit {
            false => TileSet::H8800,
            true  => TileSet::H8000,
        }
    }

    /// Get the address of a tile when this tileset is used.
    pub fn address_of_tile(&self, tile: u8) -> u16 {
        let tile_u16 = tile as u16;

        match *self {
            TileSet::H8000 => 0x8000 + (tile_u16 << 4),
            TileSet::H8800 => 0x9000 + (tile_u16 << 4) - ((tile_u16 & 0x80) << 5),
        }
    }
}


impl TileMap {
    /// Selects a TileMap based on the value of a selection bit from the LCD status register.
    pub fn by_select_bit(bit: bool) -> TileMap {
        match bit {
            false => TileMap::H9800,
            true  => TileMap::H9C00,
        }
    }

    /// Get the base address where the tilemap is stored.
    pub fn base_address(&self) -> u16 {
        match *self {
            TileMap::H9800 => 0x9800,
            TileMap::H9C00 => 0x9c00,
        }
    }
}


impl Sprite {
    /// Creates an empty sprite with all values zero.
    pub fn empty() -> Sprite {
        Sprite {
            pos_x: 0,
            pos_y: 0,
            tile:  0,
            flags: 0,
        }
    }

    /// Reads sprite data from it's OAM entry.
    pub fn from_oam(mem: &dyn MemoryRead, index: u8) -> Sprite {
        let address = MEMORY_LOCATION_OAM_BEGIN + ((index as u16) * 4);
        Self::from_address(mem, address)
    }

    /// Reads sprite data from any memory address.
    pub fn from_address(mem: &dyn MemoryRead, address: u16) -> Sprite {
        Sprite {
            pos_y: mem.read_u8(address + 0),
            pos_x: mem.read_u8(address + 1),
            tile:  mem.read_u8(address + 2),
            flags: mem.read_u8(address + 3),
        }
    }

    /// Checks whether the sprite is mirrored on X axis.
    pub fn is_flip_x(&self) -> bool {
        get_bit(self.flags, 5)
    }

    /// Checks whether the sprite is mirrored on Y axis.
    pub fn is_flip_y(&self) -> bool {
        get_bit(self.flags, 6)
    }

    /// Get the palette used by this sprite.
    pub fn get_palette(&self) -> u8 {
        if get_bit(self.flags, 4) {
            1
        }
        else {
            0
        }
    }

    /// Checks whether the sprite should always be drawn above background.
    pub fn is_bg_priority(&self) -> bool {
        get_bit(self.flags, 7)
    }
}


impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tile #{} @ {}:{} flipX={} flipY={}",
            self.tile,
            self.pos_x as i32 - 8,
            self.pos_y as i32 - 16,
            self.is_flip_x(),
            self.is_flip_y()
        )
    }
}


impl ScanlineData {
    pub fn new() -> ScanlineData {
        ScanlineData {
            line: 0,
            sprites: [Sprite::empty(); 10],
            sprites_found: 0,
            window_enabled: false,
        }
    }
}


impl Ppu {
    /// Creates a new PPU object.
    pub fn new(mem: MemoryReadWriteHandle) -> Ppu {
        Ppu {
            clock: 0,
            mem,
            mode: Mode::OamScan,
            ly: 0,
            current_line_pixel: 0,
            current_line_cycles: 0,
            current_scanline: ScanlineData::new(),
            window_line: 0,
            lcd_buffer: LcdBuffer::alloc(),
        }
    }


    /// Let the PPU process their data.
    /// This function takes the amount of ticks to be processed
    /// and the return value tells when VBlank finished and
    /// a whole new frame was generated.
    pub fn update(&mut self, cycles: clock_t) -> FrameState {
        self.clock += cycles;

        match self.mode {
            Mode::OamScan  => self.process_oam_scan(),
            Mode::DrawLine => self.process_draw_line(),
            Mode::HBlank   => self.process_hblank(),
            Mode::VBlank   => self.process_vblank(),
        }
    }


    /// Scans the object attribute memory for the current scanline
    /// to collect the objects to be drawn in this line.
    /// Enters Mode::DrawLine after the OAM scan was completed.
    fn process_oam_scan(&mut self) -> FrameState {
        if self.clock > 80 {
            self.clock -= 80;

            self.current_scanline    = self.do_oam_scan_for_line(self.ly);
            self.current_line_pixel  = 0;
            self.current_line_cycles = 80;

            self.enter_mode(Mode::DrawLine);
        }

        FrameState::Processing
    }


    /// Draws pixels of the current scanline into the LCD buffer.
    /// Enters Mode::HBlank after the drawing was completed.
    fn process_draw_line(&mut self) -> FrameState {
        let pixels_remaining = SCREEN_W - (self.current_line_pixel as u32);
        let pixels_to_update = min(self.clock / 2, pixels_remaining as u64);
        if pixels_to_update == 0 {
            return FrameState::Processing;
        }

        // update clock
        let cycles = pixels_to_update * 2;
        self.current_line_cycles += cycles;
        self.clock               -= cycles;

        let lcdc            = self.get_lcdc();
        let bg_enabled      = get_bit(lcdc, LCD_CONTROL_BIT_BG_WINDOW_ENABLED);
        let window_enabled  = get_bit(lcdc, LCD_CONTROL_BIT_WINDOW_ENABLED);
        let sprites_enabled = get_bit(lcdc, LCD_CONTROL_BIT_SPRITE_ENABLED);
        let tileset_select  = get_bit(lcdc, LCD_CONTROL_BIT_TILE_DATA_SELECT);
        let tileset         = TileSet::by_select_bit(tileset_select);
        let palette_bg      = self.mem.read_u8(MEMORY_LOCATION_PALETTE_BG);
        let palette_obp0    = self.mem.read_u8(MEMORY_LOCATION_PALETTE_OBP0);
        let palette_obp1    = self.mem.read_u8(MEMORY_LOCATION_PALETTE_OBP1);
        let wx              = self.get_window_x();
        let wy              = self.get_window_y();

        for _ in 0..pixels_to_update {
            let pixel_background = {
                // check if the flag for window/background is enabled
                if bg_enabled {
                    // check if the window is enabled and the current screen pixel is inside the area covered by wx/wy
                    if !self.current_scanline.window_enabled && window_enabled {
                        if (self.current_line_pixel+7 >= wx) && ((wy as u32) < SCREEN_H) && (wy <= self.ly) {
                            self.current_scanline.window_enabled = true;
                        }
                    }

                    // process window pixels instead of background, if the window was enabled for this scanline
                    if self.current_scanline.window_enabled {
                        let window_tilemap_select = get_bit(lcdc, LCD_CONTROL_BIT_WINDOW_TILE_MAP_SELECT);
                        let window_tilemap        = TileMap::by_select_bit(window_tilemap_select);
                        let position_in_window_x  = self.current_line_pixel+7 - wx;
                        let position_in_window_y  = self.window_line;

                        self.read_tilemap_pixel(
                            window_tilemap,
                            tileset,
                            position_in_window_x,
                            position_in_window_y
                        )
                    }
                    else {
                        // otherwise just handle the normal background

                        let bg_tilemap_select = get_bit(lcdc, LCD_CONTROL_BIT_BG_TILE_MAP_SELECT);
                        let bg_tilemap        = TileMap::by_select_bit(bg_tilemap_select);

                        let (background_x, background_y) = self.screen_to_background(
                            self.current_line_pixel,
                            self.ly
                        );

                        self.read_tilemap_pixel(
                            bg_tilemap,
                            tileset,
                            background_x,
                            background_y
                        )
                    }
                }
                else {
                    0
                }
            };

            // get the foreground pixel by reading the color of any sprite on the current
            // position within this scanline
            let sprite_data = if sprites_enabled {
                self.read_scanline_sprite_pixel(
                    &self.current_scanline,
                    self.current_line_pixel,
                    pixel_background
                )
            }
            else {
                None
            };

            let (pixel, palette) = if let Some(sprite_pixel_data) = sprite_data {
                let sprite_palette = if sprite_pixel_data.palette_index == 0 {
                    palette_obp0
                }
                else {
                    palette_obp1
                };

                (sprite_pixel_data.color_index, sprite_palette)
            }
            else {
                (pixel_background, palette_bg)
            };

            let pixel_color = (palette >> (pixel << 1)) & 0x03;

            // write pixel into LCD buffer
            self.lcd_buffer.set_pixel(
                self.current_line_pixel as u32,
                self.ly as u32,
                pixel_color
            );

            // set next pixel to compute
            self.current_line_pixel += 1;
        }

        // when reached the end of the current scanline, enter HBlank mode
        if self.current_line_pixel as u32 >= SCREEN_W {
            self.enter_mode(Mode::HBlank);
        }

        FrameState::Processing
    }


    /// Process the HBlank mode after each drawn scanline.
    /// Enters Mode::OamScan for the next line or
    /// Mode::VBlank if the current line was the last one.
    fn process_hblank(&mut self) -> FrameState {
        let remaining_cycles = CPU_CYCLES_PER_LINE - self.current_line_cycles;

        if self.clock >= remaining_cycles {
            self.clock -= remaining_cycles;

            return self.next_ly();
        }

        FrameState::Processing
    }


    /// Process the VBlank mode after all scanlines were drawn.
    /// Enters Mode::OamScan for the first scanline of the next frame,
    /// afters the VBlank was completed.
    fn process_vblank(&mut self) -> FrameState {
        if self.clock >= CPU_CYCLES_PER_LINE {
            self.clock -= CPU_CYCLES_PER_LINE;

            return self.next_ly();
        }

        FrameState::Processing
    }


    /// Switches into a given PPU mode.
    /// Updates the LCD status byte with the current mode.
    fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;

        let mut lcd_stat = self.get_lcd_stat();
        lcd_stat = lcd_stat & 0b_1111_1100;
        lcd_stat = lcd_stat | (self.mode as u8);

        self.mem.write_u8(MEMORY_LOCATION_LCD_STATUS, lcd_stat);

        // request interrupt when entering VBlank
        match mode {
            Mode::HBlank => {
                if get_bit(lcd_stat, LCD_STATUS_BIT_ENABLE_IRQ_MODE_0) {
                    self.mem.request_interrupt(Interrupt::LcdStat);
                }
            }

            Mode::VBlank => {
                if get_bit(lcd_stat, LCD_STATUS_BIT_ENABLE_IRQ_MODE_1) {
                    self.mem.request_interrupt(Interrupt::LcdStat);
                }

                self.mem.request_interrupt(Interrupt::VBlank);
            },

            Mode::OamScan => {
                if get_bit(lcd_stat, LCD_STATUS_BIT_ENABLE_IRQ_MODE_2) {
                    self.mem.request_interrupt(Interrupt::LcdStat);
                }
            }

            _ => { }
        }
    }


    /// Switches to the next scanline.
    /// Checks for coincidence with the LYC byte and updates the
    /// LCD status byte as well as the current LY byte in memory.
    /// Enters either Mode::OamScan or Mode::VBlank depending on
    /// the next scanline.
    fn next_ly(&mut self) -> FrameState {
        if self.ly == 153 {
            self.ly = 0;
        }
        else {
            self.ly = self.ly + 1;
        }

        // also progress window line counter,
        // if the window was drawn in this line
        if self.current_scanline.window_enabled {
            self.window_line += 1;
        }

        // update ly value in memory
        self.mem.write_u8(MEMORY_LOCATION_LY, self.ly);

        // check for ly == lyc coincidence
        {
            let lyc = self.mem.read_u8(MEMORY_LOCATION_LYC);
            let coincidence = self.ly == lyc;
            let mut lcd_stat = self.get_lcd_stat();
            lcd_stat = change_bit(lcd_stat, LCD_STATUS_BIT_FLAG_COINCIDENCE, coincidence);
            self.mem.write_u8(MEMORY_LOCATION_LCD_STATUS, lcd_stat);

            // fire interrupt, if enabled
            if coincidence {
                if get_bit(lcd_stat, LCD_STATUS_BIT_ENABLE_IRQ_LYC_EQ_LY) {
                    self.mem.request_interrupt(Interrupt::LcdStat);
                }
            }
        }

        // enter vblank when beyond the last scanline
        // enter OAM scan for next scanline otherwise
        match self.ly {
              0..=143 => self.enter_mode(Mode::OamScan),
            144       => self.enter_mode(Mode::VBlank),
            145..=153 => { /* remains in VBlank */ },
            _         => unreachable!()
        }

        // notify FrameCompleted after switching back to line #0
        if self.ly == 0 {
            self.on_new_frame();

            FrameState::FrameCompleted
        }
        else {
            FrameState::Processing
        }
    }


    /// Callback to reset data when starting a new frame
    fn on_new_frame(&mut self) {
        self.window_line = 0;
    }


    /// Get the LCD buffer which contains the actual data sent to the device's display.
    pub fn get_lcd(&self) -> &LcdBuffer {
        &self.lcd_buffer
    }

    /// Get the value of the LCD Control register
    pub fn get_lcdc(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_LCD_CONTROL)
    }

    /// Get the value of the LCD Status register
    pub fn get_lcd_stat(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_LCD_STATUS)
    }

    /// Get the display viewport offset on X axis.
    pub fn get_scroll_x(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_SCX)
    }

    /// Get the display viewport offset on Y axis.
    pub fn get_scroll_y(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_SCY)
    }

    /// Get the window position on X axis.
    pub fn get_window_x(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_WX)
    }

    /// Get the window position on Y axis.
    pub fn get_window_y(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_WY)
    }

    /// Compute the background location of any screen pixel.
    pub fn screen_to_background(&self, screen_x: u8, screen_y: u8) -> (u8, u8) {
        let background_x = ((screen_x as u32 + self.get_scroll_x() as u32) & 0xff) as u8;
        let background_y = ((screen_y as u32 + self.get_scroll_y() as u32) & 0xff) as u8;
        (background_x, background_y)
    }

    /// Performs an OAM scan and stores it's result in the 'scanline' object.
    pub fn do_oam_scan_for_line(&self, line_number: u8) -> ScanlineData {
        let mut scanline = ScanlineData::new();
        scanline.line = line_number;

        let lcdc        = self.get_lcdc();
        let big_sprites = get_bit(lcdc, LCD_CONTROL_BIT_SPRITE_SIZE);
        let sprite_h    = if big_sprites { 16 } else { 8 };

        // sprite position 0 is not on scanline 0, but 16 pixel above the screen to
        // allow sprites being partially outside the screen.
        // Adjust the value here to avoid doing it for each check.
        let ly_plus_16 = line_number + 16;

        // iterate through all OAM entries
        for oam_entry in 0..40 {
            let sprite = Sprite::from_oam(&self.mem, oam_entry);

            // take a sprite if x > 0 and intersects the current scanline
            if
                    sprite.pos_x > 0
                &&  ly_plus_16 >= sprite.pos_y
                &&  ly_plus_16 < (sprite.pos_y + sprite_h)
            {
                scanline.sprites[scanline.sprites_found as usize] = sprite;
                scanline.sprites_found += 1;

                if scanline.sprites_found >= 10 {
                    break;
                }
            }
        }

        // the ppu prioritizes sprites with lower x position over higher x position
        // independent of their order in the OAM list, so we sort all found sprites
        // by their x position
        scanline.sprites[0 .. scanline.sprites_found as usize].sort_by(
            |a, b| {
                let ax = a.pos_x;
                let bx = b.pos_x;
                ax.cmp(&bx)
            }
        );

        scanline
    }

    /// Reads a pixel from the current scanline sprite data on a given x position.
    pub fn read_scanline_sprite_pixel(&self, scanline: &ScanlineData, x: u8, pixel_background: u8) -> Option<SpritePixelData> {
        // screen position considering the border offset of -8 / -16
        let screen_x = x + 8;
        let screen_y = scanline.line + 16;

        let lcdc        = self.get_lcdc();
        let big_sprites = get_bit(lcdc, LCD_CONTROL_BIT_SPRITE_SIZE);
        let sprite_h    = if big_sprites { 16 } else { 8 };
        let sprite_w    = 8;

        // when big sprites are enabled, the top sprite always has the least significant bit
        // set to 0, and the bottom sprite is using the same sprite number with the LSB set to 1
        // so we're just eliminating the LSB and continue reading with sprite data behind the
        // top sprite memory location.
        let sprite_mask = if big_sprites { 0xfe } else { 0xff };

        // iterate over all sprite previously found by the OAM scan
        for sprite_index in 0..scanline.sprites_found {
            let sprite = &(scanline.sprites[sprite_index as usize]);

            // check if the sprite overlaps the current scanline pixel
            if screen_x < sprite.pos_x || x >= sprite.pos_x {
                continue;
            }

            // calculate the position inside the sprite including x and y flip
            let mut sprite_pixel_x = screen_x - sprite.pos_x;
            let mut sprite_pixel_y = screen_y - sprite.pos_y;

            if sprite.is_flip_x() {
                sprite_pixel_x = sprite_w - sprite_pixel_x - 1;
            }

            if sprite.is_flip_y() {
                sprite_pixel_y = sprite_h - sprite_pixel_y - 1;
            }

            // read the sprite pixel value
            let pixel = self.read_sprite_pixel(
                TileSet::H8000,
                sprite.tile & sprite_mask,
                sprite_pixel_x,
                sprite_pixel_y
            );

            // color index 0 is transparent; with bg priority, the sprite is behind the BG
            if pixel == 0 || (sprite.is_bg_priority() && pixel_background != 0) {
                continue;
            }

            return Some(SpritePixelData {
                color_index: pixel,
                palette_index: sprite.get_palette()
            });
        }

        None
    }

    /// Read the pixel value of the background on a given position.
    pub fn read_tilemap_pixel(&self, tilemap: TileMap, tileset: TileSet, tilemap_x: u8, tilemap_y: u8) -> u8 {
        let tile_x       = (tilemap_x / 8) as u16;
        let tile_y       = (tilemap_y / 8) as u16;
        let tile_pixel_x = (tilemap_x % 8) as u8;
        let tile_pixel_y = (tilemap_y % 8) as u8;
        let tile_index   = tile_y * 32 + tile_x;
        let tile_address = tilemap.base_address() + tile_index;
        let sprite       = self.mem.read_u8(tile_address as u16);

        self.read_sprite_pixel(
            tileset,
            sprite,
            tile_pixel_x,
            tile_pixel_y
        )
    }

    /// Read the pixel value of a sprite.
    pub fn read_sprite_pixel(&self, tileset: TileSet, sprite: u8, x: u8, y: u8) -> u8 {
        let sprite_address      = tileset.address_of_tile(sprite);
        self.read_sprite_pixel_from_address(sprite_address, x, y)
    }

    /// Read the pixel value of a sprite.
    pub fn read_sprite_pixel_from_address(&self, sprite_address: u16 , x: u8, y: u8) -> u8 {
        let sprite_line_address = sprite_address + y as u16 * 2;
        let pixel_mask            = 1u8 << (7 - x);
        let byte0                 = self.mem.read_u8(sprite_line_address + 0);
        let byte1                 = self.mem.read_u8(sprite_line_address + 1);

        let pixel =
                (if (byte0 & pixel_mask) != 0 { 0x01 } else { 0x00 })
            |   (if (byte1 & pixel_mask) != 0 { 0x02 } else { 0x00 })
        ;

        pixel
    }
}
