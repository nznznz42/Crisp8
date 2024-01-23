use crate::vm::Vm;

mod cpu;
mod memory;
mod consts;
mod keyboard;
mod vm;
mod display;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut vm = Vm::new(&sdl_context);
    vm.load("src/assets/test.ch8");

    loop {
        let inst = vm.fetch_instruction();
        vm.execute(inst);
        let mem = vm.memory.borrow_mut();
        //if mem.vram_modified {
            let mut dis = vm.display.borrow_mut();
            dis.draw(&mem, &vm);
        //}
    }
}
