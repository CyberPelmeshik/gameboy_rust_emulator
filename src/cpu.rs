use crate::mmu::Mmu;

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

    pub fn read_word(&self, mmu: &Mmu, addr: u16) -> u16 {
        let low_byte = mmu.read(addr);
        let high_byte = mmu.read(addr + 1);

        (high_byte as u16) << 8 | low_byte as u16
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        let opcode = mmu.read(self.pc);

        match opcode {
            0x00 => {
                //NOP
                self.pc += 1;
                self.cycles += 4;
            },
            0x40 => {
                // LD B, B
                self.
            }
            _ => {}
        }
    }
}
