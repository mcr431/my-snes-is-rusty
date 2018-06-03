struct CPU {
    pc: u32,
    counter: u32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0,
            counter: interrupt_period,
        }
    }

    pub fn run(&mut self) {
        loop {
            let op_code = memory[self.pc];
            self.pc += 1;

            self.counter -= cycles[self.op_code];

            match op_code {
                
            }

            if (counter <= 0) {
                // Check for interrupts
                // and cyclic tasks here
                
                self.counter += interrupt_period;
                if should_exit {
                    return
                }
            }
        }
    }
}
