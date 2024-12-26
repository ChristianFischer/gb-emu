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

use core::cmp::min;
use core::mem::take;

use crate::cpu::interrupts::Interrupt;
use crate::debug::DebugEvent;
use crate::device_type::DeviceType;
use crate::gameboy::{Clock, DeviceConfig, EmulationType};
use crate::mmu::locations::*;
use crate::mmu::memory_bus::{memory_map, MemoryBusConnection, MemoryBusSignals};
use crate::mmu::memory_data::mapped::MemoryDataMapped;
use crate::mmu::memory_data::MemoryData;
use crate::ppu::flags::{LcdControl, LcdControlFlag, LcdInterruptFlag, LcdInterruptFlags};
use crate::ppu::graphic_data::*;
use crate::ppu::sprite_image::SpriteImage;
use crate::ppu::video_memory::{OamRam, OamRamBank, Palettes, VideoMemory};
use crate::utils::{get_bit, SerializableArray};


pub const SCREEN_W: u32 = 160;
pub const SCREEN_H: u32 = 144;

pub const SCREEN_PIXELS: usize = (SCREEN_W * SCREEN_H) as usize;

pub const CPU_CYCLES_PER_LINE:                  Clock =    456;
pub const CPU_CYCLES_PER_FRAME:                 Clock = 70_224;
pub const CPU_CYCLES_OAMSCAN:                   Clock =     80;
pub const CPU_CYCLES_DRAW_MIN:                  Clock =    172;
pub const CPU_CYCLES_DRAW_MAX:                  Clock =    289;
pub const CPU_CYCLES_HBLANK_MIN:                Clock =     87;
pub const CPU_CYCLES_HBLANK_MAX:                Clock =    204;

pub const TILE_ATTR_BIT_VRAM_BANK:                  u8 = 3;
pub const TILE_ATTR_BIT_H_FLIP:                     u8 = 5;
pub const TILE_ATTR_BIT_V_FLIP:                     u8 = 6;
pub const TILE_ATTR_BIT_BG_TO_OAM_PRIO:             u8 = 7;


type PixelArray160x144  = SerializableArray<Color, SCREEN_PIXELS>;
type PixelBuffer160x144 = MemoryDataMapped<PixelArray160x144>;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LcdBuffer {
    pixels: PixelBuffer160x144,
}


#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    HBlank      = 0,
    VBlank      = 1,
    OamScan     = 2,
    DrawLine    = 3,
}


/// Defines the state whether the PPU is enabled or not.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LcdState {
    /// The PPU is disabled and does not process anything.
    Off,

    /// The PPU is enabled and processing graphic data.
    On,

    /// After the PPU was enabled, the first frame behaves differently.
    EnableFirstFrame,
}


/// An object storing data of any scanline to be processed by the PPU.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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


/// The result of fetching a pixel from either background / window
/// or OAM list.
pub struct PixelFetchResult {
    /// The color value from the tile being displayed.
    /// This value is not a final color but needs to be
    /// translated via color palette.
    pub value: SpritePixelValue,

    /// DMG: For sprites either 0 or 1 to switch between OBP0 and OBP1.
    pub palette_dmg: u8,

    /// GBC: The index of the palette being used (0-7)
    pub palette_gbc: u8,

    /// DMG: Unused.
    /// CGB: The OAM index of the sprite being displayed.
    pub sprite_priority: u8,

    /// OBJ to BG priority bit
    pub background_priority: bool,
}


/// Struct to temporarily store references to pixel data and their palettes
/// to be used to resolve the actual color for a pixel.
pub struct PixelFetchResultWithPalette<'a> {
    pub data:        &'a PixelFetchResult,
    pub palette_dmg: &'a DmgPalette,
    pub palette_gbc: &'a GbcPaletteData,
}


/// Contains information where to read tile data.
/// This information can be obtained by reading tile
/// information from the PPU tile map.
pub struct TileFetchProperties {
    /// The TileMap where to read from
    pub tilemap: TileMap,

    /// The TileSet where to take the images from, which are referenced by the TileSet.
    pub tileset: TileSet,

    /// The tile index to be read from the tile map.
    pub tile_index: u16,

    /// The pixel on the X axis to read from the tile image.
    /// This may get flipped depending on tile properties.
    pub tile_pixel_x: u8,

    /// The pixel on the Y axis to read from the tile image.
    /// This may get flipped depending on tile properties.
    pub tile_pixel_y: u8,
}


#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct PpuRegisters {
    /// LCD control flags.
    lcd_control: LcdControl,

    /// Part of the LCD Status register, which stores the flags
    /// when the LCD Stat Interrupt may be fired
    lcd_interrupts: LcdInterruptFlags,

    /// The comparison value to be compared with the current line.
    /// If LY == LYC and the according interrupt flag in LCD STAT is enabled,
    /// the LCD Stat interrupt will be fired.
    line_compare: u8,

    /// Background scroll value on X axis.
    scroll_x: u8,

    /// Background scroll value on Y axis.
    scroll_y: u8,

    /// Window x position.
    window_x: u8,

    /// Window y position.
    window_y: u8,

    /// Object priority flag
    object_priority: bool,
}


/// An object representing the gameboy's picture processing unit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ppu {
    clock: Clock,

    /// Current device config
    device_config: DeviceConfig,

    /// Pending output to be sent back through the memory bus.
    signals: MemoryBusSignals,

    /// Whether PPU is enabled or not.
    lcd_state: LcdState,

    /// `true` for the first frame after the PPU was enabled.
    is_first_frame: bool,

    /// The PPU's current mode.
    mode: Mode,

    /// Several memory units connected to the PPU.
    memory: VideoMemory,

    /// Current values of registers used by the PPU.
    registers: PpuRegisters,

    /// The currently processed scanline.
    current_line: u8,

    /// The currently processed pixel in the current scanline.
    current_line_pixel: u8,

    /// The number of cycles being consumed for the current scanline.
    current_line_cycles: Clock,

    /// The cached data of the currently processed scanline.
    current_scanline: ScanlineData,

    /// Stores the current line being processed for a window.
    /// This in independent of the frame line counter (LY) and just updated
    /// when window pixels were drawn for the current scanline.
    window_line: u8,

    /// If in DMG mode, a set of RGB colors to translate the LCD intensity values
    /// into RGB colors to be displayed on color screens.
    dmg_display_palette: DmgDisplayPalette,

    /// The data buffer to store the actual viewport content presented to the display.
    lcd_buffer: LcdBuffer,
}


impl PixelFetchResult {
    /// Creates an empty PixelFetchResult containing '0' as the pixel value,
    /// which has the lowest priority.
    pub fn none() -> Self {
        Self {
            value: SpritePixelValue::none(),
            palette_dmg: 0,
            palette_gbc: 0,
            sprite_priority: 0,
            background_priority: false,
        }
    }
}


impl LcdBuffer {
    pub fn alloc() -> LcdBuffer {
        LcdBuffer::allow_with_color(Color::white())
    }

    pub fn allow_with_color(color: Color) -> LcdBuffer {
        LcdBuffer {
            pixels: PixelBuffer160x144::new([color; SCREEN_PIXELS])
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
    pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
        let index = x + (y * SCREEN_W);
        &self.pixels.get()[index as usize]
    }

    /// Set the value of a specific pixel.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = x + (y * SCREEN_W);
        self.pixels.get_mut()[index as usize] = color;
    }

    /// Fill the whole screen with a single solid color.
    pub fn fill(&mut self, color: Color) {
        for pixel in self.pixels.get_mut() {
            *pixel = color;
        }
    }

    /// Get the pixel data to be displayed.
    pub fn get_pixels(&self) -> &PixelBuffer160x144 {
        &self.pixels
    }

    /// Get the pixel data to be displayed as a slice of bytes.
    pub fn get_pixels_as_slice(&self) -> &[u8] {
        self.pixels.as_slice()
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
    pub fn new(device_config: DeviceConfig) -> Ppu {
        let dmg_display_palette = match device_config.device {
            DeviceType::GameBoyDmg => DmgDisplayPalette::new_green(),
            _ => DmgDisplayPalette::new_gray(),
        };

        let blank_color = Self::get_blank_color(&device_config, &dmg_display_palette);
        
        Ppu {
            clock: 0,
            device_config,
            signals: MemoryBusSignals::default(),
            lcd_state: LcdState::On,
            is_first_frame: true,
            mode: Mode::OamScan,
            memory: VideoMemory::new(device_config),
            registers: PpuRegisters::default(),
            current_line: 0,
            current_line_pixel: 0,
            current_line_cycles: 0,
            current_scanline: ScanlineData::new(),
            window_line: 0,
            dmg_display_palette,
            lcd_buffer: LcdBuffer::allow_with_color(blank_color),
        }
    }
    
    
    /// Get the blank color for a disabled screen.
    fn get_blank_color(device_config: &DeviceConfig, dmg_display_palette: &DmgDisplayPalette) -> Color {
        match device_config.emulation {
            EmulationType::DMG => dmg_display_palette.get_colors()[0],
            EmulationType::GBC => Color::white(),
        }
    }


    /// Let the PPU process their data.
    /// This function takes the amount of ticks to be processed
    /// and the return value tells when VBlank finished and
    /// a whole new frame was generated.
    pub fn update(&mut self, cycles: Clock) {
        match self.lcd_state {
            LcdState::On => {
                self.clock += cycles;

                match self.mode {
                    Mode::OamScan  => self.process_oam_scan(),
                    Mode::DrawLine => self.process_draw_line(),
                    Mode::HBlank   => self.process_hblank(),
                    Mode::VBlank   => self.process_vblank(),
                }
            }

            LcdState::EnableFirstFrame => {
                self.clock += cycles;

                match self.mode {
                    // 1st frame starts with HBlank for 80 cycles
                    Mode::HBlank => {
                        if self.clock >= CPU_CYCLES_OAMSCAN {
                            self.clock -= CPU_CYCLES_OAMSCAN;
                            self.current_line_cycles += CPU_CYCLES_OAMSCAN;
                            self.enter_mode(Mode::DrawLine);
                        }
                    }

                    // after hblank we switch into drawing, but since first frame
                    // will be white, we just wait until the frame time elapsed
                    Mode::DrawLine => {
                        if self.clock >= CPU_CYCLES_DRAW_MIN {
                            self.clock -= CPU_CYCLES_DRAW_MIN;
                            self.current_line_cycles += CPU_CYCLES_DRAW_MIN;
                            self.current_line_cycles += 4;
                            self.enter_mode(Mode::HBlank);

                            self.lcd_state = LcdState::On;
                        }
                    }

                    _ => { unreachable!(); }
                }
            }

            LcdState::Off => { }
        }
    }


    /// Scans the object attribute memory for the current scanline
    /// to collect the objects to be drawn in this line.
    /// Enters Mode::DrawLine after the OAM scan was completed.
    fn process_oam_scan(&mut self) {
        if self.clock >= CPU_CYCLES_OAMSCAN {
            self.clock -= CPU_CYCLES_OAMSCAN;

            self.current_scanline    = self.do_oam_scan_for_line(self.current_line);
            self.current_line_pixel  = 0;
            self.current_line_cycles = 80;

            self.enter_mode(Mode::DrawLine);
        }
    }


    /// Draws pixels of the current scanline into the LCD buffer.
    /// Enters Mode::HBlank after the drawing was completed.
    fn process_draw_line(&mut self) {
        let pixels_remaining = SCREEN_W - (self.current_line_pixel as u32);

        if pixels_remaining > 0 {
            // how many pixels could be drawn
            let pixels_to_update = min(self.clock, pixels_remaining as u64);

            // update clock
            let cycles = pixels_to_update;
            self.current_line_cycles += cycles;
            self.clock -= cycles;

            // process the pixels
            self.process_draw_line_pixels(pixels_to_update);
        }
        else {
            self.current_line_cycles += self.clock;
            self.clock = 0;

            // wait for the minimum time mode 2+3 could take
            if self.current_line_cycles >= CPU_CYCLES_OAMSCAN + CPU_CYCLES_DRAW_MIN {
                self.enter_mode(Mode::HBlank);
            }
        }
    }


    /// Process a number of pixels within the current scanline.
    fn process_draw_line_pixels(&mut self, pixels_to_update: Clock) {
        let window_enabled   = self.check_lcdc(LcdControlFlag::WindowEnabled);
        let palette_bg       = &self.memory.palettes.bgp;
        let palette_obp      = &self.memory.palettes.obp;
        let palettes_gbc_bg  = &self.memory.palettes.gbc_background_palette.get();
        let palettes_gbc_obj = &self.memory.palettes.gbc_object_palette.get();
        let wx               = self.registers.window_x;
        let wy               = self.registers.window_y;

        for _ in 0..pixels_to_update {
            // check if the window is enabled and the current screen pixel is inside the area covered by wx/wy
            if !self.current_scanline.window_enabled && window_enabled {
                if (self.current_line_pixel+7 >= wx) && ((wy as u32) < SCREEN_H) && (wy <= self.current_line) {
                    self.current_scanline.window_enabled = true;
                }
            }

            // fetch background and foreground pixels, if any
            let fetched_pixel_background = self.fetch_background_pixel();
            let fetched_pixel_foreground = self.fetch_foreground_pixel();

            // select palettes for background pixel
            let pixel_background = PixelFetchResultWithPalette {
                data: &fetched_pixel_background,
                palette_dmg: &palette_bg,
                palette_gbc: &palettes_gbc_bg[fetched_pixel_background.palette_gbc as usize]
            };

            // select palettes for foreground pixel
            let pixel_foreground = PixelFetchResultWithPalette {
                data: &fetched_pixel_foreground,
                palette_dmg: &palette_obp[fetched_pixel_foreground.palette_dmg as usize],
                palette_gbc: &palettes_gbc_obj[fetched_pixel_foreground.palette_gbc as usize]
            };

            // select pixel to be displayed
            let pixel = self.mix_pixels(
                    &pixel_background,
                    &pixel_foreground
            );

            // the first frame does not draw pixels
            if !self.is_first_frame {
                // resolve pixel color using the according palette
                let pixel_color = match self.device_config.emulation {
                    EmulationType::DMG => {
                        let lcd_pixel = pixel.palette_dmg.get_color(&pixel.data.value);
                        *self.translate_dmg_color_index(&lcd_pixel)
                    }

                    EmulationType::GBC => {
                        pixel.palette_gbc.get_color(&pixel.data.value)
                    }
                };

                // write pixel into LCD buffer
                self.lcd_buffer.set_pixel(
                    self.current_line_pixel as u32,
                    self.current_line as u32,
                    pixel_color
                );
            }

            // set next pixel to compute
            self.current_line_pixel += 1;
        }
    }


    /// Fetch the data of the background or window layer on the current position in the active scanline.
    fn fetch_background_pixel(&self) -> PixelFetchResult {
        let bg_enabled      = self.check_lcdc(LcdControlFlag::BackgroundAndWindowEnabled);
        let tileset_select  = self.check_lcdc(LcdControlFlag::TileDataSelect);
        let tileset         = TileSet::by_select_bit(tileset_select);

        // check if the flag for window/background is enabled
        // on CGB the background is always active, but their priority
        // disabled by clearing the LCDC bit 0
        if bg_enabled || self.device_config.is_gbc_enabled() {
            // process window pixels instead of background, if the window was enabled for this scanline
            let tile_info = if self.current_scanline.window_enabled {
                let window_tilemap_select = self.check_lcdc(LcdControlFlag::WindowTileMapSelect);
                let window_tilemap        = TileMap::by_select_bit(window_tilemap_select);
                let position_in_window_x  = self.current_line_pixel+7 - self.registers.window_x;
                let position_in_window_y  = self.window_line;

                self.read_tilemap_properties(
                    window_tilemap,
                    tileset,
                    position_in_window_x,
                    position_in_window_y
                )
            }
            else {
                // otherwise just handle the normal background

                let bg_tilemap_select = self.check_lcdc(LcdControlFlag::BackgroundTileMapSelect);
                let bg_tilemap        = TileMap::by_select_bit(bg_tilemap_select);

                let (background_x, background_y) = self.screen_to_background(
                    self.current_line_pixel,
                    self.current_line
                );

                self.read_tilemap_properties(
                    bg_tilemap,
                    tileset,
                    background_x,
                    background_y
                )
            };

            self.read_tile_pixel(&tile_info)
        }
        else {
            PixelFetchResult::none()
        }
    }


    /// Fetch the foreground pixel by reading the color of any sprite on the current
    /// position within the active scanline
    pub fn fetch_foreground_pixel(&self) -> PixelFetchResult {
        let sprites_enabled = self.check_lcdc(LcdControlFlag::SpritesEnabled);

        if sprites_enabled {
            self.read_scanline_sprite_pixel(
                &self.current_scanline,
                self.current_line_pixel
            )
        }
        else {
            PixelFetchResult::none()
        }
    }


    /// Selects whether to display background or foreground pixel depending on current priority bits
    pub fn mix_pixels<'a>(
            &self,
            background: &'a PixelFetchResultWithPalette<'a>,
            foreground: &'a PixelFetchResultWithPalette<'a>
    ) -> &'a PixelFetchResultWithPalette<'a>
    {
        // on GBC the meaning of the BG enabled bit is changed into master priority, which
        // disables the background priority flags, instead of disabling the whole background
        let is_master_priority =
                self.device_config.is_gbc_enabled()
            &&  !self.check_lcdc(LcdControlFlag::BackgroundAndWindowEnabled)
        ;

        // check for background priority, if not in master priority mode
        if !is_master_priority {
            if background.data.background_priority && background.data.value.is_opaque() {
                return background;
            }

            if foreground.data.background_priority && background.data.value.is_opaque() {
                return background;
            }
        }

        // take object pixel, if not transparent
        if foreground.data.value.is_opaque() {
            return foreground;
        }

        // take background pixel otherwise
        background
    }


    /// Process the HBlank mode after each drawn scanline.
    /// Enters Mode::OamScan for the next line or
    /// Mode::VBlank if the current line was the last one.
    fn process_hblank(&mut self) {
        let remaining_cycles = CPU_CYCLES_PER_LINE - self.current_line_cycles;

        if self.clock >= remaining_cycles {
            self.clock -= remaining_cycles;

            self.enter_next_line();
        }
    }


    /// Process the VBlank mode after all scanlines were drawn.
    /// Enters Mode::OamScan for the first scanline of the next frame,
    /// afters the VBlank was completed.
    fn process_vblank(&mut self) {
        if self.clock >= CPU_CYCLES_PER_LINE {
            self.clock -= CPU_CYCLES_PER_LINE;

            self.enter_next_line();
        }
    }


    /// Switches into a given PPU mode.
    /// Updates the LCD status byte with the current mode.
    fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;

        // request interrupt when entering VBlank
        match mode {
            Mode::HBlank => {
                if self.is_interrupt_enabled(LcdInterruptFlag::InterruptByHBlank) {
                    self.request_interrupt(Interrupt::LcdStat);
                }
            }

            Mode::VBlank => {
                if self.is_interrupt_enabled(LcdInterruptFlag::InterruptByVBlank) {
                    self.request_interrupt(Interrupt::LcdStat);
                }

                // entering line 144, where VBlank begins will also trigger
                // the interrupt for entering mode 2/oam
                if self.is_interrupt_enabled(LcdInterruptFlag::InterruptByOam) {
                    self.request_interrupt(Interrupt::LcdStat);
                }

                self.request_interrupt(Interrupt::VBlank);

                // frame completed rendering; this notifies frontends to
                // display the new frame image data
                self.signals.events |= DebugEvent::PpuFrameCompleted;
            },

            Mode::OamScan => {
                if self.is_interrupt_enabled(LcdInterruptFlag::InterruptByOam) {
                    self.request_interrupt(Interrupt::LcdStat);
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
    fn enter_next_line(&mut self) {
        if self.current_line == 153 {
            self.current_line = 0;
        }
        else {
            self.current_line += 1;
        }

        // also progress window line counter,
        // if the window was drawn in this line
        if self.current_scanline.window_enabled {
            self.window_line += 1;
        }

        // reset the current line pixel
        self.current_line_pixel = 0;

        // check for ly == lyc coincidence
        self.test_coincidence();

        // enter vblank when beyond the last scanline
        // enter OAM scan for next scanline otherwise
        match self.current_line {
              0..=143 => self.enter_mode(Mode::OamScan),
            144       => self.enter_mode(Mode::VBlank),
            145..=153 => { /* remains in VBlank */ },
            _         => unreachable!()
        }

        // notify LineCompleted after switching a line
        self.signals.events |= DebugEvent::PpuLineCompleted;

        // begin a new frame after switching back to line #0
        if self.current_line == 0 {
            self.on_new_frame();
        }
    }


    /// Tests whether LY matches LYC and raises the according interrupt, if so.
    fn test_coincidence(&mut self) {
        if self.is_interrupt_enabled(LcdInterruptFlag::InterruptByCoincidence) {
            let coincidence = self.current_line == self.registers.line_compare;

            // fire interrupt on match
            if coincidence {
                self.request_interrupt(Interrupt::LcdStat);
            }
        }
    }


    /// Callback to reset data when starting a new frame
    fn on_new_frame(&mut self) {
        self.window_line    = 0;
        self.is_first_frame = false;
    }


    /// Clears the screen with a 'blank' color.
    fn clear_screen(&mut self) {
        self.lcd_buffer.fill(Self::get_blank_color(
            &self.device_config,
            &self.dmg_display_palette
        ));
    }


    /// Reset the PPU once it get disabled.
    fn on_ppu_reset(&mut self) {
        self.clock                  = 0;
        self.lcd_state              = LcdState::Off;
        self.mode                   = Mode::HBlank;
        self.current_line           = 0;
        self.current_line_cycles    = 0;
        self.current_line_pixel     = 0;
        self.registers.line_compare = 0;
        self.clear_screen();
    }


    /// The PPU just got enabled from the disabled state.
    fn on_ppu_enabled(&mut self) {
        self.lcd_state      = LcdState::EnableFirstFrame;
        self.is_first_frame = true;

        self.enter_mode(Mode::HBlank);
    }


    /// Requests an interrupt to be fired.
    fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.signals.interrupts |= interrupt;
    }

    /// Set the palette to be used to translate DMG LCD color values into RGBA colors.
    pub fn set_dmg_display_palette(&mut self, palette: DmgDisplayPalette) {
        self.dmg_display_palette = palette;
        
        // clear the screen when the palette was changed.
        if self.is_first_frame {
            self.clear_screen();
        }
    }

    /// Get the current palette to be used to translate DMG LCD color intensities into RGBA colors
    pub fn get_dmg_display_palette(&self) -> &DmgDisplayPalette {
        &self.dmg_display_palette
    }

    /// Get the RGBA color for any color color index.
    pub fn translate_dmg_color_index(&self, pixel: &DmgLcdPixel) -> &Color {
        self.get_dmg_display_palette().get_color(pixel)
    }

    /// Get the index of the line currently being drawn.
    pub fn get_current_line(&self) -> u8 {
        self.current_line
    }

    /// Get the index of the pixel currently being drawn.
    pub fn get_current_line_pixel(&self) -> u8 {
        self.current_line_pixel
    }

    /// Get the LCD buffer which contains the actual data sent to the device's display.
    pub fn get_lcd(&self) -> &LcdBuffer {
        &self.lcd_buffer
    }

    /// Get the OAM table.
    pub fn get_oam(&self) -> &OamRam {
        self.memory.oam.get()
    }

    /// Get the OAM table.
    pub fn get_oam_mut(&mut self) -> &mut OamRam {
        self.memory.oam.get_mut()
    }

    /// Get the OAM table.
    pub fn get_oam_bank_mut(&mut self) -> &mut OamRamBank {
        &mut self.memory.oam
    }

    /// Get a VRAM memory bank by its index.
    pub fn get_vram(&self, bank: usize) -> &[u8] {
        self.memory.vram_banks[bank].as_slice()
    }

    /// Get a VRAM memory bank by its index.
    pub fn get_vram_mut(&mut self, bank: usize) -> &mut [u8] {
        self.memory.vram_banks[bank].as_slice_mut()
    }

    /// Get all palettes stored inside the PPU.
    /// This also contains the GBC palettes, even when not in GBC mode.
    pub fn get_palettes(&self) -> &Palettes {
        &self.memory.palettes
    }

    /// Get all palettes stored inside the PPU.
    /// This also contains the GBC palettes, even when not in GBC mode.
    pub fn get_palettes_mut(&mut self) -> &mut Palettes {
        &mut self.memory.palettes
    }

    /// Checks the LCD control register for a specific flag to be set.
    pub fn check_lcdc(&self, flag: LcdControlFlag) -> bool {
        self.registers.lcd_control.contains(flag)
    }

    /// Checks whether a specific interrupt type was enabled via LCD STAT register.
    pub fn is_interrupt_enabled(&self, interrupt: LcdInterruptFlag) -> bool {
        self.registers.lcd_interrupts.contains(interrupt)
    }

    /// Compute the background location of any screen pixel.
    pub fn screen_to_background(&self, screen_x: u8, screen_y: u8) -> (u8, u8) {
        let background_x = ((screen_x as u32 + self.registers.scroll_x as u32) & 0xff) as u8;
        let background_y = ((screen_y as u32 + self.registers.scroll_y as u32) & 0xff) as u8;
        (background_x, background_y)
    }

    /// Performs an OAM scan and stores it's result in the 'scanline' object.
    pub fn do_oam_scan_for_line(&self, line_number: u8) -> ScanlineData {
        let mut scanline = ScanlineData::new();
        scanline.line = line_number;

        let big_sprites = self.check_lcdc(LcdControlFlag::SpritesSize);
        let sprite_h    = if big_sprites { 16 } else { 8 };

        // sprite position 0 is not on scanline 0, but 16 pixel above the screen to
        // allow sprites being partially outside the screen.
        // Adjust the value here to avoid doing it for each check.
        let ly_plus_16 = line_number + 16;

        // iterate through all OAM entries
        for oam_entry in 0..40 {
            let sprite = self.memory.oam.get()[oam_entry as usize];

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
        // On GBC this behaviour can be switched by the object priority bit in 0xff6c
        //  - 0 means OAM position has priority
        //  - 1 means X position has priority
        if !self.device_config.is_gbc_enabled() || self.registers.object_priority {
            scanline.sprites[0 .. scanline.sprites_found as usize].sort_by(
                |a, b| {
                    let ax = a.pos_x;
                    let bx = b.pos_x;
                    ax.cmp(&bx)
                }
            );
        }

        scanline
    }

    /// Reads a pixel from the current scanline sprite data on a given x position.
    pub fn read_scanline_sprite_pixel(&self, scanline: &ScanlineData, x: u8) -> PixelFetchResult {
        // screen position considering the border offset of -8 / -16
        let screen_x = x + 8;
        let screen_y = scanline.line + 16;

        let big_sprites = self.check_lcdc(LcdControlFlag::SpritesSize);
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
            let sprite_pixel_x = flipped_if(screen_x - sprite.pos_x, sprite_w, sprite.is_flip_x());
            let sprite_pixel_y = flipped_if(screen_y - sprite.pos_y, sprite_h, sprite.is_flip_y());

            // if in GBC mode, check the sprite properties for the VRAM bank index
            // where to read the pixel data from
            let vram_bank_index = if self.device_config.is_gbc_enabled() {
                sprite.get_gbc_vram_bank()
            }
            else {
                0
            };

            // read the sprite pixel value
            let pixel = self.read_sprite_pixel(
                TileSet::H8000,
                sprite.tile & sprite_mask,
                vram_bank_index,
                sprite_pixel_x,
                sprite_pixel_y
            );

            // skip transparent pixels
            if pixel.is_transparent() {
                continue;
            }

            return PixelFetchResult {
                value: pixel,
                palette_dmg: sprite.get_dmg_palette(),
                palette_gbc: sprite.get_color_palette(),
                sprite_priority: 0,
                background_priority: sprite.is_bg_priority(),
            };
        }

        PixelFetchResult::none()
    }

    /// Reads a single pixel from the tilemap.
    pub fn read_tilemap_pixel(&self, tilemap: TileMap, tileset: TileSet, tilemap_x: u8, tilemap_y: u8) -> PixelFetchResult {
        let tile = self.read_tilemap_properties(tilemap, tileset, tilemap_x, tilemap_y);
        self.read_tile_pixel(&tile)
    }

    /// Creates a set of tilemap fetch properties, which will be used for a further read operation
    /// to read data from the tilemap.
    pub fn read_tilemap_properties(&self, tilemap: TileMap, tileset: TileSet, tilemap_x: u8, tilemap_y: u8) -> TileFetchProperties {
        let tile_x       = (tilemap_x / 8) as u16;
        let tile_y       = (tilemap_y / 8) as u16;
        let tile_pixel_x = (tilemap_x % 8) as u8;
        let tile_pixel_y = (tilemap_y % 8) as u8;
        let tile_index   = tile_y * 32 + tile_x;

        TileFetchProperties {
            tilemap,
            tileset,
            tile_index,
            tile_pixel_x,
            tile_pixel_y,
        }
    }

    /// Read the pixel value from a tile using previously created TileFetchProperties.
    pub fn read_tile_pixel(&self, tile: &TileFetchProperties) -> PixelFetchResult {
        let tile_address = (tile.tilemap.base_address() + tile.tile_index - MEMORY_LOCATION_VRAM_BEGIN) as usize;
        let vram0        = &self.memory.vram_banks[0];
        let sprite       = vram0.get_at(tile_address);

        let mut fetch_position_x    = tile.tile_pixel_x;
        let mut fetch_position_y    = tile.tile_pixel_y;
        let mut tile_vram_bank      = 0;
        let mut palette_gbc         = 0;
        let mut background_priority = false;

        if self.device_config.is_gbc_enabled() {
            // read tile attributes from the same location in VRAM1
            let vram1 = &self.memory.vram_banks[1];
            let tile_attr    = vram1.get_at(tile_address);

            // read properties from the tile attribute byte
            let bank_nr_bit   = get_bit(tile_attr, TILE_ATTR_BIT_VRAM_BANK);
            let is_h_flip     = get_bit(tile_attr, TILE_ATTR_BIT_H_FLIP);
            let is_v_flip     = get_bit(tile_attr, TILE_ATTR_BIT_V_FLIP);
            let bg_to_oam_bit = get_bit(tile_attr, TILE_ATTR_BIT_BG_TO_OAM_PRIO);

            // flip fetch coordinates on mirrored sprites
            flip_if(&mut fetch_position_x, 8, is_h_flip);
            flip_if(&mut fetch_position_y, 8, is_v_flip);

            palette_gbc         = tile_attr & 0x07;
            tile_vram_bank      = bank_nr_bit as u8;
            background_priority = bg_to_oam_bit;
        }

        // get the actual sprite pixel value
        let pixel = self.read_sprite_pixel(
            tile.tileset,
            sprite,
            tile_vram_bank,
            fetch_position_x,
            fetch_position_y
        );

        PixelFetchResult {
            value: pixel,
            palette_dmg: 0,
            palette_gbc,
            sprite_priority: 0,
            background_priority,
        }
    }

    /// Read the pixel value of a sprite.
    pub fn read_sprite_pixel(&self, tileset: TileSet, sprite: u8, bank: u8, x: u8, y: u8) -> SpritePixelValue {
        let sprite_address      = tileset.address_of_tile(sprite);
        self.read_sprite_pixel_from_address(sprite_address, bank, x, y)
    }

    /// Read the pixel value of a sprite.
    pub fn read_sprite_pixel_from_address(&self, sprite_address: u16, bank: u8, x: u8, y: u8) -> SpritePixelValue {
        let vram                = &self.memory.vram_banks[(bank & 0x01) as usize];
        let sprite_line_address = (sprite_address + (y as u16 * 2) - MEMORY_LOCATION_VRAM_BEGIN) as usize;
        let pixel_mask            = 1u8 << (7 - x);
        let byte0                 = vram.get_at(sprite_line_address + 0);
        let byte1                 = vram.get_at(sprite_line_address + 1);

        let pixel =
                (if (byte0 & pixel_mask) != 0 { 0x01 } else { 0x00 })
            |   (if (byte1 & pixel_mask) != 0 { 0x02 } else { 0x00 })
        ;

        SpritePixelValue::new(pixel)
    }

    /// Get the raw data of a sprite, as it is stored in the VRAM.
    pub fn get_sprite_image(&self, sprite_index: usize, bank: u8) -> SpriteImage {
        let vram = &self.memory.vram_banks[(bank & 0x01) as usize];
        let sprite_address_begin =  sprite_index * 16 ;
        let sprite_address_end   = (sprite_index * 16) + 16;

        let slice = &vram.as_slice()[sprite_address_begin .. sprite_address_end];
        let data = <[u8; 16]>::try_from(slice)
                .unwrap_or([0x00; 16]);

        SpriteImage::new(data)
    }
}


impl MemoryBusConnection for Ppu {
    fn on_read(&self, address: u16) -> u8 {
        memory_map!(address => {
            // Video RAM
            0x8000 ..= 0x9fff => [mapped_address] {
                let bank = &self.memory.vram_banks[self.memory.vram_active_bank as usize];
                bank.get_at(mapped_address)
            },

            // OAM memory
            0xfe00 ..= 0xfe9f => [mapped_address] {
                self.memory.oam.get_at(mapped_address)
            },

            // IO Registers
            0xff00 ..= 0xffff => [] {
                match address {
                    MEMORY_LOCATION_LCD_STATUS  => {
                        let mode_bits       = self.mode as u8;
                        let coincidence_bit = if self.current_line == self.registers.line_compare { 0b_0100 } else { 0b_0000 };
                        let interrupt_flags = self.registers.lcd_interrupts.bits();
                        let lcd_enabled_bit = match self.lcd_state { LcdState::Off => 0, _ => 1, } << 7;

                        lcd_enabled_bit | interrupt_flags | coincidence_bit | mode_bits
                    },

                    MEMORY_LOCATION_LCD_CONTROL     => self.registers.lcd_control.bits(),
                    MEMORY_LOCATION_SCY             => self.registers.scroll_y,
                    MEMORY_LOCATION_SCX             => self.registers.scroll_x,
                    MEMORY_LOCATION_LY              => self.current_line,
                    MEMORY_LOCATION_LYC             => self.registers.line_compare,
                    MEMORY_LOCATION_WY              => self.registers.window_y,
                    MEMORY_LOCATION_WX              => self.registers.window_x,

                    MEMORY_LOCATION_PALETTE_BG      => self.memory.palettes.bgp.into(),
                    MEMORY_LOCATION_PALETTE_OBP0    => self.memory.palettes.obp[0].into(),
                    MEMORY_LOCATION_PALETTE_OBP1    => self.memory.palettes.obp[1].into(),

                    MEMORY_LOCATION_VBK => {
                        // on GBC: get the active RAM bank
                        match self.device_config.emulation {
                            EmulationType::DMG => 0xff,

                            // register will contain the active bank in bit #0
                            // and all other bits set to 1
                            EmulationType::GBC => 0b_1111_1110 | self.memory.vram_active_bank,
                        }
                    },

                    MEMORY_LOCATION_BCPS => {
                        match self.device_config.emulation {
                            EmulationType::DMG => 0xff,
                            EmulationType::GBC => self.memory.palettes.gbc_background_palette_pointer.get()
                        }
                    }

                    MEMORY_LOCATION_BCPD => {
                        match self.device_config.emulation {
                            EmulationType::DMG => 0xff,
                            EmulationType::GBC => 
                                    self.memory.palettes.gbc_background_palette_pointer.read(
                                        &self.memory.palettes.gbc_background_palette
                                    ),
                        }
                    }

                    MEMORY_LOCATION_OCPS => {
                        match self.device_config.emulation {
                            EmulationType::DMG => 0xff,
                            EmulationType::GBC => self.memory.palettes.gbc_object_palette_pointer.get(),
                        }
                    }

                    MEMORY_LOCATION_OCPD => {
                        match self.device_config.emulation {
                            EmulationType::DMG => 0xff,
                            EmulationType::GBC => 
                                    self.memory.palettes.gbc_object_palette_pointer.read(
                                        &self.memory.palettes.gbc_object_palette
                                    ),
                        }
                    }

                    MEMORY_LOCATION_OPRI => {
                        self.registers.object_priority as u8
                    }

                    _ => 0xff,
                }
            }
        })
    }

    fn on_write(&mut self, address: u16, value: u8) {
        memory_map!(address => {
            // Video RAM
            0x8000 ..= 0x9fff => [mapped_address] {
                let bank = &mut self.memory.vram_banks[self.memory.vram_active_bank as usize];
                bank.set_at(mapped_address, value)
            },

            // OAM memory
            0xfe00 ..= 0xfe9f => [mapped_address] {
                self.memory.oam.set_at(mapped_address, value)
            },

            // IO registers
            0xff00 ..= 0xffff => [] {
                match address {
                    MEMORY_LOCATION_LCD_CONTROL => {
                        let was_enabled            = self.registers.lcd_control.contains(LcdControlFlag::LcdEnabled);
                        self.registers.lcd_control = LcdControl::new_truncated(value);
                        let is_enabled             = self.registers.lcd_control.contains(LcdControlFlag::LcdEnabled);

                        if was_enabled && !is_enabled {
                            self.on_ppu_reset();
                        }
                        else if is_enabled && !was_enabled {
                            self.on_ppu_enabled();
                        }
                    }

                    MEMORY_LOCATION_LCD_STATUS      => self.registers.lcd_interrupts    = LcdInterruptFlags::new_truncated(value),
                    MEMORY_LOCATION_SCY             => self.registers.scroll_y          = value,
                    MEMORY_LOCATION_SCX             => self.registers.scroll_x          = value,
                    MEMORY_LOCATION_LY              => self.current_line                = value,
                    MEMORY_LOCATION_LYC             => self.registers.line_compare      = value,
                    MEMORY_LOCATION_WY              => self.registers.window_y          = value,
                    MEMORY_LOCATION_WX              => self.registers.window_x          = value,

                    MEMORY_LOCATION_PALETTE_BG      => self.memory.palettes.bgp         = DmgPalette::from(value),
                    MEMORY_LOCATION_PALETTE_OBP0    => self.memory.palettes.obp[0]      = DmgPalette::from(value),
                    MEMORY_LOCATION_PALETTE_OBP1    => self.memory.palettes.obp[1]      = DmgPalette::from(value),

                    MEMORY_LOCATION_VBK => {
                        // on GBC: switch VRAM bank
                        if let EmulationType::GBC = self.device_config.emulation {
                            let bank = value & 0x01;
                            self.memory.vram_active_bank = bank;
                        }
                    },

                    MEMORY_LOCATION_BCPS => {
                        self.memory.palettes.gbc_background_palette_pointer.set(value)
                    }

                    MEMORY_LOCATION_BCPD => {
                        self.memory.palettes.gbc_background_palette_pointer.write(
                            &mut self.memory.palettes.gbc_background_palette,
                            value
                        )
                    }

                    MEMORY_LOCATION_OCPS => {
                        self.memory.palettes.gbc_object_palette_pointer.set(value)
                    }

                    MEMORY_LOCATION_OCPD => {
                        self.memory.palettes.gbc_object_palette_pointer.write(
                            &mut self.memory.palettes.gbc_object_palette,
                            value
                        )
                    }

                    MEMORY_LOCATION_OPRI => {
                        self.registers.object_priority = get_bit(value, 0)
                    }

                    _ => { }
                }
            }
        });
    }


    fn take_signals(&mut self) -> MemoryBusSignals {
        take(&mut self.signals)
    }
}


/// Get the flipped value if a sprite is mirrored.
fn flipped_if(value: u8, max_value: u8, flip: bool) -> u8 {
    if flip {
        max_value - value - 1
    }
    else {
        value
    }
}


/// Flips a value if a sprite is mirrored.
fn flip_if(value: &mut u8, max_value: u8, flip: bool) {
    if flip {
        *value = max_value - *value - 1;
    }
}
