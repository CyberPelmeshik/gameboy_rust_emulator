mod cpu;
mod mmu;
mod timer;

use std::range;

use crate::cpu::Cpu;
use crate::mmu::Mmu;

fn main() {
    //let game_path: &str = "../data/Space Invaders (USA) (SGB Enhanced).gb";

    let game_path: &str = "./data/cpu_instrs/individual/01-special.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);
    println!("Start");
    loop {
        let cycles_cnt = cpu.step(&mut mmu);
        mmu.timediv_reg_add(cycles_cnt as u8);
    }
    //let _ = mmu.load_rom(game_path);
}
