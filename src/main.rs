mod viz_arch_dependent;
use viz_arch_dependent::arch_dependent;

struct VirtualMachine {
    reg_a:u16,
    reg_b:u16,
    reg_c:u16,
    reg_x:u16,
    reg_y:u16,
    reg_z:u16,
    reg_f:u16,
    reg_h:u16,
    pc:u16,
    sp:u8,
    mem:[u16; 0x10000]
    // A B X Y Z F H PC
}
impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_x: 0,
            reg_y: 0,
            reg_z: 0,
            reg_f: 0,
            reg_h: 0,
            pc: 0,
            sp: 0xff,
            mem: [0 as u16; 0x10000]
        }
    }
    pub fn get_param(&mut self, number:u8, fixed:u16) -> Option<&mut u16> {
        match number {
            0 => Some(&mut self.reg_a),
            1 => Some(&mut self.reg_b),
            2 => Some(&mut self.reg_x),
            3 => Some(&mut self.reg_y),
            4 => Some(&mut self.reg_z),
            5 => Some(&mut self.reg_f),
            6 => Some(&mut self.reg_h),
            7 => Some(&mut self.pc),
            8 => None,
            9 => Some(&mut self.reg_c),
            10 => Some(&mut self.mem[self.reg_a as usize]),
            11 => Some(&mut self.mem[self.reg_b as usize]),
            12 => Some(&mut self.mem[self.reg_z as usize]),
            13 => Some(&mut self.mem[self.reg_h as usize]),
            14 => Some(&mut self.mem[fixed as usize]),
            15 => Some(&mut self.mem[self.reg_c as usize]),

            _ => None
        }
    }
    pub fn get_readonly_param(&self, number:u8, fixed:u16) -> Option<u16> {
        match number {
            0 => Some(self.reg_a),
            1 => Some(self.reg_b),
            2 => Some(self.reg_x),
            3 => Some(self.reg_y),
            4 => Some(self.reg_z),
            5 => Some(self.reg_f),
            6 => Some(self.reg_h),
            7 => Some(self.pc),
            8 => Some(fixed),
            9 => Some(self.reg_c),
            10 => Some(self.mem[self.reg_a as usize]),
            11 => Some(self.mem[self.reg_b as usize]),
            12 => Some(self.mem[self.reg_z as usize]),
            13 => Some(self.mem[self.reg_h as usize]),
            14 => Some(self.mem[fixed as usize]),
            15 => Some(self.mem[self.reg_c as usize]),

            _ => None
        }
    }
    /**
    Runs a single operation cycle.
    Returns true if the machine has halted
    */
    pub fn do_iteration(&mut self) -> bool {
        let opcode:u16 = self.mem[self.pc as usize];
        let addressing_mode = ((self.mem[(self.pc + 1) as usize] & 0xFF00) >> 8) as u8;
        let registers = (self.mem[(self.pc + 1) as usize] & 0xFF) as u8;
        let fixed0:u16 = self.mem[(self.pc + 2) as usize];
        let fixed1:u16 = self.mem[(self.pc + 3) as usize];
        let mut null:u16 = 0;
        let in0_pre = match self.get_readonly_param(registers >> 4, fixed0) {
            Some(val) => val,
            None => null
        };
        let in1_pre = match self.get_readonly_param(registers & 0x0F, fixed1) {
            Some(val) => val,
            None => null
        };
        let in0 = match addressing_mode {
            0 => in0_pre,
            1 => in0_pre,
            2 => in0_pre,
            3 => in0_pre + self.reg_y,
            4 => in0_pre + self.reg_x,
            _ => in0_pre
        };
        let in1 = match addressing_mode {
            0 => in1_pre,
            1 => in1_pre + self.reg_x,
            2 => in1_pre + self.reg_y,
            3 => in1_pre + self.reg_x,
            4 => in1_pre + self.reg_y,
            _ => in1_pre
        };
        //println!("Running iteration opcode {} pc {} in0 {} in1 {} regs {}", opcode, self.pc, in0, in1, registers);
        let mut increment = true;
        match opcode {
            //NOP
            0 => {

            },
            //ADD
            1 => {
                let out0 = match self.get_param(registers >> 4, fixed0) {
                    Some(val) => val,
                    None => &mut null
                };
                *out0 = in0.wrapping_add(in1);
            },
            //SUB
            2 => {
                let out0 = match self.get_param(registers >> 4, fixed0) {
                    Some(val) => val,
                    None => &mut null
                };
                *out0 = in0.wrapping_sub(in1);
            },
            //MUL
            3 => {
                let out0 = match self.get_param(registers >> 4, fixed0) {
                    Some(val) => val,
                    None => &mut null
                };
                *out0 = in0.wrapping_mul(in1);
            },
            //DIV
            4 => {
                if in1 == 0 {
                    return true;
                } else {
                    let out0 = match self.get_param(registers >> 4, fixed0) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out0 = in0.wrapping_div(in1);
                }
            },
            //CMP
            5 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                if in0 > in1 {
                    *out1 = 0xffff;
                } else if in0 == in1 {
                    *out1 = 0;
                } else {
                    *out1 = 1;
                }
            },
            //JMP
            6 => {
                increment = false;
                self.pc = in1;
            },
            //GFX (Undefined)
            7 => {

            },
            //AND
            8 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in0 & in1;
            },
            //NOT
            9 => {
                {
                    let out0 = match self.get_param(registers >> 4, fixed0) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out0 = !in0;
                }
                {
                    let out1 = match self.get_param(registers & 0x0F, fixed1) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out1 = !in1;
                }
            },
            //OOR
            10 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in0 | in1;
            },
            //XOR
            11 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in0 ^ in1;
            },
            //INP
            12 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = arch_dependent::io_in(in0);
            },
            //OUT
            13 => {
                arch_dependent::io_out(in1, in0);
            },
            //RSH
            14 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in1 >> in0;
            },
            //LSH
            15 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in1 << in0;
            },
            //SET
            16 => {
                self.mem[in0 as usize] = in1;
            },
            //GET
            17 => {
                let res = self.mem[in0 as usize];
                {
                    let out1 = match self.get_param(registers & 0x0F, fixed1) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out1 = res;
                }
            },
            //JOZ
            18 => {
                if in0 == 0 {
                    self.pc = in1;
                    increment = false;
                }
            },
            //RND
            19 => {
                {
                    let out0 = match self.get_param(registers >> 4, fixed0) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out0 = arch_dependent::entropy();
                }
                {
                    let out1 = match self.get_param(registers & 0x0F, fixed1) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out1 = arch_dependent::entropy();
                }
            },
            //MOV
            20 => {
                let out1 = match self.get_param(registers & 0x0F, fixed1) {
                    Some(val) => val,
                    None => &mut null
                };
                *out1 = in0;
            },
            //PSH
            21 => {
                self.mem[(0xFF00 | (self.sp as u16)) as usize] = in0;
                self.sp = self.sp.wrapping_sub(1);
            },
            //POP
            22 => {
                self.sp += 1;
                let res = self.mem[(0xFF00 | (self.sp as u16)) as usize];
                {
                    let out1 = match self.get_param(registers & 0x0F, fixed1) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out1 = res;
                }
            },
            //MOD
            23 => {
                if in1 == 0 {
                    return true;
                } else {
                    let out0 = match self.get_param(registers >> 4, fixed0) {
                        Some(val) => val,
                        None => &mut null
                    };
                    *out0 = in0 % in1;
                }
            },
            //HLT
            24 => {
                return true;
            },
            //JNZ
            25 => {
                if in0 != 0 {
                    increment = false;
                    self.pc = in1;
                }
            },
            //POW
            26 => {
                let out0 = match self.get_param(registers >> 4, fixed0) {
                    Some(val) => val,
                    None => &mut null
                };
                *out0 = arch_dependent::pow16(in0, in1)
            },
            //CAL
            27 => {
                self.mem[(0xFF00 | (self.sp as u16)) as usize] = self.pc + 4;
                self.sp = self.sp.wrapping_sub(1);
                self.pc = in1;
                increment = false;
            },
            //RET
            28 => {
                self.sp = self.sp.wrapping_add(1);
                self.pc = self.mem[(0xFF00 | (self.sp as u16)) as usize];
                increment = false;
            },
            _ => {}
        };
        if increment {
            self.pc = self.pc.wrapping_add(4);
        }
        return false;
    }
    /**
    Runs the machine to completion
    */
    pub fn run(&mut self) {
        while !self.do_iteration() {

        }
    }
}
fn main() {
    arch_dependent::license_notice();
    let mut machine = VirtualMachine::new();
    arch_dependent::load_program(&mut machine.mem);
    machine.run();
}
