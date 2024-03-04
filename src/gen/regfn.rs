use super::gen::REGISTERS;
//todo : creating a output file

pub static mut FREE_REGISTERS: [i32; 4] = [1, 1, 1, 1];
pub fn cgpreamble() {
    free_all_registers();
}
// close all the unsafe blocks
pub fn cgpostamble() {}
pub fn cgload(value: i32)-> i32 {
    let reg: i32 = alloc_register();
    println!("\tmov\t{}, %d{}", REGISTERS[reg as usize], value);
    reg
}

pub fn cgadd(leftreg: i32, rightreg: i32) -> i32 {
    println!(
        "\taddq {}, {}",
        REGISTERS[leftreg as usize], REGISTERS[rightreg as usize]
    );
    free_register(leftreg);
    leftreg
}
pub fn cgmul(leftreg: i32, rightreg: i32) -> i32 {
    println!(
        "\timulq {}, {}",
        REGISTERS[leftreg as usize], REGISTERS[rightreg as usize]
    );
    free_register(leftreg);
    leftreg
}

pub fn cgsub(leftreg: i32, rightreg: i32) -> i32 {
    println!(
        "\tsubq {}, {}",
        REGISTERS[leftreg as usize], REGISTERS[rightreg as usize]
    );
    free_register(leftreg);
    leftreg
}
pub fn cgdiv(leftreg: i32, rightreg: i32) -> i32 {
    println!("\tmovq %rax, {}", REGISTERS[leftreg as usize]);
    println!("\tcqo");
    println!("\tidivq {}", REGISTERS[rightreg as usize]);
    free_register(rightreg);
    leftreg
}
pub fn cgprintint(reg: i32) {
    println!("\tmovq {}, %rdi", REGISTERS[reg as usize]);
    println!("\tcallq printint");
    free_register(reg);
}

pub fn alloc_register() -> i32 {
    unsafe {
        for i in 0..4 {
            if FREE_REGISTERS[i] == 1 {
                FREE_REGISTERS[i] = 0;
                return i as i32;
            }
        }
    }
    println!("Out of registers");
    std::process::exit(1);
}

pub fn free_register(reg: i32) {
    unsafe {
        if FREE_REGISTERS[reg as usize] == 0 {
            FREE_REGISTERS[reg as usize] = 1;
        } else {
            println!("Error: freeing register {}", reg);
            std::process::exit(1);
        }
    }
}

pub fn free_all_registers() {
    unsafe {
        FREE_REGISTERS[0] = 1;
        FREE_REGISTERS[1] = 1;
        FREE_REGISTERS[2] = 1;
        FREE_REGISTERS[3] = 1;
    }
}


