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
            0x80 => {}
            _ => {}
        }
    }
}
