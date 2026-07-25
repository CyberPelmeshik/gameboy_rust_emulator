mod cpu;
mod mmu;

use crate::cpu::Cpu;
use crate::mmu::Mmu;

fn main() {
    let game_path: &str = "../data/Space Invaders (USA) (SGB Enhanced).gb";

    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    //let _ = mmu.load_rom(game_path);
}
