use std::{env, fs, path::Path};

mod errhandling;
mod instruction;
mod stackval;
mod parse;
mod interpret;

use instruction::*;
use stackval::StackVal;
use parse::parse_bng;
use interpret::interpret_instructions;

fn perform_math_op(operands: (f32, f32), operation: &Instr) -> StackVal {
    match operation {
        Instr::Plus => StackVal::Number(operands.0 + operands.1),
        Instr::Div => StackVal::Number(operands.0 / operands.1),
        Instr::Sub => StackVal::Number(operands.0 - operands.1),
        Instr::Mul => StackVal::Number(operands.0 * operands.1),
        _ => unimplemented!(),
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        err!("expected 1 arg, got {}", args.len() - 1);
    }

    let fname = &args[1];

    if *fname == "-i".to_string() {
        // interpreter
        let mut buf = String::new();
        let stdin = std::io::stdin();

        println!("\x1b[92mbingbang interpreter\n   -> press q to quit\x1b[0m");

        let mut stack: Vec<StackVal> = Vec::new();

        loop {
            print!("\x1b[93m>\x1b[0m");
            pflush!();

            buf.clear();
            stdin
                .read_line(&mut buf)
                .expect("failed to read from stdin");

            let mut instructions = match parse_bng(buf.trim_end().into()) {
                Ok(v) => v,
                Err(e) => {
                    err!(e);
                    continue;
                }
            };
            if instructions.len() >= 1 {
                instructions.pop();
            }
            match interpret_instructions(instructions, &mut stack) {
                Ok(_) => (),
                Err(e) => err!(e),
            }
        }
    }

    if !Path::new(fname).is_file() {
        err!("file `{fname}` does not exist.");
    }

    let content = fs::read_to_string(fname).unwrap();

    let instructions = match parse_bng(content) {
        Ok(v) => v,
        Err(e) => {
            err!(e);
            std::process::exit(1);
        }
    };

    println!("instructions = {instructions:#?}");
    println!("\x1b[93m{fname}\x1b[0m\n");

    let mut stack: Vec<StackVal> = Vec::new();

    match interpret_instructions(instructions, &mut stack) {
        Ok(_) => (),
        Err(e) => err!(e),
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_bng, Instr};
    #[test]
    fn hello_world() {
        let parsed = parse_bng("'Hello World'P".into()).unwrap();
        assert!(
            parsed
                == vec![
                    Instr::PushStr("Hello World".into()),
                    Instr::PrintLn,
                    Instr::Exit
                ]
        );
    }
    #[test]
    fn basic_math() {
        let parsed = parse_bng("1 2 3 + P".into()).unwrap();
        assert!(
            parsed
                == vec![
                    Instr::PushNum(1.0),
                    Instr::PushNum(2.0),
                    Instr::PushNum(3.0),
                    Instr::Plus,
                    Instr::PrintLn,
                    Instr::Exit
                ]
        );
    }

    #[test]
    fn parse_numbers() {
        let parsed = parse_bng("1 '1'n + P".into()).unwrap();
        assert!(
            parsed
                == vec![
                    Instr::PushNum(1.0),
                    Instr::PushStr("1".into()),
                    Instr::ParseNum,
                    Instr::Plus,
                    Instr::PrintLn,
                    Instr::Exit
                ]
        );
    }
}
