use crate::*;

pub fn interpret_instructions(
    instructions: Vec<Instr>,
    stack: &mut Vec<StackVal>,
) -> Result<(), String> {
    for (idx, inst) in instructions.iter().enumerate() {
        match inst {
            Instr::PushStr(s) => stack.push(StackVal::String(s.into())),
            Instr::PushNum(i) => stack.push(StackVal::Number(*i)),

            Instr::Print => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing instruction [Print] at index {}, stack empty.",
                        idx
                    ));
                }

                print!("{last}", last = stack.pop().unwrap());
                pflush!();
            }
            Instr::PrintLn => {
                println!("{last}", last = stack.pop().unwrap());
            }

            Instr::Plus | Instr::Div | Instr::Sub | Instr::Mul => {
                if stack.len() < 2 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, stack length too short. (must be >= 2)",
                        inst,
                        idx
                    ));
                }

                let vals: Vec<Result<f32, String>> = vec![stack.pop().unwrap(), stack.pop().unwrap()].iter().map(|v| {
                    match v {
                        StackVal::Number(n) => Ok(*n),
                        _ => Err(format!("while performing [{:?}] at index {}, expected type Number, but got type {}.", inst, idx, v))
                    }
                }).collect();

                let oper1;
                let oper2;

                match &vals[0] {
                    Ok(n) => oper1 = n,
                    Err(e) => return Err(e.clone()),
                }
                match &vals[1] {
                    Ok(n) => oper2 = n,
                    Err(e) => return Err(e.clone()),
                }

                stack.push(perform_math_op((*oper1, *oper2), inst))
            }
            Instr::Sum => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, failed to SUM because stack is empty.",
                        inst,
                        idx
                    ));
                }

                let mut nums: Vec<f32> = Vec::new();

                while stack.len() >= 1 {
                    if let StackVal::Number(n) = stack.pop().unwrap() {
                        nums.push(n);
                    } else {
                        err!(
                            "while performing [{:?}] at index {}, failed to SUM because non-number is in stack.",
                            inst, idx
                        )
                    }
                }

                for s in stack.iter() {
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
                if stack.len() < 1 {
                    return Err(format!(
                        "while trying to [{:?}] at index {}, failed because stack is empty.",
                        inst, idx
                    ));
                }
                let parsed = match stack.pop().unwrap() {
                    StackVal::Number(n) => n,
                    StackVal::String(s) => {
                        let p = s.trim().parse::<f32>();
                        if let Err(e) = p {
                            return Err(format!("while trying to [{:?}] at index {}, failed to parse because of error: {}",
                                inst, idx, e));
                        } else {
                            p.unwrap()
                        }
                    }
                    StackVal::Bool(b) => (b as i8) as f32,
                };
                stack.push(StackVal::Number(parsed));
            }

            Instr::Eq => {
                let l = stack.len();
                if l < 2 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, expected stack length to be >= 2, but got {}.",
                        inst, idx, l
                    ));
                }

                let elem1 = stack.pop().unwrap();
                let elem2 = stack.pop().unwrap();

                stack.push(StackVal::Bool(elem1 == elem2));
            }

            Instr::Exit => std::process::exit(0),
            Instr::ClearStack => stack.clear(),
            Instr::PrintStack => {
                if stack.len() == 0 {
                    continue;
                }
                while stack.len() > 1 {
                    print!("{}", stack.pop().unwrap());
                    pflush!();
                }
                println!("");
            },
            Instr::PrintStackLn => {
                if stack.len() == 0 {
                    continue;
                }
                while stack.len() != 0 {
                    println!("{}", stack.pop().unwrap());
                }
            },
            Instr::ShowStack => {
                for s in stack.iter() {
                    println!("{s}");
                }
            }
        }
    }
    Ok(())
}