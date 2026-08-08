use crate::mmu::Mmu;

/*
enum Instruction {
  ADD(ArithmeticTarget),
}
*/

#[derive(Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flag_z: bool, // Последняя мат. операция равна нулю или два операнда оказались равными при сравнении
    flag_n: bool, // Последння операция вычитание
    flag_h: bool, // В последней мат операции был перенос из младшего полу-байта
    flag_c: bool, // Произошел переном при последней мат операции
    pc: u16,      // Счетсчик исструкций, указывает на следующую инструкцию
    sp: u16,      // Вершина стека
    ime: bool,    // Флаг обработки прерываний?
    cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flag_z: false,
            flag_n: false,
            flag_h: false,
            flag_c: false,
            pc: 0x0100,
            sp: 0xFFFE,
            ime: false,
            cycles: 0,
        }
    }

    fn get_register(&self, x: Register) -> u8 {
        match x {
            Register::A => return self.a,
            Register::B => return self.b,
            Register::C => return self.c,
            Register::D => return self.d,
            Register::E => return self.e,
            Register::H => return self.h,
            Register::L => return self.l,
        }
    }

    fn get_pair(&self, high_reg: Register, low_reg: Register) -> u16 {
        let high_byte: u8;
        let low_byte: u8;

        match high_reg {
            Register::A => high_byte = self.a,
            Register::B => high_byte = self.b,
            Register::C => high_byte = self.c,
            Register::D => high_byte = self.d,
            Register::E => high_byte = self.e,
            Register::H => high_byte = self.h,
            Register::L => high_byte = self.l,
        }

        match low_reg {
            Register::A => low_byte = self.a,
            Register::B => low_byte = self.b,
            Register::C => low_byte = self.c,
            Register::D => low_byte = self.d,
            Register::E => low_byte = self.e,
            Register::H => low_byte = self.h,
            Register::L => low_byte = self.l,
        }

        return (high_byte as u16) << 8 | low_byte as u16;
    }

    fn set_pair(&mut self, high_reg: Register, low_reg: Register, value: u16) {
        let high_value: u8 = (value >> 8) as u8;
        let low_value: u8 = value as u8;

        match high_reg {
            Register::A => self.a = high_value,
            Register::B => self.b = high_value,
            Register::C => self.c = high_value,
            Register::D => self.d = high_value,
            Register::E => self.e = high_value,
            Register::H => self.h = high_value,
            Register::L => self.l = high_value,
        };

        match low_reg {
            Register::A => self.a = low_value,
            Register::B => self.b = low_value,
            Register::C => self.c = low_value,
            Register::D => self.d = low_value,
            Register::E => self.e = low_value,
            Register::H => self.h = low_value,
            Register::L => self.l = low_value,
        };
    }

    fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    fn inc(&mut self, reg: Register) -> u8 {
        let value = self.get_register(reg).wrapping_add(1);
        self.set_register(reg, value);

        self.flag_z = (value == 0x00);

        self.flag_n = false;

        self.flag_h = ((value & 0x0F) == 0);

        return value;
    }

    fn dec(&mut self, reg: Register) -> u8 {
        let value = self.get_register(reg).wrapping_sub(1);
        self.set_register(reg, value);

        self.flag_z = (value == 0x00);

        self.flag_n = true;

        self.flag_h = (value & 0x0F == 0x0F);

        return value;
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        let opcode = mmu.read(self.pc);

        match opcode {
            0x00 => {
                //NOP
                self.pc += 1;
                self.cycles += 4;
            }
            0x01 => {
                // LD BC, n16

                let (high, low) = mmu.read_word_pair(self.pc + 1);

                self.set_register(Register::B, high);
                self.set_register(Register::C, low);

                self.pc += 3;
                self.cycles += 12;
            }
            0x02 => {
                // LD [BC], A

                let addr = self.get_pair(Register::B, Register::C);
                mmu.write(addr, self.get_register(Register::A));

                self.pc += 1;
                self.cycles += 8;
            }
            0x03 => {
                // INC BC

                let value = self.get_pair(Register::B, Register::C);
                self.set_pair(Register::B, Register::C, value.wrapping_add(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x04 => {
                // INC B

                self.inc(Register::B);

                self.pc += 1;
                self.cycles += 4;
            }
            0x05 => {
                // DEC B

                self.dec(Register::B);

                self.pc += 1;
                self.cycles += 4;
            }
            0x06 => {
                // LD B, n8

                let value = mmu.read(self.pc + 1);
                self.set_register(Register::B, value);

                self.pc += 2;
                self.cycles += 8;
            }
            0x07 => {
                // RLCA
                self.flag_c = (self.a & 0x80) != 0;

                self.a = (self.a << 1) | (self.a >> 7);

                self.flag_z = false;
                self.flag_h = false;
                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x08 => {
                //LD [a16], SP

                let addr = mmu.read_word(self.pc + 1);
                mmu.write_word(addr, self.sp);

                self.pc += 3;
                self.cycles += 20;
            }
            0x09 => {
                // ADD HL, BC

                let bc = self.get_pair(Register::B, Register::C);
                let hl = self.get_pair(Register::H, Register::L);

                let value = bc.wrapping_add(hl);

                self.set_pair(Register::H, Register::L, value);

                self.flag_n = false;
                self.flag_h = ((bc & 0x0FFF) + (hl & 0x0FFF)) > 0x0FFF;
                self.flag_c = hl as u32 + bc as u32 > 0xFFFF;

                self.pc += 1;
                self.cycles += 8;
            }
            0x0A => {
                // LD A, [BC]

                let bc = self.get_pair(Register::B, Register::C);
                self.a = mmu.read(bc);

                self.pc += 1;
                self.cycles += 8;
            }
            0x0B => {
                // DEC BC

                let mut bc = self.get_pair(Register::B, Register::C);
                bc = bc.wrapping_sub(1);

                self.set_pair(Register::B, Register::C, bc);

                self.pc += 1;
                self.cycles += 8;
            }
            0x0C => {
                // INC B

                self.inc(Register::C);

                self.pc += 1;
                self.cycles += 4;
            }
            0x0D => {
                // DEC C

                self.dec(Register::C);

                self.pc += 1;
                self.cycles += 4;
            }
            0x0E => {
                // LD C, n8

                self.c = mmu.read(self.pc + 1);

                self.pc += 2;
                self.cycles += 8;
            }
            0x0F => {
                // RRCA

                self.flag_c = (self.a & 0x01) != 0;

                self.a = (self.a >> 1) | (self.a << 7);

                self.flag_z = false;
                self.flag_h = false;
                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x10 => {
                // STOP n8

                // Потом доработать, пока не надо

                self.pc += 2;
                self.cycles += 4;
            }
            0x11 => {
                // LD DE, n16

                let (high, low) = mmu.read_word_pair(self.pc + 1);

                self.set_register(Register::D, high);
                self.set_register(Register::E, low);

                self.pc += 3;
                self.cycles += 12;
            }
            0x12 => {
                // LD [DE], A

                let addr = self.get_pair(Register::D, Register::E);
                mmu.write(addr, self.get_register(Register::A));

                self.pc += 1;
                self.cycles += 8;
            }
            0x13 => {
                // INC DE

                let value = self.get_pair(Register::D, Register::E);
                self.set_pair(Register::D, Register::E, value.wrapping_add(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x14 => {
                // INC D

                self.inc(Register::D);

                self.pc += 1;
                self.cycles += 8;
            }
            0x15 => {
                // DEC D

                self.dec(Register::D);

                self.pc += 1;
                self.cycles += 8;
            }
            0x16 => {
                // LD D, n8

                let value = mmu.read(self.pc + 1);
                self.set_register(Register::D, value);

                self.pc += 2;
                self.cycles += 8;
            }
            0x16 => {
                // RLA

                // RLCA
                self.flag_c = (self.a & 0x80) != 0;

                self.a = (self.a << 1) | (self.a >> 7);

                self.flag_z = false;
                self.flag_h = false;
                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x21 => {
                // LD HL, n16

                let (high, low) = mmu.read_word_pair(self.pc + 1);

                self.set_register(Register::H, high);
                self.set_register(Register::L, low);

                self.pc += 3;
                self.cycles += 12;
            }
            0x31 => {
                // LD SP, n16

                self.sp = mmu.read_word(self.pc + 1);

                self.pc += 3;
                self.cycles += 12;
            }
            0x42 => {
                // LD B, C

                self.b = self.c;

                self.pc += 1;
                self.cycles += 4;
            }

            _ => {}
        }
    }
}
