use std::fs;

pub struct Mmu {
    vram: [u8; 8192],
    wram: [u8; 8192],
    oam: [u8; 160],
    io_registers: [u8; 128],
    hram: [u8; 127],
    rom: Vec<u8>,
    ie: u8,
    sb: String,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            vram: [0; 8192],
            wram: [0; 8192],
            oam: [0; 160],
            io_registers: [0; 128],
            hram: [0; 127],
            rom: Vec::new(),
            ie: 0,
            sb: String::new(),
        }
    }

    pub fn sb(&self) -> &String {
        return &self.sb;
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = fs::read(path)?;
        self.rom = data;
        //println!("{}", self.rom.len());
        Ok(())
    }

    pub fn load_bytes(&mut self, data: &[u8]) {
        self.rom = data.to_vec();
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let low_byte = self.read(addr);
        let high_byte = self.read(addr + 1);

        (high_byte as u16) << 8 | low_byte as u16
    }

    pub fn read_word_pair(&self, addr: u16) -> (u8, u8) {
        let low_byte = self.read(addr);
        let high_byte = self.read(addr + 1);

        return (high_byte, low_byte);
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write(addr, value as u8);
        self.write(addr + 1, (value >> 8) as u8);
    }

    pub fn write_word_pair(&mut self, addr: u16, value_1: u8, value_2: u8) {
        self.write(addr, value_1);
        self.write(addr + 1, value_2);
    }

    pub fn div_reg_add(&mut self, cycles: u8) {
        // Добавляет в регистр, выделенный для записи таймера, кол-во циклов процессора

        let value = self.read(0xFF04).wrapping_add(cycles);
        self.write(0xFF04, value);
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
                .eram.
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
                    if addr == 0xFF01 {
                        self.sb.push(value as char);
                    }

                    // При записи в регистр, выделенный для DIV, значение должно сбрасываться
                    if addr == 0xFF04 {
                        *mem = 0;
                        return;
                    }
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
