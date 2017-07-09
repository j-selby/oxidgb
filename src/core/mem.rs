/**
 * mem.rs
 *
 * Handles the Gameboy's memory bus.
**/

use core::rom::GameROM;
use core::gpu::GPU;
use core::io;

pub struct GBMemory {
    pub rom : GameROM,
    pub ram : [u8; 8192],
    pub high_ram : [u8; 127 /* - interrupt enable reg */],
    pub gpu : GPU
}

impl GBMemory {
    /// Reads a value from memory. 0xFF if invalid.
    pub fn read(&self, ptr : u16) -> u8 {
        //println!("${:04X}: Read", ptr);
        match ptr {
            0xFFFF => { // Interrupt enable reg
                //println!("WARN: Reading from interrupt enable reg: {:04x}", ptr);
                return 0xFF;
            }
            0xFF80 ... 0xFFFE => { // High internal RAM
                return self.high_ram[(ptr - 0xFF80) as usize];
            }
            0xFF4C ... 0xFF7F => { // Unusable
                //println!("WARN: Reading from unreadable memory: {:04x}", ptr);
                return 0xFF;
            }
            0xFF00 ... 0xFF4B => { // I/O Registers
                return io::read(self, (ptr & 0xFF) as u8);
            }
            0xFEA0 ... 0xFEFF => { // Unusable
                //println!("WARN: Reading from unreadable memory: {:04x}", ptr);
                return 0xFF;
            }
            0xFE00 ... 0xFE9F => { // OAM
                return self.gpu.oam[(ptr - 0xFE00) as usize];
            }
            0xE000 ... 0xFDFF => { // RAM Echo
                return self.ram[(ptr - 0xE000) as usize];
            }
            0xC000 ... 0xDFFF => { // Internal RAM
                return self.ram[(ptr - 0xC000) as usize];
            }
            0xA000 ... 0xBFFF => { // Switchable RAM
                return self.rom.read_ram(ptr - 0xA000);
            }
            0x8000 ... 0x9FFF => { // GPU
                return self.gpu.vram[(ptr - 0x8000) as usize];
            }
            0x0000 ... 0x7FFF => { // Cartridge / Switchable ROM
                return self.rom.read(ptr);
            }
            _ => {
                panic!("Programmer error: {:04x} was not matched!", ptr);
            }
        }
    }

    /// Writes a value to a memory location if possible.
    pub fn write(&mut self, ptr : u16, val : u8) {
        //println!("${:04X}: Write ${:02X}", ptr, val);

        match ptr {
            0xFFFF => { // Interrupt enable reg
                //println!("WARN: Writing to interrupt enable reg: {:04x} = {:02x}", ptr, val);
            }
            0xFF80 ... 0xFFFE => { // High internal RAM
                self.high_ram[(ptr - 0xFF80) as usize] = val;
            }
            0xFF4C ... 0xFF7F => { // Unusable
                //println!("WARN: Writing to unreadable memory: {:04x} = {:02x}", ptr, val);
            }
            0xFF00 ... 0xFF4B => { // I/O Registers
                io::write(self, (ptr & 0xFF) as u8, val);
            }
            0xFEA0 ... 0xFEFF => { // Unusable
                //println!("WARN: Writing to unreadable memory: {:04x} = {:02x}", ptr, val);
            }
            0xFE00 ... 0xFE9F => { // OAM
                self.gpu.oam[(ptr - 0xFE00) as usize] = val;
            }
            0xE000 ... 0xFDFF => { // RAM Echo
                self.ram[(ptr - 0xE000) as usize] = val;
            }
            0xC000 ... 0xDFFF => { // Internal RAM
                self.ram[(ptr - 0xC000) as usize] = val;
            }
            0xA000 ... 0xBFFF => { // Switchable RAM
                self.rom.write_ram(ptr - 0xA000, val);
            }
            0x8000 ... 0x9FFF => { // GPU
                self.gpu.vram[(ptr - 0x8000) as usize] = val;
            }
            0x0000 ... 0x7FFF => { // Cartridge / Switchable ROM
                self.rom.write(ptr, val);
            }
            _ => {
                panic!("Programmer error: {:04x} = {:02x} was not matched!", ptr, val);
            }
        }
    }

    /// Reads a short. 0xFFFF if invalid.
    pub fn read_short(&self, ptr : u16) -> u16 {
        return (self.read(ptr) as u16) | ((self.read(ptr + 1) as u16) << 8);
    }

    /// Writes a short value to a memory location if possible.
    pub fn write_short(&mut self, ptr : u16, val : u16) {
        self.write(ptr, (val & 0xFF) as u8);
        self.write(ptr + 1, ((val >> 8) & 0xFF) as u8);
    }

    /// Builds a new memory manager.
    pub fn build(rom : GameROM) -> GBMemory {
        return GBMemory {
            rom : rom,
            ram : [0; 8192],
            high_ram : [0; 127],
            gpu : GPU::build()
        }
    }
}