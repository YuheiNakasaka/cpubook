const MOV: u16 = 0;
const ADD: u16 = 1;
const SUB: u16 = 2;
const AND: u16 = 3;
const OR: u16 = 4;
const SL: u16 = 5;
const SR: u16 = 6;
const SRA: u16 = 7;
const LDL: u16 = 8;
const LDH: u16 = 9;
const CMP: u16 = 10;
const JE: u16 = 11;
const JMP: u16 = 12;
const LD: u16 = 13;
const ST: u16 = 14;
const HLT: u16 = 15;
const REG0: u16 = 0;
const REG1: u16 = 1;
const REG2: u16 = 2;
const REG3: u16 = 3;
const REG4: u16 = 4;
const REG5: u16 = 5;
const REG6: u16 = 6;
const REG7: u16 = 7;

pub struct Emulator {
    reg: [u16; 8],
    rom: [u16; 256],
    ram: [u16; 256],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            reg: [0; 8],
            rom: [0; 256],
            ram: [0; 256],
        }
    }

    pub fn run(&mut self) {
        let mut pc: u16 = 0;
        let mut ir: u16 = 0;
        let mut flag_eq: u16 = 0;
        self.assembler();

        while Emulator::op_code(ir) != HLT {
            ir = self.rom[pc as usize];
            println!(
                "{} {} {} {} {} {}",
                pc, ir, self.reg[0], self.reg[1], self.reg[2], self.reg[3]
            );

            pc = pc + 1;

            match Emulator::op_code(ir) {
                MOV => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        self.reg[Emulator::op_reg_b(ir) as usize]
                }
                ADD => {
                    self.reg[Emulator::op_reg_a(ir) as usize] = self.reg
                        [Emulator::op_reg_a(ir) as usize]
                        + self.reg[Emulator::op_reg_b(ir) as usize]
                }
                SUB => {
                    self.reg[Emulator::op_reg_a(ir) as usize] = self.reg
                        [Emulator::op_reg_a(ir) as usize]
                        - self.reg[Emulator::op_reg_b(ir) as usize]
                }
                AND => {
                    self.reg[Emulator::op_reg_a(ir) as usize] = self.reg
                        [Emulator::op_reg_a(ir) as usize]
                        & self.reg[Emulator::op_reg_b(ir) as usize]
                }
                OR => {
                    self.reg[Emulator::op_reg_a(ir) as usize] = self.reg
                        [Emulator::op_reg_a(ir) as usize]
                        | self.reg[Emulator::op_reg_b(ir) as usize]
                }
                SL => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        self.reg[Emulator::op_reg_a(ir) as usize] << 1
                }
                SR => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        self.reg[Emulator::op_reg_a(ir) as usize] >> 1
                }
                SRA => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        (self.reg[Emulator::op_reg_a(ir) as usize] & 0x8000)
                            | (self.reg[Emulator::op_reg_a(ir) as usize] >> 1)
                }
                LDL => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        (self.reg[Emulator::op_reg_a(ir) as usize] & 0xff00)
                            | (Emulator::op_data(ir) & 0x00ff)
                }
                LDH => {
                    self.reg[Emulator::op_reg_a(ir) as usize] = (Emulator::op_data(ir) << 8)
                        | (self.reg[Emulator::op_reg_a(ir) as usize] & 0x00ff)
                }
                CMP => {
                    if self.reg[Emulator::op_reg_a(ir) as usize]
                        == self.reg[Emulator::op_reg_b(ir) as usize]
                    {
                        flag_eq = 1;
                    } else {
                        flag_eq = 0;
                    }
                }
                JE => {
                    if flag_eq == 1 {
                        pc = Emulator::op_addr(ir)
                    }
                }
                JMP => pc = Emulator::op_addr(ir),
                LD => {
                    self.reg[Emulator::op_reg_a(ir) as usize] =
                        self.ram[Emulator::op_addr(ir) as usize]
                }
                ST => {
                    self.ram[Emulator::op_addr(ir) as usize] =
                        self.reg[Emulator::op_reg_a(ir) as usize]
                }
                _ => (),
            }
        }

        println!("ram[64] = {}", self.ram[64]);
    }

    fn assembler(&mut self) {
        self.rom[0] = Emulator::ldh(REG0, 0);
        self.rom[1] = Emulator::ldl(REG0, 0);
        self.rom[2] = Emulator::ldh(REG1, 0);
        self.rom[3] = Emulator::ldl(REG1, 1);
        self.rom[4] = Emulator::ldh(REG2, 0);
        self.rom[5] = Emulator::ldl(REG2, 0);
        self.rom[6] = Emulator::ldh(REG3, 0);
        self.rom[7] = Emulator::ldl(REG3, 10);
        self.rom[8] = Emulator::add(REG2, REG1);
        self.rom[9] = Emulator::add(REG0, REG2);
        self.rom[10] = Emulator::st(REG0, 64);
        self.rom[11] = Emulator::cmp(REG2, REG3);
        self.rom[12] = Emulator::je(14);
        self.rom[13] = Emulator::jmp(8);
        self.rom[14] = Emulator::hlt();
    }

    fn mov(ra: u16, rb: u16) -> u16 {
        return MOV << 11 | ra << 8 | rb << 5;
    }

    fn add(ra: u16, rb: u16) -> u16 {
        return ADD << 11 | ra << 8 | rb << 5;
    }

    fn sub(ra: u16, rb: u16) -> u16 {
        return SUB << 11 | ra << 8 | rb << 5;
    }

    fn and(ra: u16, rb: u16) -> u16 {
        return AND << 11 | ra << 8 | rb << 5;
    }

    fn or(ra: u16, rb: u16) -> u16 {
        return OR << 11 | ra << 8 | rb << 5;
    }

    fn sl(ra: u16) -> u16 {
        return SL << 11 | ra << 8;
    }

    fn sr(ra: u16) -> u16 {
        return SR << 11 | ra << 8;
    }

    fn sra(ra: u16) -> u16 {
        return SRA << 11 | ra << 8;
    }

    fn ldl(ra: u16, ival: u16) -> u16 {
        return LDL << 11 | ra << 8 | (ival & 0x00ff);
    }

    fn ldh(ra: u16, ival: u16) -> u16 {
        return LDH << 11 | ra << 8 | (ival & 0x00ff);
    }

    fn cmp(ra: u16, rb: u16) -> u16 {
        return CMP << 11 | ra << 8 | rb << 5;
    }

    fn je(addr: u16) -> u16 {
        return JE << 11 | (addr & 0x00ff);
    }

    fn jmp(addr: u16) -> u16 {
        return JMP << 11 | (addr & 0x00ff);
    }

    fn ld(ra: u16, addr: u16) -> u16 {
        return LD << 11 | ra << 8 | (addr & 0x00ff);
    }

    fn st(ra: u16, addr: u16) -> u16 {
        return ST << 11 | ra << 8 | (addr & 0x00ff);
    }

    fn hlt() -> u16 {
        return HLT << 11;
    }

    fn op_code(ir: u16) -> u16 {
        return ir >> 11;
    }

    fn op_reg_a(ir: u16) -> u16 {
        return ir >> 8 & 0x0007;
    }

    fn op_reg_b(ir: u16) -> u16 {
        return ir >> 5 & 0x0007;
    }

    fn op_data(ir: u16) -> u16 {
        return ir & 0x00ff;
    }

    fn op_addr(ir: u16) -> u16 {
        return ir & 0x00ff;
    }
}

fn main() {
    let mut emulator = Emulator::new();
    emulator.run();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instructions() {
        // 0 0000 001 010 00000
        assert_eq!(Emulator::mov(1u16, 2u16), 320u16);
        // 0 0001 001 010 00000
        assert_eq!(Emulator::add(1u16, 2u16), 2368u16);
        // 0 0010 001 010 00000
        assert_eq!(Emulator::sub(1u16, 2u16), 4416u16);
        // 0 0011 001 010 00000
        assert_eq!(Emulator::and(1u16, 2u16), 6464u16);
        // 0 0100 001 010 00000
        assert_eq!(Emulator::or(1u16, 2u16), 8512u16);
        // 0 0101 001 00000000
        assert_eq!(Emulator::sl(1u16), 10496u16);
        // 0 0110 001 00000000
        assert_eq!(Emulator::sr(1u16), 12544u16);
        // 0 0111 001 00000000
        assert_eq!(Emulator::sra(1u16), 14592u16);
        // 0 1000 001 00000101
        assert_eq!(Emulator::ldl(1u16, 5u16), 16645u16);
        // 0 1001 001 00000101
        assert_eq!(Emulator::ldh(1u16, 5u16), 18693u16);
        // 0 1010 001 00000101
        assert_eq!(Emulator::cmp(1u16, 5u16), 20896u16);
        // 0 1011 000 00000101
        assert_eq!(Emulator::je(5u16), 22533u16);
        // 0 1100 000 00000101
        assert_eq!(Emulator::jmp(5u16), 24581u16);
        // 0 1101 001 00000101
        assert_eq!(Emulator::ld(1u16, 5u16), 26885u16);
        // 0 1110 101 00000001
        assert_eq!(Emulator::st(5u16, 1u16), 29953u16);
        // 0 1111 101 00000001
        assert_eq!(Emulator::hlt(), 30720u16);
    }

    #[test]
    fn test_operations() {
        // 0 1000 00000000000
        assert_eq!(Emulator::op_code(32768u16), 16u16);
        // 0 0000 100 00000000
        assert_eq!(Emulator::op_reg_a(1024u16), 4u16);
        // 0 0000 000 100 00000
        assert_eq!(Emulator::op_reg_b(128u16), 4u16);
        // 0 0000 001 10000000
        assert_eq!(Emulator::op_data(384u16), 128u16);
        // 0 0000 001 00000001
        assert_eq!(Emulator::op_addr(257u16), 1u16);
    }
}
