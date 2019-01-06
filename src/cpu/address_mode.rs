use cpu::cpu::{CPU};
use cpu::memory::Mem;

const JMP_JSR_OPCODES: [u8; 7] = [0x4C, 0x5C, 0x6C, 0x7C, 0xDC, 0x20, 0xFC];
const PEI_OPCODE: u8 = 0xD4;

type MemoryAddress = (u8, u16);

// address mode mappings
// =====================
// (dir, X)   => DirectIndexedIndirect
// stk, S     => StackRelative
// dir        => Direct
// [dir]      => DirectIndirectLong
// imm        => Immediate
// abs        => Absolute
// (dir), Y   => DirectIndirectIndexed
// (dir)      => DirectIndirect
// (stk,S), Y => StackRelativeIndirectIndexed
// dir, X     => DirectIndexedX
// [dir], Y   => DirectIndirectIndexedLong
// abs, Y     => AbsoluteIndexedY
// abs, X     => AbsoluteIndexedX
// long       => AbsoluteLong
// long, X    => AbsoluteLongIndexedX
// (abs)      => AbsoluteIndirect
// [abs]      => AbsoluteIndirectLong
// (abs,X)    => AbsoluteIndexedIndirect

#[derive(PartialEq)]
pub enum AddressMode {
    Accumulator,
    Immediate,
    Implied,
    Relative8,
    Relative16,
    Absolute(u8),
    Direct(u8),
    DirectIndexedX,
    DirectIndexedY,
    DirectIndexedIndirect,
    DirectIndirect,
    DirectIndirectLong,
    DirectIndirectIndexed,
    DirectIndirectIndexedLong,
    StackRelative,
    StackRelativeIndirectIndexed,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    AbsoluteLong,
    AbsoluteLongIndexedX,
    AbsoluteIndirect,
    AbsoluteIndexedIndirect,
    AbsoluteIndirectLong,
    SourceDestination
}

impl AddressMode {
    fn absolute(cpu: &mut CPU, opcode: &u8) -> MemoryAddress {
        let lo = cpu.next_b() as u16;
        let hi = cpu.next_b() as u16;

        let bank = if JMP_JSR_OPCODES.contains(opcode) {
            cpu.pbr()
        } else {
            cpu.dbr()
        };

        let address = (hi << 8) | lo;
        (bank, address)
    }

    fn absolute_indexed(cpu: &mut CPU, index: u16) -> MemoryAddress {
        let lo = cpu.next_b() as u16;
        let hi = cpu.next_b() as u16;

        let addr = ((hi << 8) | lo).wrapping_add(index);
        (cpu.dbr(), addr)
    }

    fn absolute_indirect(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let p_lo = cpu.next_b() as u16;
        let p_hi = cpu.next_b() as u16;

        let p_base_addr = (p_hi << 8) | p_lo;
        let lo = mem.load(0, p_base_addr) as u16;
        let hi = mem.load(0, p_base_addr.wrapping_add(1)) as u16;

        (cpu.pbr(), (hi << 8 | lo))
    }

    fn absolute_indirect_long(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let p_lo = cpu.next_b() as u16;
        let p_hi = cpu.next_b() as u16;

        let p_base_addr = (p_hi << 8) | p_lo;
        let lo = mem.load(0, p_base_addr) as u16;
        let md = mem.load(0, p_base_addr.wrapping_add(1)) as u16;
        let hi = mem.load(0, p_base_addr.wrapping_add(2));

        (hi, (md << 8 | lo))
    }

    fn absolute_indexed_indirect(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let p_lo = cpu.next_b() as u32;
        let p_hi = cpu.next_b() as u32;

        let addr_pointer = (hi << 8 | lo).wrapping_add(cpu.x());
        let lo = mem.load(cpu.pbr(), addr_pointer) as u16;
        let hi = mem.load(cpu.pbr(), addr_pointer.wrapping_add(1)) as u16;

        (cpu.pbr(), hi << 8 | lo)
    }

    fn direct(cpu: &mut CPU, opcode: &u8) -> MemoryAddress {
        let lo = cpu.next_b();

        if opcode != &PEI_OPCODE && cpu.is_emulation() && cpu.dl() == 0x00 {
            let addr = (cpu.dh() as u16) << 8 | lo as u16;
            (0, addr)
        } else {
            (0, (cpu.d().wrapping_add(lo as u16)))
        }
    }

    fn direct_indexed(cpu: &mut CPU, index: u16) -> MemoryAddress {
        let lo = cpu.next_b();

        if cpu.is_emulation() && cpu.dl() == 0x00 {
            let addr = (cpu.dh() as u16) << 8 | lo.wrapping_add(index);
            (0, addr)
        } else {
            (0, cpu.d().wrapping_add(lo).wrapping_add(index))
        }
    }

    fn direct_indirect(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let lo = cpu.next_b() as u16;

        let (pointer_lo, pointer_hi) = if cpu.is_emulation() && cpu.dl() == 0 {
            let lp = ((cpu.dh() as u16) << 8) | lo;
            let hp = ((cpu.dh() as u16) << 8) | (lo.wrapping_add(1));
            (lp, hp)
        } else {
            let lp = cpu.d().wrapping_add(lo);
            (lp, lp.wrapping_add(1))
        };

        let data_lo = mem.load(0, pointer_lo);
        let data_hi = mem.load(0, pointer_hi);

        let data_addr = (data_hi as u16) << 8 | data_lo as u16;
        (cpu.dbr(), data_addr)
    }

    fn direct_indirect_long(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let lo = cpu.next_b();

        let pointer_lo = cpu.dbr().wrapping_add(lo);
        let pointer_md = pointer_lo.wrapping_add(1);
        let pointer_hi = pointer_md.wrapping_add(1);

        let data_lo = mem.load(0, pointer_lo);
        let data_md = mem.load(0, pointer_md);
        let data_hi = mem.load(0, pointer_hi);

        (data_hi, (data_md as u16) << 8 | data_lo as u16)
    }

    fn direct_indexed_indirect(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let lo = cpu.next_b();

        let (pointer_lo, pointer_hi) = if cpu.is_emulation() && cpu.dl() == 0x00 {
            let mid = (cpu.dh() as u16) << 8;
            let lp = mid | lo.wrapping_add(cpu.x());
            let hp = mid | lo.wrapping_add(cpu.x()).wrapping_add(1);
            (lp, hp)
        } else {
            let lp = cpu.d().wrapping_add(lo).wrapping_add(cpu.x());
            let hp = lp.wrapping_add(1);
            (lp, hp)
        };

        let data_lo = mem.load(0, pointer_lo);
        let data_hi = mem.load(0, pointer_hi);

        let final_addr = (data_hi as u16) << 8 | data_lo as u16;
        (cpu.dbr(), final_addr)
    }

    fn direct_indirect_indexed(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let addr = Self::direct_indirect(cpu, mem);
        Self::add_index_to_address(addr, cpu.y())
    }

    fn direct_indirect_indexed_long(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let addr = Self::direct_indirect_long(cpu, mem);
        Self::add_index_to_address(addr, cpu.y());
    }

    fn immediate(cpu: &mut CPU) -> MemoryAddress {
        // TODO -> maybe handle the 4 cases listed online but I think the instructions handle those

        cpu.increment_pc();
        (cpu.pbr(), cpu.pc())
    }

    fn absolute_long(cpu: &mut CPU) -> MemoryAddress {
        let lo = cpu.next_b();
        let md = cpu.next_b();
        let hi = cpu.next_b();

        (hi, (md as u16) << 8 | (lo as u16))
    }

    fn absolute_long_indexed(cpu: &mut CPU) -> MemoryAddress {
        let address = Self::absolute_long(cpu);
        Self::add_index_to_address(address, cpu.x())
    }

    fn relative_8(cpu: &mut CPU) -> MemoryAddress {
        let lo = cpu.next_b();

        if lo <= 0x7F {
            (cpu.pbr(), cpu.pc().wrapping_add(2).wrapping_add(lo))
        } else {
            (cpu.pbr(), cpu.pc().wrapping_sub(254).wrapping_add(lo))
        }
    }

    fn relative_16(cpu: &mut CPU) -> MemoryAddress {
        let lo = cpu.next_b();
        let hi = cpu.next_b();

        let displacement = (hi as u16) << 8 | (lo as u16);
        (cpu.pbr(), cpu.pc().wrapping_add(3).wrapping_add(displacement))
    }

    fn stack_relative(cpu: &mut CPU) -> MemoryAddress {
        let lo = cpu.next_b();
        (0, lo.wrapping_add(cpu.sp()))
    }

    fn stack_relative_indirect_indexed(cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        let lo = cpu.next_b();

        let pointer_lo = lo.wrapping_add(cpu.sp());
        let pointer_hi = pointer_lo.wrapping_add(1);

        let addr_lo = mem.load(0, pointer_lo) as u16;
        let addr_hi = mem.load(0, pointer_hi) as u16;

        let address = (cpu.dbr(), addr_hi << 8 | addr_lo);
        Self::add_index_to_address(address, cpu.y())
    }

    fn add_index_to_address(address: MemoryAddress, index: u16) -> MemoryAddress {
        let (base_bank, base_addr) = address;

        if (base_addr as u32) + (index as u32) > 0xFFFF {
            (base_bank.wrapping_add(1), index - 1)
        } else {
            (base_bank, base_addr + index)
        }
    }

    pub fn get_address_8(&self, cpu: &mut CPU, mem: &Box<dyn Mem>) -> MemoryAddress {
        return match self {
            Self::Accumulator => {
                panic!("trying to load with accumulator addressing mode");
            },
            Self::Immediate => {
                Self::immediate(cpu)
            },
            Self::Implied => {
                panic!("trying to generate address with implied addressing");
            },
            Self::Relative8 => {
                Self::relative_8(cpu)
            },
            Self::Relative16 => {
                Self::relative_16(cpu)
            },
            Self::Absolute(opcode) => {
                Self::absolute(cpu, opcode)
            },
            Self::Direct(opcode) => {
                Self::direct(cpu, opcode)
            },
            Self::DirectIndexedX => {
                Self::direct_indexed(cpu, cpu.x())
            },
            Self::DirectIndexedY => {
                Self::direct_indexed(cpu, cpu.y())
            },
            Self::DirectIndexedIndirect => {
                Self::direct_indexed_indirect(cpu, mem)
            }
            Self::DirectIndirect => {
                Self::direct_indirect(cpu, mem)
            },
            Self::DirectIndirectLong => {
                Self::direct_indirect_long(cpu, mem)
            },
            Self::DirectIndirectIndexed => {
                Self::direct_indirect_indexed(cpu, mem)
            },
            Self::DirectIndirectIndexedLong => {
                Self::direct_indirect_indexed_long(cpu, mem)
            },
            Self::AbsoluteIndexedX => {
                Self::absolute_indexed(cpu, cpu.x())
            },
            Self::AbsoluteIndexedY => {
                Self::absolute_indexed(cpu, cpu.y())
            },
            Self::StackRelative => {
                Self::stack_relative(cpu)
            },
            Self::StackRelativeIndirectIndexed => {
                Self::stack_relative_indirect_indexed(cpu, mem)
            },
            Self::AbsoluteLong => {
                Self::absolute_long(cpu)
            },
            Self::AbsoluteLongIndexedX => {
                Self::absolute_long_indexed(cpu)
            },
            Self::AbsoluteIndirect => {
                Self::absolute_indirect(cpu, mem)
            },
            Self::AbsoluteIndirectLong => {
                Self::absolute_indirect_long(cpu, mem)
            },
            Self::AbsoluteIndexedIndirect => {
                Self::absolute_indexed_indirect(cpu, mem)
            },
            Self::SourceDestination => {
                panice!("trying to get a single address from SourceDestination addressing");
            }
        };
    }

    fn increment_addr_with_bank_wrapping(addr: MemoryAddress) -> MemoryAddress {
        if addr.1 == 0xFFFF {
            (addr.0.wrapping_add(1), 0x0000)
        } else {
            (addr.0, addr.1 + 1)
        }
    }

    fn increment_addr_with_page_wrapping(addr: MemoryAddress) -> MemoryAddress {
        (addr.0, addr.1.wrapping_add(1))
    }

    pub fn get_address_16(&self, cpu: &mut CPU, mem: &Box<dyn Mem>) -> (MemoryAddress, MemoryAddress) {
        match self {
            AddressMode::Absolute(opcode) => {
                if JMP_JSR_OPCODES.contains(opcode) {
                    panic!("trying to get 16 bit address for jmp/jsr opcodes")
                }

                let lo = Self::absolute(cpu, opcode);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::AbsoluteIndexedX => {
                let lo = Self::absolute_indexed(cpu, cpu.x());
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::AbsoluteIndexedY => {
                let lo = Self::absolute_indexed(cpu, cpu.y());
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            }
            AddressMode::AbsoluteIndirect => {
                panic!("trying to get two addresses with AbsoluteIndirect addressing mode");

                // let lo = Self::absolute_indirect(cpu, mem);
                // let hi = Self::increment_addr_with_page_wrapping(lo);
                // (lo, hi)
            },
            AddressMode::AbsoluteIndirectLong => {
                panic!("trying to get two addresses with AbsoluteIndirectLong addressing mode");

                // let lo = Self::absolute_indirect(cpu, mem);
                // let hi = Self::increment_addr_with_page_wrapping(lo);
                // (lo, hi)
            },
            AddressMode::AbsoluteIndexedIndirect => {
                panic!("trying to get two addresses with AbsoluteIndexedIndirect addressing mode");

                // let lo = Self::absolute_indexed_indirect(cpu, mem);
                // let hi = Self::increment_addr_with_page_wrapping(lo);
                // (lo, hi)
            }
            AddressMode::Accumulator => {
                panic!("trying to get address from accumulator addressing mode");
            },
            AddressMode::Direct(opcode) => {
                // todo -> remove this if check after some testing to make sure we don't panic
                if opcode != &PEI_OPCODE && cpu.is_emulation() && cpu.dl() == 0x00 {
                    panic!("trying to get 2 two addresses with Direct addressing but CPU conditions only allow 1")
                }

                let lo = Self::direct(cpu, opcode);
                let hi = increment_addr_with_page_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndexedX => {
                let lo = Self::direct_indexed(cpu, cpu.x());
                let hi = Self::increment_addr_with_page_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndexedY => {
                let lo = Self::direct_indexed(cpu, cpu.y());
                let hi = Self::increment_addr_with_page_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndirect => {
                let lo = Self::direct_indirect(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndirectLong => {
                let lo = Self::direct_indirect_long(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndexedIndirect => {
                let lo = Self::direct_indexed_indirect(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndirectIndexed => {
                let lo = Self::direct_indirect_indexed(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::DirectIndirectIndexedLong => {
                let lo = Self::direct_indirect_indexed_long(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::Immediate => {
                let lo = Self::immediate(cpu);
                let hi = Self::immediate(cpu);
                (lo, hi)
            },
            AddressMode::Implied => {
                panic!("trying to generate two addresses with implied addressing");
            },
            AddressMode::AbsoluteLong => {
                let lo = Self::absolute_long(cpu);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::AbsoluteLongIndexedX => {
                let lo = Self::absolute_long_indexed(cpu);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            },
            AddressMode::Relative8 => {
                panic!("attempting to get two addresses from Relative8 addressing");
            },
            AddressMode::Relative16 => {
                panic!("attempting to get two addresses from Relative16 addressing");
            },
            AddressMode::SourceDestination => {
                let tt = cpu.next_b();
                let ss = cpu.next_b();

                let source = (ss, cpu.x());
                let dest = (tt, cpu.y());

                (source, dest)
            },
            AddressMode::StackRelative => {
                let lo = Self::stack_relative(cpu);
                let hi = Self::increment_addr_with_page_wrapping(lo);
                (lo, hi)
            },
            AddressMode::StackRelativeIndirectIndexed => {
                let lo = Self::stack_relative_indirect_indexed(cpu, mem);
                let hi = Self::increment_addr_with_bank_wrapping(lo);
                (lo, hi)
            }
        }
    }
}