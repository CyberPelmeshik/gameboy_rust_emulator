use gameboy::cpu::Cpu;
use gameboy::mmu::Mmu;

#[test]
fn cpu_test_rom_01_scecial() {
    let game_path: &str = "./data/cpu_instrs/individual/01-special.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
#[ignore]
fn cpu_test_rom_02_interrupts() {
    let game_path: &str = "./data/cpu_instrs/individual/02-interrupts.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_03_op_sp_hl() {
    let game_path: &str = "./data/cpu_instrs/individual/03-op sp,hl.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_04_op_r_imm() {
    let game_path: &str = "./data/cpu_instrs/individual/04-op r,imm.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_05_op_rp() {
    let game_path: &str = "./data/cpu_instrs/individual/05-op rp.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_06_ld_r_r() {
    let game_path: &str = "./data/cpu_instrs/individual/06-ld r,r.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_07_jr_jp_call_ret_rst() {
    let game_path: &str = "./data/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_08_misc_instrs() {
    let game_path: &str = "./data/cpu_instrs/individual/08-misc instrs.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_09_op_r_r() {
    let game_path: &str = "./data/cpu_instrs/individual/09-op r,r.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_10_bit_ops() {
    let game_path: &str = "./data/cpu_instrs/individual/10-bit ops.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
fn cpu_test_rom_11_op_a_hl() {
    let game_path: &str = "./data/cpu_instrs/individual/11-op a,(hl).gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}

#[test]
#[ignore]
fn cpu_test_all_instr() {
    let game_path: &str = "./data/cpu_instrs/cpu_instrs.gb";
    let mut mmu = Mmu::new();
    let mut cpu = Cpu::new();

    let _ = mmu.load_rom(game_path);

    loop {
        cpu.step(&mut mmu);
        if mmu.sb().contains("Passed") {
            return;
        }
        if mmu.sb().contains("Failed") {
            print!("{}", mmu.sb());
            panic!();
        }
    }
}
