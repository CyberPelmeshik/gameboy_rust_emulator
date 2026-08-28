mod cpu;
mod mmu;

use std::range;

use crate::cpu::Cpu;
use crate::mmu::Mmu;

fn main() {
    //let game_path: &str = "../data/Space Invaders (USA) (SGB Enhanced).gb";

    let game_path: &str = "./data/cpu_instrs/individual/01-special.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    mmu.load_rom(game_path);
    println!("Start");
    loop {
        cpu.step(&mut mmu);
    }
    //let _ = mmu.load_rom(game_path);
}
