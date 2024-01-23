use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use crate::cpu::Processor;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

pub struct Vm {
    pub cpu: Rc<RefCell<Processor>>,
    pub memory: Rc<RefCell<Memory>>,
    pub keyboard: Rc<RefCell<Keyboard>>,
    pub display: Rc<RefCell<Display>>,
}

impl Vm {
    pub fn new(context: &sdl2::Sdl) -> Self {
        let cpu = Rc::new(RefCell::new(Processor::new()));
        let memory = Rc::new(RefCell::new(Memory::new()));
        let keyboard = Rc::new(RefCell::new(Keyboard::new(context)));
        let display = Rc::new(RefCell::new(Display::new(context)));

        return Self {
            cpu,
            memory,
            keyboard,
            display
        }
    }

    pub fn fetch_instruction(&self) -> u16 {
        let mem = self.memory.borrow();
        let mut cpu = self.cpu.borrow_mut();
        let high = mem.read_ram(cpu.pc) as u16;
        let low = mem.read_ram(cpu.pc + 1) as u16;
        let op = (high << 8) | low;
        cpu.increment_pc();
        return op;
    }

    pub fn decode_instruction(&self, op: u16) -> (u16, u8, u8, u8, u8) {
        let nnn = op & 0x0FFF;
        let nn = (op & 0x00FF) as u8;
        let n = (op & 0x000F) as u8; //dig 4
        let x = ((op & 0x0F00) >> 8) as u8; //dig 2
        let y = ((op & 0x00F0) >> 4) as u8; //dig 3
        //op, x, y, n
        return (nnn, nn, n, x, y)
    }

    pub fn execute(&mut self, inst: u16) {
        let mut cpu = self.cpu.borrow_mut();
        let (nnn, nn, n, x, y) = self.decode_instruction(inst);
        let op = (inst & 0xF000) >> 12; //dig 1
        //println!("inst: {}, op: {}, nnn: {}, nn: {}, n: {}, x: {}, y: {}", inst, op, nnn, nn, n, x, y);

        match op {
            0x0 => cpu.display_op(n, self),
            0x1 => cpu.jump_to_addr(nnn),
            0x2 => cpu.call(nnn),
            0x3 => cpu.skip_execution(op, x, nn),
            0x4 => cpu.skip_execution(op, x, nn),
            0x5 => cpu.reg_check_skip_exec(op, x, y),
            0x6 => cpu.set_reg_nn(x, nn),
            0x7 => cpu.add_reg_nn(x, nn),
            0x8 => cpu.alu_op(x, y, n),
            0x9 => cpu.reg_check_skip_exec(op, x, y),
            0xA => cpu.set_i_to_addr(nnn),
            0xB => cpu.jump_to_addr_plus_offset(nnn),
            0xC => cpu.set_reg_rand(x, nn),
            0xD => cpu.draw_op(x, y, n, self),
            0xE => cpu.key_op(x, nn, self),
            0xF => cpu.timer_op(x, nn, self),
            _ => {
                println!("Unknown OpCode: {}", inst);
                exit(1);
            }
        }   
    }

    pub fn load(&mut self, filepath: &str) {
        let mut mem = self.memory.borrow_mut();
        mem.load_fonts();
        let path = Path::new(filepath);
        let name = path.file_name().unwrap().to_str().unwrap();
        let mut file = File::open(path).unwrap();
        println!("Loading ROM: {}", name);

        let mut buffer: [u8; 3824] = [0; 3824];
        let _ = file.read(&mut buffer);

        mem.load_rom(&buffer);
    }

    pub fn tick(&mut self) {
        let inst = self.fetch_instruction();
        self.execute(inst);
    }
}