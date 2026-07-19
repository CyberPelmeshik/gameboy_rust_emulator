use std::fs;

pub struct Mmu {
    vram: [u8; 8192],
    wram: [u8; 8192],
    oam: [u8; 160],
    io_registers: [u8; 128],
    hram: [u8; 127],
    rom: Vec<u8>,
    ie: u8,
}

impl Mmu {
    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = fs::read(path)?;
        self.rom = data;
        Ok(())
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom.get(addr as usize).copied().unwrap_or(0xFF),
            0x8000..=0x9FFF => self
                .vram
                .get((addr - 0x8000) as usize)
                .copied()
                .unwrap_or(0xFF),
            /*
            0xA000..=0xBFFF => self
                .eram
                .get((addr - 0xA000) as usize)
                .copied()
                .unwrap_or(0xFF),
            */
            0xC000..=0xDFFF => self
                .wram
                .get((addr - 0xC000) as usize)
                .copied()
                .unwrap_or(0xFF),
            0xE000..=0xFDFF => self
                .wram
                .get((addr - 0xE000) as usize)
                .copied()
                .unwrap_or(0xFF),
            0xFE00..=0xFE9F => self
                .oam
                .get((addr - 0xFE00) as usize)
                .copied()
                .unwrap_or(0xFF),
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => self
                .io_registers
                .get((addr - 0xFF00) as usize)
                .copied()
                .unwrap_or(0xFF),
            0xFF80..=0xFFFE => self
                .hram
                .get((addr - 0xFF80) as usize)
                .copied()
                .unwrap_or(0xFF),
            0xFFFF => self.ie,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => (),
            0x8000..=0x9FFF => {
                if let Some(mem) = self.vram.get_mut((addr - 0x8000) as usize) {
                    *mem = value
                }
            }
            /*
            0xA000..=0xBFFF => {
                if let Some(mem) = self.eram.get_mut((addr - 0xA000) as usize) {
                    *mem = value
                }
            },
            */
            0xC000..=0xDFFF => {
                if let Some(mem) = self.wram.get_mut((addr - 0xC000) as usize) {
                    *mem = value
                }
            }
            0xE000..=0xFDFF => {
                if let Some(mem) = self.wram.get_mut((addr - 0xE000) as usize) {
                    *mem = value
                }
            }
            0xFE00..=0xFE9F => {
                if let Some(mem) = self.oam.get_mut((addr - 0xFE00) as usize) {
                    *mem = value
                }
            }
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => {
                if let Some(mem) = self.io_registers.get_mut((addr - 0xFF00) as usize) {
                    *mem = value
                }
            }
            0xFF80..=0xFFFE => {
                if let Some(mem) = self.hram.get_mut((addr - 0xFF80) as usize) {
                    *mem = value
                }
            }
            0xFFFF => {
                self.ie = value;
            }
            _ => (),
        }
    }
}
