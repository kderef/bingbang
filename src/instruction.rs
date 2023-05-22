
pub const STR_DELIM: char = '\'';

pub const PUSH_DELIM: char = ',';

pub const DIV: char = '/';
pub const PLUS: char = '+';
pub const SUB: char = '-';
pub const MUL: char = '*';
pub const EQUALS: char = '=';

pub const IF: char = '?';
pub const NOT: char = '!';

pub const SYSCALL: char = '$';
pub const GT: char = '>';
pub const LT: char = '<';
pub const RANGE: char = '.';

pub const INTERACTIVE_COMMENT: char = ';';

#[derive(Debug, PartialEq)]
pub enum Instr {
    Print,
    PrintLn,
    Read,

    Div,
    Plus,
    Sub,
    Mul,
    GreaterThan,
    LessThan,
    Sum,

    ParseNum,
    Reverse,
    GenRange,
    FlipStack,

    Pop,

    PushStr(String),
    PushNum(f32),

    Syscall,
    Time,
    TimeFmt,

    IfStmt(Vec<Instr>),

    Eq,
    Not,
    ClearStack,
    PrintStack,
    PrintStackLn,
    ShowStack,
    Exit,
}
