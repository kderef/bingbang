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

pub const FN_DEF: char = '@';
pub const FN_CALL: char = ':';

pub const LOOP_IN: char = '(';
pub const LOOP_OUT: char = ')';

pub const INTERACTIVE_COMMENT: char = ';';

type Body = Vec<Instr>;

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

    FnDef,
    FnCall,

    Loop(Body),

    Pop,

    ClearScreen,

    PushStr(String),
    PushNum(f32),

    Syscall,
    Time,
    TimeFmt,

    IfStmt(Body),

    Eq,
    Not,
    ClearStack,
    PrintStack,
    PrintStackLn,
    ShowStack,
    Exit,
}
