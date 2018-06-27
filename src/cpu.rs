struct Registers {  
    accumulator:        [bool; 16],
    data_bank:          [bool;  8],
    direct:             [bool; 16],
    program_bank:       [bool;  8],
    program_counter:    [bool; 16],
    processor_status:   Flags,
    stack_pointer:      [bool; 16],
    x_index:            [bool; 16],
    y_index:            [bool; 16],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            accumulator:        [false; 16],
            data_bank:          [false;  8],
            direct:             [false; 16],
            program_bank:       [false;  8],
            program_counter:    [false; 16],
            processor_status:   Flags::new(),
            stack_pointer:      [false; 16],
            x_index:            [false; 16],
            y_index:            [false; 16],
        }
    }
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

impl Flags {
    pub fn new() -> Flags {
        Flags {
            break_flag: false,
            carry: false,
            decimal_mode: false,
            emulation_mode: false,
            interrupt_disable: false,
            accumulator_width: false,
            negative: false,
            overflow: false,
            index_width: false,
            zero: false,
        }
    }
}

struct CPU {
    pc: u32,
    counter: u32,
    registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0,
            counter: interrupt_period,
            registers: Registers::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let op_code = memory[self.pc];
            self.pc += 1;

            self.counter -= cycles[op_code];

            match op_code {

            }

            if counter <= 0 {
                // Check for interrupts
                // and cyclic tasks here
                
                self.counter += interrupt_period;
                if should_exit {
                    return
                }
            }
        }
    }

//    The n flag is 0 when the high bit of the result (bit 15 when the m flag is 0, bit 7 when the m flag is 1) is 0, and the n flag is 1 when the high bit of the result is 1.
//    The v flag is 0 when there is not a signed arithmetic overflow, and the v flag is 1 when there is a signed arithmetic overflow.
//              For 8-bit signed numbers, $00 to $7F represents 0 to 127, and $80 to $FF represents -128 to -1;
//              an 8-bit arithmetic overflow occurs when the result is outside the range -128 to 127.
//              For 16-bit signed numbers, $0000 to $7FFF represents 0 to 32767, and $8000 to $FFFF represents -32768 to -1;
//              a 16-bit arithmetic overflow occurs when the result is outside the range -32768 to 32767.
//    The z flag is 0 when the 16-bit (when m flag is 0) or 8-bit (when the m flag is 1) result is nonzero, and the z flag is 1 when the result is zero.
//    The c flag is 0 when there is not an unsigned carry, and the c flag is 1 when there is an unsigned carry.
//              For 8-bit unsigned numbers, $00 to $FF represents 0 to 255; for addition, an 8-bit carry occurs when the result is greater than 255.
//              For 16-bit unsigned numbers, $0000 to $FFFF represents 0 to 65535;
//              for addition, an 16-bit carry occurs when the result is greater than 65535.
//              For subtraction (8-bit or 16-bit), there is a carry when the accumulator is greater than or equal to the data.


    ////////////////////////////////////
    //
    //              ADC
    //
    ////////////////////////////////////

    #[inline]

    fn adc_8(&self, to_add: u8) {
        // accumulator += data + carry;

        if self.registers.processor_status.decimal_mode {
            // bcd mode
            let mut a1: u8  = self.registers.A() & 0x0F;
            let mut a2: u16 = self.registers.A() & 0xF0;

            let to_add1: u8  = to_add & 0x0F;
            let to_add2: u8  = to_add & 0xF0;

            a1 += to_add1 + self.registers.carry_bit();

            if a1 > 0x09 {
                a1 -= 0x0A; // subtract 10
                a1 &= 0x0F; // zero out unnecessary bits
                a2 += 0x10; // add 16 to second digit
            }

            a2 += to_add2;

            if a2 > 0x90 {
                a2 -= 0x0A; // subtract 10
                a2 &= 0xF0; // zero out unnecessary bits
                self.registers.set_carry_bit();
            } else {
                self.registers.clear_carry_bit();
            }

            let result_8 = a1 | a2;

            // TODO -> DETERMINE OVERFLOW

            self.registers.AL = result_8;

            // TODO -> SET ZN FLAGS

        } else {
            // binary mode
            let result_16: u8 = self.registers.A() + to_add + self.registers.carry_bit();

            if result_16 >= 0x100 {
                self.registers.set_carry_bit();
            } else {
                self.registers.clear_carry_bit();
            }

            // TODO -> DETERMINE OVERFLOW

            NEED_TO_DETERMINE_WHICH_REGISTER = result_16 as u8;

            // TODO -> SET ZN FLAGS
        }
    }

    #[inline]
    fn adc_16(&self, to_add: u16) {
        // accumulator += data + carry;

        if self.registers.processor_status.decimal_mode {
            // bcd mode
            let mut a1: u16 = self.registers.A() & 0x000F;
            let mut a2: u16 = self.registers.A() & 0x00F0;
            let mut a3: u16 = self.registers.A() & 0x0F00;
            let mut a4: u32 = self.registers.A() & 0xF000;

            let to_add1: u16 = to_add & 0x000F;
            let to_add2: u16 = to_add & 0x00F0;
            let to_add3: u16 = to_add & 0x0F00;
            let to_add4: u16 = to_add & 0xF000;

            a1 += to_add1 + self.registers.carry_bit();

            if a1 > 0x09 {
                a1 -= 0x000A; // subtract 10
                a1 &= 0x000F; // zero out unnecessary bits
                a2 += 0x0010; // add 16 to second digit
            }

            a2 += to_add2;

            if a2 > 0x90 {
                a2 -= 0x00A0;
                a2 &= 0x00F0; // zero out unnecessary bits
                a3 += 0x0100; // add 16 to third digits
            }

            a3 += to_add3;

            if a3 > 0x0900 {
                a3 -= 0x0A00; // subtract 10
                a3 &= 0x0F00; // zero out uneccessar bits
                a4 += 0x1000; // add 16 to last digit;
            }

            a4 += to_add4;

            if a4 > 0x9000 {
                a4 -= 0xA000;
                a4 &= 0xF000;
                self.registers.set_carry_bit();
            } else {
                self.registers.clear_carry_bit();
            }

            let result_32 = a1 | a2 | a3 | a4;

            // TODO -> DETERMINE OVERFLOW

            self.registers.A = result_32 as u16;

            // TODO -> SET ZN FLAGS

        } else {
            // binary mode

            let result_32: u32 = NEED_TO_DETERMINE_WHICH_REGISTER + to_add + carry_bit();

            if result_32 >= 0x10000 {
                set_carry_bit(1);
            } else {
                set_carry_bit(0);
            }

            // TODO -> DETERMINE OVERFLOW

            self.registers.A = result_32 as u16;

            // TODO -> SET ZN FLAGS
        }
    }

    ////////////////////////////////////
    //
    //              SBC
    //
    ////////////////////////////////////

    fn sbc_8(&self) {
        // accumulator -= data - 1 + carry;

        if self.registers.processor_status.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    fn sbc_16(&self) {
        // accumulator -= data - 1 + carry;

        if self.registers.processor_status.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    ////////////////////////////////////
    //
    //              CMP
    //
    ////////////////////////////////////

    fn cmp_8(&self, to_compare: u8) {

    }

    fn cmp_16(&self, to_compare: u16) {

    }

    ////////////////////////////////////
    //
    //              CPX
    //
    ////////////////////////////////////

    ////////////////////////////////////
    //
    //              CPY
    //
    ////////////////////////////////////
}
