use crate::consts::{CHIP8_HEIGHT, CHIP8_WIDTH, FONTSET, PROGRAM_START};

pub struct Memory {
    pub vram: [bool; CHIP8_WIDTH * CHIP8_HEIGHT],
    pub ram: [u8; 4500],
    pub vram_modified: bool,
    pub audio_enabled: bool
}

impl Memory {
    pub fn new() -> Self {
        let vram = [false; CHIP8_WIDTH * CHIP8_HEIGHT];
        let ram = [0u8; 4500];
        let vram_modified = false;
        let audio_enabled = false;

        return Self {
            vram,
            ram,
            vram_modified,
            audio_enabled
        }
    }

    pub fn load_fonts(&mut self) {
        for (i, val) in FONTSET.iter().enumerate() {
            self.ram[i] = *val;
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let offset = PROGRAM_START;
        for (i, val) in rom.iter().enumerate() {
            self.ram[offset as usize + i] = *val;
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        return self.ram[address as usize];
    }

    pub fn write_ram(&mut self, address: u16, value:u8) {
        self.ram[address as usize] = value;
    }

    pub fn toggle_vram_mod(&mut self) {
        self.vram_modified = !self.vram_modified
    }

    pub fn clear_vram(&mut self) {
        self.vram = [false; CHIP8_WIDTH * CHIP8_HEIGHT];
    }
}