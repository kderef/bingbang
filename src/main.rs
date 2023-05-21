use std::{env, fmt::Display, fs, ops::Sub, path::Path};

const PRINT: char = 'p';
const PRINTLN: char = 'P';
const READ: char = 'i';

const STR_DELIM: char = '\'';

const EXIT: char = '\\';
const PUSH_DELIM: char = ',';

const DIV: char = '/';
const PLUS: char = '+';
const SUB: char = '-';
const MUL: char = '*';
const SUM: char = 's';

const PARSE_NUM: char = 'n';

macro_rules! pflush {
    () => {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    };
}
macro_rules! err {
    ($msg:expr) => {
        eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", $msg);
        std::process::exit(1);
    };
    ($msg:expr, $($fmt_args:expr),*) => {
        {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", format!($msg, $($fmt_args),*));
            std::process::exit(1);
        }
    };
}

macro_rules! assert_err {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", $msg);
            std::process::exit(1);
        }
    };
    ($cond:expr, $msg:expr, $($fmt_args:expr),*) => {
        if !($cond) {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", format!($msg, $($fmt_args),*));
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
enum Instr {
    Print,
    PrintLn,
    Read,

    Div,
    Plus,
    Sub,
    Mul,
    Sum,

    ParseNum,

    PushStr(String),
    PushNum(f32),

    Exit,
}

#[derive(Debug)]
enum StackVal {
    Number(f32),
    String(String),
}

impl Display for StackVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackVal::Number(n) => write!(f, "{n}"),
            StackVal::String(s) => write!(f, "{s}"),
        }
    }
}

fn perform_math_op(operands: (f32, f32), operation: &Instr) -> StackVal {
    match operation {
        Instr::Plus => StackVal::Number(operands.0 + operands.1),
        Instr::Div => StackVal::Number(operands.0 / operands.1),
        Instr::Sub => StackVal::Number(operands.0 - operands.1),
        Instr::Mul => StackVal::Number(operands.0 * operands.1),
        _ => unimplemented!(),
    }
}

fn interpret_instructions(instructions: Vec<Instr>) {
    let mut stack: Vec<StackVal> = Vec::new();

    for (idx, inst) in instructions.iter().enumerate() {
        match inst {
            Instr::PushStr(s) => stack.push(StackVal::String(s.into())),
            Instr::PushNum(i) => stack.push(StackVal::Number(*i)),

            Instr::Print => {
                assert_err!(
                    stack.len() >= 1,
                    "while performing instruction [Print] at index {}, stack empty.",
                    idx
                );

                print!("{last}", last = stack.pop().unwrap());
                pflush!();
            }
            Instr::PrintLn => {
                println!("{last}", last = stack.pop().unwrap());
            }

            Instr::Plus | Instr::Div | Instr::Sub | Instr::Mul => {
                assert_err!(
                    stack.len() >= 2,
                    "while performing [{:?}] at index {}, stack length too short. (must be >= 2)",
                    inst,
                    idx
                );

                let vals: Vec<f32> = vec![stack.pop().unwrap(), stack.pop().unwrap()].iter().map(|v| {
                    match v {
                        StackVal::Number(n) => *n,
                        StackVal::String(_) => {
                            err!("while performing [{:?}] at index {}, expected type Number, but got type String.", inst, idx);
                        }
                    }
                }).collect();

                stack.push(perform_math_op((vals[0], vals[1]), inst))
            }
            Instr::Sum => {
                assert_err!(
                    stack.len() >= 1,
                    "while performing [{:?}] at index {}, failed to SUM because stack is empty.",
                    inst,
                    idx
                );

                let mut nums: Vec<f32> = Vec::new();

                for s in &stack {
                    if let StackVal::Number(n) = s {
                        nums.push(*n);
                    } else {
                        err!(
                            "while performing [{:?}] at index {}, failed to SUM because non-number is in stack.",
                            inst, idx
                        )
                    }
                }

                let sum = nums.iter().fold(0.0, |acc: f32, n| acc + n);
                stack.push(StackVal::Number(sum));
            }
            Instr::Read => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                stack.push(StackVal::String(buf));
            }
            Instr::ParseNum => {
                assert_err!(
                    stack.len() >= 1,
                    "while trying to [{:?}] at index {}, failed because stack is empty.",
                    inst,
                    idx
                );
                let parsed = match stack.pop().unwrap() {
                    StackVal::Number(n) => n,
                    StackVal::String(s) => {
                        let p = s.trim().parse::<f32>();
                        if let Err(e) = p {
                            err!("while trying to [{:?}] at index {}, failed to parse because of error: {}",
                                inst, idx, e);
                        } else {
                            p.unwrap()
                        }
                    }
                };
                stack.push(StackVal::Number(parsed));
            }
            Instr::Exit => std::process::exit(0),
        }
    }
}

fn parse_bng(line: String) -> Vec<Instr> {
    let mut instructions: Vec<Instr> = Vec::new();
    let mut chars = line.chars().collect::<Vec<char>>();

    let mut pos = (1u32, 1u32);

    while chars.len() >= 1 {
        pos.1 += 1;
        let c = chars.remove(0);

        match c {
            '\n' => {
                pos.0 += 1;
                continue;
            }
            PUSH_DELIM | ' ' => continue,
            '0'..='9' => {
                let mut nums = String::from(c);

                let ch_clone = chars.clone();

                if ch_clone.len() >= 1 {
                    if !ch_clone.get(0).unwrap().is_numeric() {
                        instructions.push(Instr::PushNum((c as i32 - 48) as f32));
                        continue;
                    }

                    for x in chars.clone() {
                        if x.is_numeric() {
                            nums.push(x);
                            chars.remove(0);
                            pos.1 += 1;
                        } else {
                            break;
                        }
                    }
                }

                instructions.push(Instr::PushNum(nums.parse().unwrap()));
            }
            STR_DELIM => {
                let mut tot_str = String::new();

                let mut sc = chars.remove(0);

                if sc == STR_DELIM {
                    instructions.push(Instr::PushStr("".into()));
                    continue;
                } else {
                    tot_str.push(sc);
                }

                while sc != STR_DELIM {
                    assert_err!(chars.len() != 0, "line {}; string never closed.", pos.0);
                    sc = chars.remove(0);

                    if sc == '\n' {
                        pos.0 += 1;
                    } else {
                        pos.1 += 1;
                    }

                    tot_str.push(sc);
                }

                let new = tot_str.strip_suffix(STR_DELIM);

                if new.is_some() {
                    tot_str = new.unwrap().into();
                }

                instructions.push(Instr::PushStr(tot_str));
                continue;
            }
            PRINT => instructions.push(Instr::Print),
            PRINTLN => instructions.push(Instr::PrintLn),

            PLUS => instructions.push(Instr::Plus),
            SUB => instructions.push(Instr::Sub),
            DIV => instructions.push(Instr::Div),
            MUL => instructions.push(Instr::Mul),
            SUM => instructions.push(Instr::Sum),

            READ => instructions.push(Instr::Read),

            EXIT => instructions.push(Instr::Exit),
            PARSE_NUM => instructions.push(Instr::ParseNum),

            _ => err!("unkown token: {}", c),
        }
    }

    println!("{instructions:?}");

    instructions.push(Instr::Exit);

    instructions
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        err!("expected 1 arg, got {}", args.len() - 1);
    }

    let fname = &args[1];

    if !Path::new(fname).is_file() {
        err!("file `{fname}` does not exist.");
    }

    let content = fs::read_to_string(fname).unwrap();

    let instructions = parse_bng(content);

    println!("instructions = {instructions:#?}");
    println!("\x1b[93m{fname}\x1b[0m\n");

    interpret_instructions(instructions);
}
