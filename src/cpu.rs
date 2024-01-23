use std::process::exit;
use rand::random;
use crate::consts::{CHIP8_HEIGHT, CHIP8_WIDTH, PROGRAM_START, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::vm::Vm;

pub struct Processor {
    pub reg: [u8; 16],
    pub sp: u16,
    pub pc: u16,
    pub i_reg: u16,
    pub stack: [u16; 16],
    pub dt: u8,
    pub st: u8,
}

impl Processor {
    pub fn new() -> Self {
        return Self {
            reg: [0; 16],
            sp: 0,
            pc: PROGRAM_START,
            i_reg: 0,
            stack: [0; 16],
            dt: 0,
            st: 0,
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn skip_instruction(&mut self) {
        self.pc += 4;
    }

    pub(crate) fn display_op(&mut self, nn: u8, vm: &Vm) {
        let mut mem = vm.memory.borrow_mut();
        match nn {
            0x0000 => { //CLS
                mem.clear_vram();
                mem.toggle_vram_mod();
            }

            0x000E => { //RET
                self.sp -= 1;
                self.pc = self.stack[self.pc as usize];
            }

            _ => {
                println!("Unknown OpCode: {}", nn);
                exit(1);
            }
        }
    }

    pub(crate) fn jump_to_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub(crate) fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    pub(crate) fn skip_execution(&mut self, inst: u16, x: u8, nn: u8) {
        match inst {
            0x3 => {
                if self.reg[x as usize] == nn {
                    self.increment_pc();
                }
            }

            0x4 => {
                if self.reg[x as usize] != nn {
                    self.increment_pc();
                }
            }

            _ => {
                println!("Unknown OpCode: {}", inst);
                exit(1);
            }
        }
    }

    pub(crate) fn reg_check_skip_exec(&mut self, inst: u16, x: u8, y: u8) {
        match inst {
            0x5 => {
                if self.reg[x as usize] == self.reg[y as usize] {
                    self.increment_pc();
                }
            }

            0x9 => {
                if self.reg[x as usize] != self.reg[y as usize] {
                    self.increment_pc();
                }
            }

            _ => {
                println!("Unknown OpCode: {}", inst);
                exit(1);
            }
        }
    }

    pub(crate) fn set_reg_nn(&mut self, x: u8, nn:u8) {
        self.reg[x as usize] = nn;
    }

    pub(crate) fn add_reg_nn(&mut self, x: u8, nn: u8) {
        self.reg[x as usize] = self.reg[x as usize].wrapping_add(nn);
    }

    pub(crate) fn alu_op(&mut self, x: u8, y: u8, n: u8) {
        match n {
            0x0000 => {
                self.reg[x as usize] = self.reg[y as usize];
            }

            0x0001 => {
                self.reg[x as usize] |= self.reg[y as usize];
            }

            0x0002 => {
                self.reg[x as usize] &= self.reg[y as usize];
            }

            0x0003 => {
                self.reg[x as usize] ^= self.reg[y as usize];
            }

            0x0004 => {
                self.reg[x as usize] += self.reg[y as usize];
                if self.reg[x as usize] > 0xFF {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
            }

            0x0005 => {
                let diff: i8 = self.reg[x as usize] as i8 - self.reg[y as usize] as i8;
                self.reg[x as usize] = diff as u8;
                if diff < 0 {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
            }

            0x0006 => {
                self.reg[0xF] = self.reg[x as usize] & 0x1;
                self.reg[x as usize] >>= 1;
            }

            0x0007 => {
                let diff: i8 = self.reg[y as usize] as i8 - self.reg[x as usize] as i8;
                self.reg[x as usize] = diff as u8;
                if diff < 0 {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
            }

            0x000E => {
                self.reg[0xF] = self.reg[x as usize] >> 7;
                self.reg[x as usize] <<= 1;
            }

            _ => {
                println!("Unknown OpCode: {}", n);
                exit(1);
            }
        }
    }

    pub(crate) fn set_i_to_addr(&mut self, addr: u16) {
        self.i_reg = addr;
    }

    pub(crate) fn jump_to_addr_plus_offset(&mut self, addr: u16) {
        self.pc = addr + self.reg[0] as u16;
    }

    pub(crate) fn set_reg_rand(&mut self, x: u8, nn: u8) {
        let rng: u8 = random();
        self.reg[x as usize] = rng & nn;
    }

    pub(crate) fn draw_op(&mut self, x: u8, y: u8, n: u8, vm: &Vm) {
        let mut mem = vm.memory.borrow_mut();
        let x_coord = self.reg[x as usize] as u16;
        let y_coord = self.reg[y as usize] as u16;
        let rows = n;
        //println!("args: x: {}, y: {}, n: {}\n", x, y ,n);
        //println!("X reg: {}, Y reg: {}", x_coord, y_coord);
        let mut toggle = false;

        for y_line in 0..rows {
            let addr = self.i_reg + y_line as u16;
            let pixels = mem.ram[addr as usize];
            //println!("addr: {}\npixels: {:b}", addr, pixels);

            for x_line in 0..8 {
                if ((pixels >> (7 - x_line)) & 0x1) != 0 {
                    let x_val = (x_coord + x_line) as usize % CHIP8_WIDTH;
                    let y_val = (y_coord + y_line as u16) as usize % CHIP8_HEIGHT;

                    let idx = x_val + CHIP8_WIDTH * y_val;
                    toggle |= mem.vram[idx];
                    mem.vram[idx] ^= true;
                }
            }
        }

        if toggle {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }

        mem.toggle_vram_mod();
    }

    pub(crate) fn key_op(&mut self, x: u8, nn: u8, vm: &Vm) {
        let keys = vm.keyboard.borrow();
        match nn {
            0x009E => {
                let reg = self.reg[x as usize];
                let key = keys.keys[reg as usize];
                if key {
                    self.increment_pc();
                }
            }

            0x00A1 => {
                let reg = self.reg[x as usize];
                let key = keys.keys[reg as usize];
                if !key {
                    self.increment_pc();
                }
            }

            _ => {
                println!("Unknown OpCode: {}", nn);
                exit(1);
            }
        }
    }

    pub(crate) fn timer_op(&mut self, x: u8, nn: u8, vm: &Vm)  {
        match nn {
            0x0007 => {
                self.reg[x as usize] = self.dt;
            }

            0x000A => {
                let keys = vm.keyboard.borrow();
                let mut pressed = false;
                for i in 0..16 {
                    if keys.is_key_pressed(i) {
                        self.reg[x as usize] = i as u8;
                        pressed = true;
                    }
                }

                if pressed == false {
                    return;
                }
            }

            0x0015 => {
                self.dt = self.reg[x as usize];
            }

            0x0018 => {
                self.st =self.reg[x as usize];
            }

            0x001E => {
                if (self.i_reg + self.reg[x as usize] as u16 > 0xFFF) {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }

                self.i_reg += self.reg[x as usize] as u16;
            }

            0x0029 => {
                self.i_reg = (self.reg[x as usize] * 0x5) as u16;
            }

            0x0033 => {
                let mut mem = vm.memory.borrow_mut();
                mem.write_ram(self.i_reg, self.reg[x as usize] / 100);
                mem.write_ram(self.i_reg + 1, (self.reg[x as usize] / 10) % 10);
                mem.write_ram(self.i_reg + 2, self.reg[x as usize] % 10);
            }

            0x0055 => {
                let mut mem = vm.memory.borrow_mut();
                for idx in 0..x {
                    mem.write_ram(self.i_reg + idx as u16, self.reg[idx as usize]);
                }
                self.i_reg += x as u16 + 1;
                self.increment_pc();
            }

            0x0065 => {
                let mut mem = vm.memory.borrow_mut();
                for idx in 0..x {
                    self.reg[idx as usize] = mem.read_ram(self.i_reg + idx as u16);
                }
                self.i_reg += x as u16 + 1;
            }

            _ => {
                println!("Unknown OpCode: {}", nn);
                exit(1);
            }
        }
    }
}