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
                // NOP

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
                self.cycles += 4;
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
                // INC C

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
                self.cycles += 4;
            }
            0x15 => {
                // DEC D

                self.dec(Register::D);

                self.pc += 1;
                self.cycles += 4;
            }
            0x16 => {
                // LD D, n8

                let value = mmu.read(self.pc + 1);
                self.set_register(Register::D, value);

                self.pc += 2;
                self.cycles += 8;
            }
            0x17 => {
                // RLA

                let flag_c_bit = self.flag_c as u8;
                self.flag_c = (self.a & 0x80) != 0;

                self.a = (self.a << 1) | (flag_c_bit);

                self.flag_z = false;
                self.flag_h = false;
                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x18 => {
                // JR e8

                self.pc = self.pc.wrapping_add((mmu.read(self.pc + 1) as i8) as u16);

                self.pc += 2;
                self.cycles += 12;
            }
            0x19 => {
                // ADD HL, DE

                let de = self.get_pair(Register::D, Register::E);
                let hl = self.get_pair(Register::H, Register::L);

                let value = de.wrapping_add(hl);

                self.set_pair(Register::H, Register::L, value);

                self.flag_n = false;
                self.flag_h = ((de & 0x0FFF) + (hl & 0x0FFF)) > 0x0FFF;
                self.flag_c = hl as u32 + de as u32 > 0xFFFF;

                self.pc += 1;
                self.cycles += 8;
            }
            0x1A => {
                // LD A, [DE]

                let de = self.get_pair(Register::D, Register::E);
                self.a = mmu.read(de);

                self.pc += 1;
                self.cycles += 8;
            }
            0x1B => {
                // DEC DE

                let mut de = self.get_pair(Register::D, Register::E);
                de = de.wrapping_sub(1);

                self.set_pair(Register::D, Register::E, de);

                self.pc += 1;
                self.cycles += 8;
            }
            0x1C => {
                // INC E

                self.inc(Register::E);

                self.pc += 1;
                self.cycles += 4;
            }
            0x1D => {
                // DEC E

                self.dec(Register::E);

                self.pc += 1;
                self.cycles += 4;
            }
            0x1E => {
                // LD E, n8

                self.e = mmu.read(self.pc + 1);

                self.pc += 2;
                self.cycles += 8;
            }
            0x1F => {
                // RRA

                let flag_c_bit = self.flag_c as u8;
                self.flag_c = (self.a & 0x01) != 0;

                self.a = (self.a >> 1) | (flag_c_bit << 7);

                self.flag_z = false;
                self.flag_h = false;
                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x20 => {
                // JR NZ, e8

                if !self.flag_z {
                    self.pc = self.pc.wrapping_add((mmu.read(self.pc + 1) as i8) as u16);

                    self.pc += 2;
                    self.cycles += 12;
                } else {
                    self.pc += 2;
                    self.cycles += 8;
                }
            }
            0x21 => {
                // LD HL, n16

                let (high, low) = mmu.read_word_pair(self.pc + 1);

                self.set_register(Register::H, high);
                self.set_register(Register::L, low);

                self.pc += 3;
                self.cycles += 12;
            }
            0x22 => {
                // LD [HL+], A

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.get_register(Register::A));
                self.set_pair(Register::H, Register::L, addr.wrapping_add(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x23 => {
                // INC HL

                let value = self.get_pair(Register::H, Register::L);
                self.set_pair(Register::H, Register::L, value.wrapping_add(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x24 => {
                // INC H

                self.inc(Register::H);

                self.pc += 1;
                self.cycles += 4;
            }
            0x25 => {
                // DEC H

                self.dec(Register::H);

                self.pc += 1;
                self.cycles += 4;
            }
            0x26 => {
                // LD H, n8

                let value = mmu.read(self.pc + 1);
                self.set_register(Register::H, value);

                self.pc += 2;
                self.cycles += 8;
            }
            0x27 => {
                // DAA

                if self.flag_n {
                    if self.flag_h {
                        self.a = self.a.wrapping_sub(0x06);
                    }

                    if self.flag_c {
                        self.c = self.c.wrapping_sub(0x60);
                    }
                } else {
                    if self.flag_h || (self.a & 0x0F) > 0x09 {
                        self.a = self.a.wrapping_add(0x06);
                    }

                    if self.flag_c || self.a > 0x99 {
                        self.c = self.c.wrapping_add(0x60);
                    }
                }

                let value = mmu.read(self.pc + 1);
                self.set_register(Register::H, value);

                self.flag_h = false;
                self.flag_n = false;
                self.flag_z = self.a == 0;

                if self.a > 0x99 {
                    self.flag_c = true;
                }

                self.pc += 1;
                self.cycles += 4;
            }
            0x28 => {
                // JR Z, e8

                if self.flag_z {
                    self.pc = self.pc.wrapping_add((mmu.read(self.pc + 1) as i8) as u16);

                    self.pc += 2;
                    self.cycles += 12;
                } else {
                    self.pc += 2;
                    self.cycles += 8;
                }
            }
            0x29 => {
                // ADD HL, HL

                let hl = self.get_pair(Register::H, Register::L);

                let value = hl.wrapping_add(hl);

                self.set_pair(Register::H, Register::L, value);

                self.flag_n = false;
                self.flag_h = ((hl & 0x0FFF) + (hl & 0x0FFF)) > 0x0FFF;
                self.flag_c = hl as u32 + hl as u32 > 0xFFFF;

                self.pc += 1;
                self.cycles += 8;
            }
            0x2A => {
                // LD A, [HL+]

                let hl = self.get_pair(Register::H, Register::L);
                self.a = mmu.read(hl);

                self.set_pair(Register::H, Register::L, hl.wrapping_add(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x2B => {
                // DEC HL

                let mut hl = self.get_pair(Register::H, Register::L);
                hl = hl.wrapping_sub(1);

                self.set_pair(Register::H, Register::L, hl);

                self.pc += 1;
                self.cycles += 8;
            }
            0x2C => {
                // INC L

                self.inc(Register::L);

                self.pc += 1;
                self.cycles += 4;
            }
            0x2D => {
                // INC L

                self.dec(Register::L);

                self.pc += 1;
                self.cycles += 4;
            }
            0x2E => {
                // LD L, n8

                self.l = mmu.read(self.pc + 1);

                self.pc += 2;
                self.cycles += 8;
            }
            0x2F => {
                // CPL

                self.a = !self.a;

                self.flag_n = true;
                self.flag_h = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x30 => {
                // JR NC, e8

                if !self.flag_c {
                    self.pc = self.pc.wrapping_add((mmu.read(self.pc + 1) as i8) as u16);

                    self.pc += 2;
                    self.cycles += 12;
                } else {
                    self.pc += 2;
                    self.cycles += 8;
                }
            }
            0x31 => {
                // LD SP, n16

                self.sp = mmu.read_word(self.pc + 1);

                self.pc += 3;
                self.cycles += 12;
            }
            0x32 => {
                // LD [HL-], A

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.get_register(Register::A));
                self.set_pair(Register::H, Register::L, addr.wrapping_sub(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x33 => {
                // INC SP

                self.sp = self.sp.wrapping_add(1);

                self.pc += 1;
                self.cycles += 8;
            }
            0x34 => {
                // INC [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr).wrapping_add(1);
                mmu.write(addr, value);

                self.flag_z = (value == 0x00);

                self.flag_n = false;

                self.flag_h = ((value & 0x0F) == 0);

                self.pc += 1;
                self.cycles += 12;
            }
            0x35 => {
                // DEC [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr).wrapping_sub(1);
                mmu.write(addr, value);

                self.flag_z = (value == 0x00);

                self.flag_n = true;

                self.flag_h = (value & 0x0F == 0x0F);

                self.pc += 1;
                self.cycles += 12;
            }
            0x36 => {
                // LD [HL], n8

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(self.pc + 1);

                mmu.write(addr, value);

                self.pc += 2;
                self.cycles += 12;
            }
            0x37 => {
                // SCF

                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x38 => {
                // JR C, e8

                if self.flag_c {
                    self.pc = self.pc.wrapping_add((mmu.read(self.pc + 1) as i8) as u16);

                    self.pc += 2;
                    self.cycles += 12;
                } else {
                    self.pc += 2;
                    self.cycles += 8;
                }
            }
            0x39 => {
                // ADD HL, SP

                let hl = self.get_pair(Register::H, Register::L);

                let value = hl.wrapping_add(self.sp);

                self.set_pair(Register::H, Register::L, value);

                self.flag_n = false;
                self.flag_h = ((hl & 0x0FFF) + (self.sp & 0x0FFF)) > 0x0FFF;
                self.flag_c = hl as u32 + self.sp as u32 > 0xFFFF;

                self.pc += 1;
                self.cycles += 8;
            }
            0x3A => {
                // LD A, [HL-]

                let hl = self.get_pair(Register::H, Register::L);
                self.a = mmu.read(hl);

                self.set_pair(Register::H, Register::L, hl.wrapping_sub(1));

                self.pc += 1;
                self.cycles += 8;
            }
            0x3B => {
                // DEC SP

                self.sp = self.sp.wrapping_sub(1);

                self.pc += 1;
                self.cycles += 8;
            }
            0x3C => {
                // INC A

                self.inc(Register::A);

                self.pc += 1;
                self.cycles += 4;
            }
            0x3D => {
                // DEC A

                self.dec(Register::A);

                self.pc += 1;
                self.cycles += 4;
            }
            0x3E => {
                // LD A, n8

                self.a = mmu.read(self.pc + 1);

                self.pc += 2;
                self.cycles += 8;
            }
            0x3F => {
                // CCF

                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = !self.flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x40 => {
                // LD B, B

                self.pc += 1;
                self.cycles += 4;
            }
            0x41 => {
                // LD B, C

                self.b = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x42 => {
                // LD B, D

                self.b = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x43 => {
                // LD B, E

                self.b = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x44 => {
                // LD B, H

                self.b = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x45 => {
                // LD B, L

                self.b = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x46 => {
                // LD B, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.b = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x47 => {
                // LD B, A

                self.b = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x48 => {
                // LD C, B

                self.c = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x49 => {
                // LD C, C

                self.pc += 1;
                self.cycles += 4;
            }
            0x4A => {
                // LD C, D

                self.c = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x4B => {
                // LD C, E

                self.c = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x4C => {
                // LD C, H

                self.c = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x4D => {
                // LD c, L

                self.c = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x4E => {
                // LD C, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.c = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x4F => {
                // LD C, A

                self.c = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x50 => {
                // LD D, B

                self.d = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x51 => {
                // LD D, C

                self.d = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x52 => {
                // LD D, D

                self.pc += 1;
                self.cycles += 4;
            }
            0x53 => {
                // LD D, E

                self.d = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x54 => {
                // LD D, H

                self.d = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x55 => {
                // LD D, L

                self.d = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x56 => {
                // LD D, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.d = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x57 => {
                // LD D, A

                self.d = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x58 => {
                // LD E, B

                self.e = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x59 => {
                // LD E, C

                self.e = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x5A => {
                // LD E, D

                self.e = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x5B => {
                // LD E, E

                self.pc += 1;
                self.cycles += 4;
            }
            0x5C => {
                // LD E, H

                self.e = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x5D => {
                // LD E, L

                self.e = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x5E => {
                // LD E, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.e = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x5F => {
                // LD E, A

                self.e = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x60 => {
                // LD H, B

                self.h = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x61 => {
                // LD H, C

                self.h = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x62 => {
                // LD H, D

                self.h = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x63 => {
                // LD H, E

                self.h = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x64 => {
                // LD H, H

                self.pc += 1;
                self.cycles += 4;
            }
            0x65 => {
                // LD H, L

                self.h = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x66 => {
                // LD H, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.h = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x67 => {
                // LD H, A

                self.h = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x68 => {
                // LD L, B

                self.l = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x69 => {
                // LD L, C

                self.l = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x6A => {
                // LD L, D

                self.l = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x6B => {
                // LD L, E

                self.l = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x6C => {
                // LD L, H

                self.l = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x6D => {
                // LD L, L

                self.pc += 1;
                self.cycles += 4;
            }
            0x6E => {
                // LD L, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.l = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x6F => {
                // LD L, A

                self.l = self.a;

                self.pc += 1;
                self.cycles += 4;
            }
            0x70 => {
                // LD [HL], B

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.b);

                self.pc += 1;
                self.cycles += 8;
            }
            0x71 => {
                // LD [HL], C

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.c);

                self.pc += 1;
                self.cycles += 8;
            }
            0x72 => {
                // LD [HL], D

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.d);

                self.pc += 1;
                self.cycles += 8;
            }
            0x73 => {
                // LD [HL], E

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.e);

                self.pc += 1;
                self.cycles += 8;
            }
            0x74 => {
                // LD [HL], H

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.h);

                self.pc += 1;
                self.cycles += 8;
            }
            0x75 => {
                // LD [HL], l

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.l);

                self.pc += 1;
                self.cycles += 8;
            }
            0x76 => {
                // HALT

                self.pc += 1;
                self.cycles += 8;
            }
            0x77 => {
                // LD [HL], A

                let addr = self.get_pair(Register::H, Register::L);
                mmu.write(addr, self.a);

                self.pc += 1;
                self.cycles += 8;
            }
            0x78 => {
                // LD A, B

                self.a = self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0x79 => {
                // LD A, C

                self.a = self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x7A => {
                // LD A, D

                self.a = self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0x7B => {
                // LD A, E

                self.a = self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0x7C => {
                // LD A, H

                self.a = self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0x7D => {
                // LD A, L

                self.a = self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0x7E => {
                // LD L, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                self.a = mmu.read(addr);

                self.pc += 1;
                self.cycles += 4;
            }
            0x7F => {
                // LD A, A

                self.pc += 1;
                self.cycles += 4;
            }
            0x80 => {
                // ADD A, B

                self.flag_h = ((self.a & 0x0F) + (self.b & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.b as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.b);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x81 => {
                // ADD A, С

                self.flag_h = ((self.a & 0x0F) + (self.c & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.c);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x82 => {
                // ADD A, С

                self.flag_h = ((self.a & 0x0F) + (self.d & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.d as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.d);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x83 => {
                // ADD A, E

                self.flag_h = ((self.a & 0x0F) + (self.e & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.e as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.e);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x84 => {
                // ADD A, H

                self.flag_h = ((self.a & 0x0F) + (self.h & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.h as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.h);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x85 => {
                // ADD A, L

                self.flag_h = ((self.a & 0x0F) + (self.l & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.l as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.l);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x86 => {
                // ADD A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.flag_h = ((self.a & 0x0F) + (value & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + value as u16 > 0xFF;

                self.a = self.a.wrapping_add(value);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 8;
            }
            0x87 => {
                // ADD A, A

                self.flag_h = ((self.a & 0x0F) + (self.a & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + self.a as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.a);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x88 => {
                // ADC A, B

                self.flag_h = ((self.a & 0x0F) + (self.b & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.b as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.b).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x89 => {
                // ADC A, C

                self.flag_h = ((self.a & 0x0F) + (self.c & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.c as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.c).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x8A => {
                // ADC A, D

                self.flag_h = ((self.a & 0x0F) + (self.d & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.d as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.d).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x8B => {
                // ADC A, E

                self.flag_h = ((self.a & 0x0F) + (self.e & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.e as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.e).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x8C => {
                // ADC A, H

                self.flag_h = ((self.a & 0x0F) + (self.h & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.h as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.h).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x8D => {
                // ADC A, L

                self.flag_h = ((self.a & 0x0F) + (self.l & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.l as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.l).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x8E => {
                // ADC A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.flag_h = ((self.a & 0x0F) + (value & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + value as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(value).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 8;
            }
            0x8F => {
                // ADC A, А

                self.flag_h = ((self.a & 0x0F) + (self.a & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + self.a as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(self.a).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 1;
                self.cycles += 4;
            }
            0x90 => {
                // SUB A, B

                self.flag_h = (self.a & 0x0F) < (self.b & 0x0F);
                self.flag_c = self.a < self.b;

                self.a = self.a.wrapping_sub(self.b);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x91 => {
                // SUB A, C

                self.flag_h = (self.a & 0x0F) < (self.c & 0x0F);
                self.flag_c = self.a < self.c;

                self.a = self.a.wrapping_sub(self.c);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x92 => {
                // SUB A, D

                self.flag_h = (self.a & 0x0F) < (self.d & 0x0F);
                self.flag_c = self.a < self.d;

                self.a = self.a.wrapping_sub(self.d);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x93 => {
                // SUB A, E

                self.flag_h = (self.a & 0x0F) < (self.e & 0x0F);
                self.flag_c = self.a < self.e;

                self.a = self.a.wrapping_sub(self.e);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x94 => {
                // SUB A, H

                self.flag_h = (self.a & 0x0F) < (self.h & 0x0F);
                self.flag_c = self.a < self.h;

                self.a = self.a.wrapping_sub(self.h);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x95 => {
                // SUB A, L

                self.flag_h = (self.a & 0x0F) < (self.l & 0x0F);
                self.flag_c = self.a < self.l;

                self.a = self.a.wrapping_sub(self.l);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x96 => {
                // SUB A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.flag_h = (self.a & 0x0F) < (value & 0x0F);
                self.flag_c = self.a < value;

                self.a = self.a.wrapping_sub(value);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 8;
            }
            0x97 => {
                // SUB A, A

                self.a = self.a.wrapping_sub(self.a);

                self.flag_z = true;
                self.flag_n = true;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0x98 => {
                // SBC A, B

                self.flag_h = (self.a & 0x0F) < (self.b & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.b as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.b).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x99 => {
                // SBC A, C

                self.flag_h = (self.a & 0x0F) < (self.c & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.c as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.c).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x9A => {
                // SBC A, D

                self.flag_h = (self.a & 0x0F) < (self.d & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.d as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.d).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x9B => {
                // SBC A, E

                self.flag_h = (self.a & 0x0F) < (self.e & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.e as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.e).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x9C => {
                // SBC A, H

                self.flag_h = (self.a & 0x0F) < (self.h & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.h as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.h).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x9D => {
                // SBC A, L

                self.flag_h = (self.a & 0x0F) < (self.l & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (self.l as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(self.l).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0x9E => {
                // SBC A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.flag_h = (self.a & 0x0F) < (value & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (value as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(value).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 8;
            }
            0x9F => {
                // SBC A, A

                self.flag_h = (self.a & 0x0F) < (self.a & 0x0F) + self.flag_c as u8;

                self.a = self.a.wrapping_sub(self.a).wrapping_sub(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA0 => {
                // AND A, B

                self.a = self.a & self.b;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA1 => {
                // AND A, C

                self.a = self.a & self.c;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA2 => {
                // AND A, D

                self.a = self.a & self.d;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA3 => {
                // AND A, E

                self.a = self.a & self.e;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA4 => {
                // AND A, H

                self.a = self.a & self.h;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA5 => {
                // AND A, L

                self.a = self.a & self.l;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA6 => {
                // AND A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.a = self.a & value;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 8;
            }
            0xA7 => {
                // AND A, A

                self.a = self.a & self.a;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA8 => {
                // XOR A, B

                self.a = self.a ^ self.b;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xA9 => {
                // XOR A, С

                self.a = self.a ^ self.c;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xAA => {
                // XOR A, D

                self.a = self.a ^ self.d;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xAB => {
                // XOR A, E

                self.a = self.a ^ self.e;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xAC => {
                // XOR A, H

                self.a = self.a ^ self.h;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xAD => {
                // XOR A, L

                self.a = self.a ^ self.l;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xAE => {
                // XOR A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.a = self.a ^ value;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 8;
            }
            0xAF => {
                // XOR A, A

                self.a = self.a ^ self.a;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB0 => {
                // OR A, B

                self.a = self.a | self.b;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB1 => {
                // OR A, C

                self.a = self.a | self.c;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB2 => {
                // OR A, D

                self.a = self.a | self.d;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB3 => {
                // OR A, E

                self.a = self.a | self.e;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB4 => {
                // OR A, H

                self.a = self.a | self.h;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB5 => {
                // OR A, L

                self.a = self.a | self.l;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB6 => {
                // OR A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.a = self.a | value;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 8;
            }
            0xB7 => {
                // OR A, A

                self.a = self.a | self.a;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB8 => {
                // CP A, B

                self.flag_z = self.a == self.b;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.b & 0x0f);
                self.flag_c = self.a < self.b;

                self.pc += 1;
                self.cycles += 4;
            }
            0xB9 => {
                // CP A, C

                self.flag_z = self.a == self.c;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.c & 0x0f);
                self.flag_c = self.a < self.c;

                self.pc += 1;
                self.cycles += 4;
            }
            0xBA => {
                // CP A, D

                self.flag_z = self.a == self.d;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.d & 0x0f);
                self.flag_c = self.a < self.d;

                self.pc += 1;
                self.cycles += 4;
            }
            0xBB => {
                // CP A, E

                self.flag_z = self.a == self.e;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.e & 0x0f);
                self.flag_c = self.a < self.e;

                self.pc += 1;
                self.cycles += 4;
            }
            0xBC => {
                // CP A, H

                self.flag_z = self.a == self.h;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.h & 0x0f);
                self.flag_c = self.a < self.h;

                self.pc += 1;
                self.cycles += 4;
            }
            0xBD => {
                // CP A, L

                self.flag_z = self.a == self.l;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (self.l & 0x0f);
                self.flag_c = self.a < self.l;

                self.pc += 1;
                self.cycles += 4;
            }
            0xBE => {
                // CP A, [HL]

                let addr = self.get_pair(Register::H, Register::L);
                let value = mmu.read(addr);

                self.flag_z = self.a == value;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (value & 0x0f);
                self.flag_c = self.a < value;

                self.pc += 1;
                self.cycles += 8;
            }
            0xBF => {
                // CP A, A

                self.flag_z = true;
                self.flag_n = true;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xC0 => {
                // RET NZ

                if !self.flag_z {
                    self.pc = mmu.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                    self.pc += 1;
                }
            }
            0xC1 => {
                // POP BC

                let addr = mmu.read_word(self.sp);
                self.c = (addr & 0xFF) as u8;
                self.b = (addr >> 8 & 0xFF) as u8;
                self.sp += 2;

                self.cycles += 12;
                self.pc += 1;
            }
            0xC2 => {
                // JP NZ, a16

                if !self.flag_z {
                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xC3 => {
                // JP a16

                self.pc = mmu.read_word(self.pc + 1);
                self.cycles += 16;
            }
            0xC4 => {
                // CALL NZ, a16

                if !self.flag_z {
                    self.sp -= 2;
                    mmu.write_word(self.sp, self.pc + 3);

                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xC5 => {
                // PUSH BC

                mmu.write(self.sp - 1, self.b);
                mmu.write(self.sp - 2, self.c);
                self.sp -= 2;

                self.cycles += 16;
                self.pc += 1;
            }
            0xC6 => {
                // ADD A, n8

                let data = mmu.read(self.pc + 1);

                self.flag_h = ((self.a & 0x0F) + (data & 0x0F)) > 0x0F;
                self.flag_c = self.a as u16 + data as u16 > 0xFF;

                self.a = self.a.wrapping_add(data);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.pc += 2;
                self.cycles += 8;
            }
            0xC7 => {
                // RST $00

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0000;
                self.cycles += 16;
            }
            0xC8 => {
                // RET Z

                if self.flag_z {
                    self.pc = mmu.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                    self.pc += 1;
                }
            }
            0xC9 => {
                // RET

                self.pc = mmu.read_word(self.sp);
                self.sp += 2;
                self.cycles += 16;
            }
            0xCA => {
                // JP Z, a16

                if self.flag_z {
                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xCB => {
                // PREFIX

                // Доработать
            }
            0xCC => {
                // CALL Z, a16

                if self.flag_z {
                    self.sp -= 2;
                    mmu.write_word(self.sp, self.pc + 3);

                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xCD => {
                // CALL a16

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 3);
                self.pc = mmu.read_word(self.pc + 1);
                self.cycles += 24;
            }
            0xCE => {
                // ADC A, n8

                let value = mmu.read(self.pc + 1);

                self.flag_h = ((self.a & 0x0F) + (value & 0x0F) + self.flag_c as u8) > 0x0F;
                let new_flag_c = self.a as u16 + value as u16 + self.flag_c as u16 > 0xFF;

                self.a = self.a.wrapping_add(value).wrapping_add(self.flag_c as u8);

                self.flag_z = self.a == 0;

                self.flag_n = false;

                self.flag_c = new_flag_c;

                self.pc += 2;
                self.cycles += 8;
            }
            0xCF => {
                // RST $08

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0008;
                self.cycles += 16;
            }
            0xD0 => {
                // RET NC

                if !self.flag_c {
                    self.pc = mmu.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                    self.pc += 1;
                }
            }
            0xD1 => {
                // POP DE

                let addr = mmu.read_word(self.sp);
                self.e = (addr & 0xFF) as u8;
                self.d = (addr >> 8 & 0xFF) as u8;
                self.sp += 2;

                self.cycles += 12;
                self.pc += 1;
            }
            0xD2 => {
                // JP NC, a16

                if !self.flag_c {
                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xD3 => {
                // Pass
            }
            0xD4 => {
                // CALL NC, a16

                if !self.flag_c {
                    self.sp -= 2;
                    mmu.write_word(self.sp, self.pc + 3);

                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xD5 => {
                // PUSH DE

                mmu.write(self.sp - 1, self.d);
                mmu.write(self.sp - 2, self.e);
                self.sp -= 2;

                self.cycles += 16;
                self.pc += 1;
            }
            0xD6 => {
                // SUB A, n8

                let data = mmu.read(self.pc + 1);

                self.flag_h = (self.a & 0x0F) < (data & 0x0F);
                self.flag_c = self.a < data;

                self.a = self.a.wrapping_sub(data);

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 2;
                self.cycles += 8;
            }
            0xD7 => {
                // RST $10

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0010;
                self.cycles += 16;
            }
            0xD8 => {
                // RET C

                if self.flag_c {
                    self.pc = mmu.read_word(self.sp);
                    self.sp += 2;
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                    self.pc += 1;
                }
            }
            0xD9 => {
                // RETI

                self.pc = mmu.read_word(self.sp);
                self.sp += 2;
                self.ime = true;
                self.cycles += 16;
            }
            0xDA => {
                // JP C, a16

                if self.flag_c {
                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xDB => {
                // Pass
            }
            0xDC => {
                // CALL C, a16

                if self.flag_c {
                    self.sp -= 2;
                    mmu.write_word(self.sp, self.pc + 3);

                    self.pc = mmu.read_word(self.pc + 1);
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                    self.pc += 3;
                }
            }
            0xDD => {
                // Pass
            }
            0xDE => {
                // SBC A, n8

                let data = mmu.read(self.pc + 1);

                self.flag_h = (self.a & 0x0F) < (data & 0x0F) + self.flag_c as u8;
                let new_flag_c = (self.a as u16) < (data as u16) + (self.flag_c as u16);

                self.a = self.a.wrapping_sub(data).wrapping_sub(self.flag_c as u8);

                self.flag_c = new_flag_c;

                self.flag_z = self.a == 0;

                self.flag_n = true;

                self.pc += 2;
                self.cycles += 8;
            }
            0xDF => {
                // RST $18

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0018;
                self.cycles += 16;
            }
            0xE0 => {
                // LDH [a8], A

                let addr = (mmu.read(self.pc + 1) as u16) + 0xFF00;
                mmu.write(addr, self.a);

                self.cycles += 12;
                self.pc += 2;
            }
            0xE1 => {
                // POP HL

                let addr = mmu.read_word(self.sp);
                self.l = (addr & 0xFF) as u8;
                self.h = (addr >> 8 & 0xFF) as u8;
                self.sp += 2;

                self.cycles += 12;
                self.pc += 1;
            }
            0xE2 => {
                // LDH [C], A

                let addr = (self.c as u16) + 0xFF00;
                mmu.write(addr, self.a);

                self.cycles += 8;
                self.pc += 1;
            }
            0xE3 => {
                // Pass
            }
            0xE4 => {
                // Pass
            }
            0xE5 => {
                // PUSH HL

                mmu.write(self.sp - 1, self.h);
                mmu.write(self.sp - 2, self.l);
                self.sp -= 2;

                self.cycles += 16;
                self.pc += 1;
            }
            0xE6 => {
                // AND A, n8

                let data = mmu.read(self.pc + 1);

                self.a = self.a & data;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = true;
                self.flag_c = false;

                self.pc += 1;
                self.cycles += 4;
            }
            0xE7 => {
                // RST $20

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0020;
                self.cycles += 16;
            }
            0xE8 => {
                // ADD SP, e8

                let value = (mmu.read(self.pc + 1) as i8) as u16;

                self.flag_h = ((self.sp & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
                self.flag_c = self.sp as u32 + value as u32 > 0xFFFF;

                self.sp = self.sp.wrapping_add(value);

                self.flag_z = false;

                self.flag_n = false;

                self.pc += 2;
                self.cycles += 16;
            }
            0xE9 => {
                // JP HL

                self.pc = self.get_pair(Register::H, Register::L);
                self.cycles += 4;
            }
            0xEA => {
                // LD [a16], A

                let addr = mmu.read_word(self.pc + 1);
                mmu.write(addr, self.a);

                self.pc += 3;
                self.cycles += 16;
            }
            0xEB => {
                // Pass
            }
            0xEC => {
                // Pass
            }
            0xED => {
                // Pass
            }
            0xEE => {
                // XOR A, n8

                let value = mmu.read(self.pc + 1);
                self.a = self.a ^ value;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 2;
                self.cycles += 8;
            }
            0xEF => {
                // RST $28

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0028;
                self.cycles += 16;
            }
            0xF0 => {
                // LDH A, [a8]

                let addr = (mmu.read(self.pc + 1) as u16) + 0xFF00;
                self.a = mmu.read(addr);

                self.cycles += 12;
                self.pc += 2;
            }
            0xF1 => {
                // POP AF

                let f = mmu.read(self.sp);
                self.flag_z = (f & 0x80) != 0;
                self.flag_n = (f & 0x40) != 0;
                self.flag_h = (f & 0x20) != 0;
                self.flag_c = (f & 0x10) != 0;

                self.a = mmu.read(self.sp + 1);
                self.sp += 2;

                self.cycles += 12;
                self.pc += 1;
            }
            0xF2 => {
                // LDH A, [C]

                let addr = (self.c as u16) + 0xFF00;
                let value = mmu.read(addr);
                self.a = value;

                self.cycles += 8;
                self.pc += 1;
            }
            0xF3 => {
                // DI

                self.ime = false;

                self.cycles += 4;
                self.pc += 1;
            }
            0xF4 => {
                // Pass
            }
            0xF5 => {
                // PUSH AF

                let f = ((self.flag_z as u8) << 7)
                    | ((self.flag_n as u8) << 6)
                    | ((self.flag_h as u8) << 5)
                    | ((self.flag_c as u8) << 4);

                mmu.write(self.sp - 1, self.a);
                mmu.write(self.sp - 2, f);
                self.sp -= 2;

                self.cycles += 16;
                self.pc += 1;
            }
            0xF6 => {
                // OR A, n8

                let value = mmu.read(self.pc + 1);
                self.a = self.a | value;

                self.flag_z = self.a == 0;
                self.flag_n = false;
                self.flag_h = false;
                self.flag_c = false;

                self.pc += 2;
                self.cycles += 8;
            }
            0xF7 => {
                // RST $30

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0030;
                self.cycles += 16;
            }
            0xF8 => {
                // LD HL, SP + e8

                let e8 = (mmu.read(self.pc + 1) as i8) as u16;
                let value = self.sp.wrapping_add(e8);
                self.set_pair(Register::H, Register::L, value);

                self.flag_z = false;
                self.flag_n = false;

                self.flag_h = ((self.sp & 0x0FFF) + (e8 & 0x0FFF)) > 0x0FF;
                self.flag_c = self.sp as u32 + e8 as u32 > 0xFFFF;

                self.pc += 2;
                self.cycles += 12;
            }
            0xF9 => {
                // LD SP, HL

                self.sp = self.get_pair(Register::H, Register::L);

                self.pc += 1;
                self.cycles += 8;
            }
            0xFA => {
                // LD A, [a16]

                let addr = mmu.read_word(self.pc + 1);
                self.a = mmu.read(addr);

                self.pc += 3;
                self.cycles += 16;
            }
            0xFB => {
                // EI

                self.ime = true;

                self.pc += 1;
                self.cycles += 4;
            }
            0xFC => {
                // Pass
            }
            0xFD => {
                // Pass
            }
            0xFE => {
                // CP A, n8

                let value = mmu.read(self.pc + 1);

                self.flag_z = self.a == value;
                self.flag_n = true;
                self.flag_h = (self.a & 0x0F) < (value & 0x0f);
                self.flag_c = self.a < value;

                self.pc += 1;
                self.cycles += 4;
            }
            0xFF => {
                // RST $38

                self.sp -= 2;
                mmu.write_word(self.sp, self.pc + 1);

                self.pc = 0x0038;
                self.cycles += 16;
            }
            _ => {}
        }
    }
}
