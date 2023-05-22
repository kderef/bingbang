use crate::*;

pub fn interpret_instructions(
    instructions: &Vec<Instr>,
    stack: &mut Vec<StackVal>,
) -> Result<(), String> {
    for (idx, inst) in instructions.iter().enumerate() {
        match inst {
            Instr::PushStr(s) => stack.push(StackVal::String(s.into())),
            Instr::PushNum(i) => stack.push(StackVal::Number(*i)),

            Instr::Print => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing instruction [{inst:?}] at index {}, stack empty.",
                        idx
                    ));
                }

                print!("{last}", last = stack.pop().unwrap());
                pflush!();
            }
            Instr::PrintLn => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing instruction [{inst:?}] at index {}, stack empty.",
                        idx
                    ));
                }

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
                stack.push(StackVal::String(
                    buf.split_once('\n').unwrap().0.to_string(),
                ));
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
            }
            Instr::PrintStackLn => {
                if stack.len() == 0 {
                    continue;
                }
                while stack.len() != 0 {
                    println!("{}", stack.pop().unwrap());
                }
            }
            Instr::ShowStack => {
                for s in stack.iter() {
                    println!("{s}");
                }
            }
            Instr::Syscall => {
                if stack.len() < 2 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, stack must contain 2 values.",
                        inst, idx
                    ));
                }

                let oper1 = match stack.pop().unwrap() {
                    StackVal::Number(n) => n,
                    _ => {
                        return Err(format!(
                            "while trying to [{:?}] at index {}, type is not number.",
                            inst, idx,
                        ))
                    }
                };
                let oper2 = stack.pop().unwrap();

                match oper1 as i32 {
                    0 => {
                        // exit
                        if let StackVal::Number(n) = oper2 {
                            std::process::exit(n as i32);
                        } else {
                            return Err(format!("for syscall argument 0, got invalid type. Expected Number, got {:?}", oper2));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "at index {}: unkown syscall argument: {}",
                            idx, oper1
                        ))
                    }
                }
            }
            Instr::Time => {
                let time = chrono::Local::now().format("%d-%m-%Y");
                stack.push(StackVal::String(time.to_string()));
            }
            Instr::TimeFmt => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, stack is empty.",
                        inst, idx
                    ));
                }
                let last = stack.pop().unwrap();

                let to_push = {
                    if let StackVal::String(fmt) = last {
                        chrono::Local::now().format(&fmt).to_string()
                    } else {
                        return Err(format!(
                            "while trying to perform [{:?}] at index {}, expected type String, but got {:?}.",
                            inst, idx, last
                        ));
                    }
                };

                stack.push(StackVal::String(to_push));
            },
            Instr::Not => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, stack is empty.",
                        inst, idx
                    ))
                }
                let last = stack.pop().unwrap();

                if let StackVal::Bool(b) = last {
                    stack.push(StackVal::Bool(!b));
                } else {
                    return Err(
                        format!("while trying to perform operation [{:?}], expected Bool(), got {:?}", inst, last)
                    )
                }
            }
            Instr::IfStmt(body) => {
                if stack.len() < 1 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, stack is empty.",
                        inst, idx
                    ));
                }
                let last = stack.pop().unwrap();
                let cond = if let StackVal::Bool(b) = last {
                    b
                } else {
                    return Err(format!(
                        "while performing [{:?}] at index {}, expected Bool() on stack, got {:?}",
                        inst, idx, last
                    ));
                };

                if cond {
                    match interpret_instructions(body, stack) {
                        Ok(_) => {},
                        Err(e) => return Err(e)
                    }
                }
            },
            Instr::GreaterThan | Instr::LessThan => {
                if stack.len() < 2 {
                    return Err(format!(
                        "while performing [{:?}] at index {}, not enough elements in stack.",
                        inst, idx
                    ))
                }

                let elem1 = stack.pop().unwrap();
                let elem2 = stack.pop().unwrap();

                let oper1;
                let oper2;

                if let StackVal::Number(n) = elem1 {
                    oper1 = n;
                } else {
                    return Err(format!("while trying to perform [{inst:?}], Not a Number: {elem1:?}"))
                }

                if let StackVal::Number(n) = elem2 {
                    oper2 = n;
                } else {
                    return Err(format!("while trying to perform [{inst:?}], Not a Number: {elem2:?}"))
                }

                let to_push = if *inst == Instr::GreaterThan {
                    StackVal::Bool(oper1 > oper2)
                } else if *inst == Instr::LessThan {
                    StackVal::Bool(oper1 < oper2)
                } else {unimplemented!()};
                stack.push(to_push);
            },
            Instr::Reverse => {
                if stack.len() < 1 {
                    return Err(format!("while trying to perform [{inst:?}] at index {idx}, stack is empty."))
                }
                let last = stack.pop().unwrap();
                if let StackVal::String(s) = last {
                    stack.push(StackVal::String(s.chars().rev().collect()))
                } else {
                    return Err(format!("while trying to perform [{inst:?}] at index {idx}, expected String(), got {inst:?}"))
                }
            },
            Instr::Pop => {
                if stack.len() >= 1 {
                    stack.pop();
                }
            },
            Instr::GenRange => {
                if stack.len() < 2 {
                    return Err(format!(
                        "while trying to perform [{inst:?}] at index {idx}, stack length too short."
                    ))
                }

                let elem1 = stack.pop().unwrap();
                let elem2 = stack.pop().unwrap();

                let oper1;
                let oper2;

                if let StackVal::Number(n) = elem1 {
                    oper1 = n as i32;
                } else {
                    return Err(format!("failed to [{inst:?}]: Not a Number: {elem1:?}"))
                }
                if let StackVal::Number(n) = elem2 {
                    oper2 = n as i32;
                } else {
                    return Err(format!("failed to [{inst:?}]: Not a Number: {elem2:?}"))
                }

                for i in oper1..oper2 {
                    stack.push(StackVal::Number(i as f32));
                }
            },
            Instr::FlipStack => {
                stack.reverse();
            }
        }
    }
    Ok(())
}
