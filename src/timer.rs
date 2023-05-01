use crate::cpu::Mem;

pub struct Timer {
    divider: u8,
    counter: u8,
    modulo: u8,
    timer_ctrl: u8,
    timer_enabled: bool,
    clock_freq: u32,
    div_internal: u32,
    timer_internal: u32,
    pub interrupt: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            divider: 0,
            counter: 0,
            modulo: 0,
            timer_ctrl: 0,
            timer_enabled: false,
            clock_freq: 0,
            div_internal: 0,
            timer_internal: 0,
            interrupt: 0,
        }
    }

    pub fn execute_cycle(&mut self, time: u32) {
        self.div_internal += time;
        while self.div_internal >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.div_internal -= 256;
        }

        if !self.timer_enabled {
            return;
        }

        while self.timer_internal >= self.clock_freq {
            self.counter = self.counter.wrapping_add(1);
            if self.counter == 0 {
                self.counter = self.modulo;
                self.interrupt |= 0b0000_0100;
            }
            self.timer_internal -= self.clock_freq;
        }
    }

    fn extract_timer_ctrl_reg(&mut self) {
        self.timer_enabled = (self.timer_ctrl & 0b0000_0100) == 0;
        self.clock_freq = match self.timer_ctrl & 0b0000_0011 {
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => 1024,
        }
    }
}

impl Mem for Timer {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => self.timer_ctrl,
            _ => panic!("Timer can't read {addr:4x}"),
        }
    }

    fn mem_write_u8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF04 => self.divider = data,
            0xFF05 => self.counter = data,
            0xFF06 => self.modulo = data,
            0xFF07 => {
                self.timer_ctrl = data;
                self.extract_timer_ctrl_reg();
            }
            _ => panic!("Timer can't read {addr:4x}"),
        }
    }
}
