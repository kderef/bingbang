use crate::*;

pub fn parse_bng(line: String) -> Result<Vec<Instr>, String> {
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
                    if chars.len() == 0 {
                        return Err(format!("line {}; never closed.", pos.0));
                    }
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
            EQUALS => instructions.push(Instr::Eq),

            EXIT => instructions.push(Instr::Exit),
            PARSE_NUM => instructions.push(Instr::ParseNum),
            CLEAR_STACK => instructions.push(Instr::ClearStack),
            PRINT_STACK => instructions.push(Instr::PrintStack),
            PRINT_STACK_LN => instructions.push(Instr::PrintStackLn),
            SHOW_STACK => instructions.push(Instr::ShowStack),

            SYSCALL => instructions.push(Instr::Syscall),

            _ => return Err(format!("unkown token: {}", c)),
        }
    }

    instructions.push(Instr::Exit);

    Ok(instructions)
}