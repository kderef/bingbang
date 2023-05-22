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
            'a' => instructions.push(Instr::PrintStack),
            'A' => instructions.push(Instr::PrintStackLn),
            'b' => {},
            'B' => {},
            'c' => instructions.push(Instr::ClearStack),
            'C' => {},
            'd' => instructions.push(Instr::Pop),
            'D' => {},
            'e' => {},
            'E' => {},
            'f' => {},
            'F' => {},
            'g' => {},
            'G' => {},
            'h' => {},
            'H' => {},
            'i' => instructions.push(Instr::Read),
            'I' => {},
            'j' => {},
            'J' => {},
            'k' => {},
            'K' => {},
            'l' => {},
            'L' => {},
            'm' => {},
            'M' => {},
            'n' => instructions.push(Instr::ParseNum),
            'N' => {},
            'o' => {},
            'O' => {},
            'p' => instructions.push(Instr::Print),
            'P' => instructions.push(Instr::PrintLn),
            'q' => instructions.push(Instr::Exit),
            'Q' => {},
            'r' => instructions.push(Instr::Reverse),
            'R' => instructions.push(Instr::FlipStack),
            's' => instructions.push(Instr::Sum),
            'S' => instructions.push(Instr::ShowStack),
            't' => instructions.push(Instr::Time),
            'T' => instructions.push(Instr::TimeFmt),
            'u' => {},
            'U' => {},
            'v' => {},
            'V' => {},
            'w' => {},
            'W' => {},
            'x' => {},
            'X' => {},
            'y' => {},
            'Y' => {},
            'z' => {},
            'Z' => {}

            PLUS => instructions.push(Instr::Plus),
            SUB => instructions.push(Instr::Sub),
            DIV => instructions.push(Instr::Div),
            MUL => instructions.push(Instr::Mul),

            GT => instructions.push(Instr::GreaterThan),
            LT => instructions.push(Instr::LessThan),

            RANGE => instructions.push(Instr::GenRange),

            EQUALS => instructions.push(Instr::Eq),
            SYSCALL => instructions.push(Instr::Syscall),
            NOT => instructions.push(Instr::Not),
            IF => {
                // 1 1 = ? ['1 == '1P]
                let mut next = chars.remove(0);

                while next.is_whitespace() {
                    next = chars.remove(0);
                }

                if next != '[' {
                    return Err(format!("unexpected token after '{IF}': expected '[', but got '{next}'."))
                }

                let mut body = String::new();


                while next != ']' {
                    next = chars.remove(0);
                    body.push(next);
                }
                body.pop().unwrap();
                let mut parsed_body = match parse_bng(body) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                parsed_body.pop();

                instructions.push(Instr::IfStmt(parsed_body));
            },


            _ => return Err(format!("unkown token: {}", c)),
        }
    }

    instructions.push(Instr::Exit);

    Ok(instructions)
}