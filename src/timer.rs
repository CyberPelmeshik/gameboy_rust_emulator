pub struct Timer {
    system_counter: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            system_counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn add_cycles(&mut self, cycles: u8) {
        let old_system_counter = self.system_counter;
        self.system_counter = self.system_counter.wrapping_add(cycles as u16);
        let timer_status = self.tac & 0x04 == 4;
        let timer_inc = self.tac & 0x03;

        if timer_status {
            match timer_inc {
                0x00 => {
                    if ((old_system_counter & 0x0400) == 1024)
                        & ((self.system_counter & 0x0400) == 0)
                    {
                        self.tima = self.tima.wrapping_add(1);
                    }
                }
                0x01 => {
                    if ((old_system_counter & 0x0010) == 16) & ((self.system_counter & 0x0010) == 0)
                    {
                        self.tima = self.tima.wrapping_add(1);
                    }
                }
                0x02 => {
                    if ((old_system_counter & 0x0040) == 64) & ((self.system_counter & 0x0040) == 0)
                    {
                        self.tima = self.tima.wrapping_add(1);
                    }
                }
                0x03 => {
                    if ((old_system_counter & 0x0100) == 256)
                        & ((self.system_counter & 0x0100) == 0)
                    {
                        self.tima = self.tima.wrapping_add(1);
                    }
                }
                _ => panic!("Timer state bigger then 3!"),
            }
            if self.tima == 0 {
                self.tima = self.tma;
            }
        }
    }

    pub fn read_div(&self) -> u8 {
        (self.system_counter >> 8) as u8
    }

    pub fn write_div(&mut self) {
        self.system_counter = self.system_counter & 0x00FF;
    }

    pub fn write_tima(&mut self, tima: u8) {
        self.tima = tima;
    }

    pub fn read_time(&self) -> u8 {
        self.tima
    }

    pub fn write_tma(&mut self, tma: u8) {
        self.tma = tma;
    }

    pub fn read_tma(&self) -> u8 {
        self.tma
    }

    pub fn write_tac(&mut self, tac: u8) {
        self.tac = tac;
    }

    pub fn read_tac(&self) -> u8 {
        self.tac
    }
}
