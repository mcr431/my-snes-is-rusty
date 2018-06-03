struct Registers {  
    accumulator:        [bool; 16],
    data_bank:          [bool;  8],
    direct:             [bool; 16],
    program_bank:       [bool;  8],
    program_counter:    [bool; 16],
    processor_status:   [bool;  8],
    stack_pointer:      [bool; 16],
    x_index:            [bool; 16],
    y_index:            [bool; 16],
}

struct Flags {
    break_flag:         bool,
    carry:              bool,
    decimal_mode:       bool,
    emulation_mode:     bool,
    interrupt_disable:  bool,
    accumulator_width:  bool,
    negative:           bool,
    overflow:           bool,
    index_width:        bool,
    zero:               bool,
}

struct CPU {
    pc: u32,
    counter: u32,
    registers: Registers,
    flags: Flags,
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
                OpCode::
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
