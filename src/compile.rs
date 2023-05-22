use std::fs::OpenOptions;
use std::io::Write;

use crate::*;

pub fn compile(instructions: &mut Vec<Instr>) {
    todo!();

    if std::path::Path::new("out.rs").is_file() {
        std::fs::remove_file("out.rs").unwrap();
    }

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("out.rs")
        .unwrap();

    if !instructions.ends_with(&[Instr::Exit]) {
        instructions.push(Instr::Exit);
    }

    for instr in instructions.iter() {
        match instr {
            Instr::PushNum(n) => {
                writeln!(out_file, "{}", format!("; --- push {n} to stack\nPUSH {}", *n as i32));
            },
            Instr::Exit => writeln!(out_file, "{ASM_EXIT}").unwrap(),
            _ => todo!()
        }
    }
}
