pub const PRINT: char = 'p';
pub const PRINTLN: char = 'P';
pub const READ: char = 'i';

pub const STR_DELIM: char = '\'';

pub const EXIT: char = 'q';
pub const PUSH_DELIM: char = ',';

pub const DIV: char = '/';
pub const PLUS: char = '+';
pub const SUB: char = '-';
pub const MUL: char = '*';
pub const EQUALS: char = '=';
pub const SUM: char = 's';

pub const PARSE_NUM: char = 'n';
pub const CLEAR_STACK: char = 'c';
pub const PRINT_STACK: char = 'a';
pub const PRINT_STACK_LN: char = 'A';
pub const SHOW_STACK: char = 'S';


#[derive(Debug, PartialEq)]
pub enum Instr {
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

    Eq,
    ClearStack,
    PrintStack,
    PrintStackLn,
    ShowStack,
    Exit,
}
