
#[derive(Debug)]
pub enum StackVal {
    Number(f32),
    String(String),
    Bool(bool)
}

impl std::cmp::PartialEq for StackVal {
    fn eq(&self, other: &Self) -> bool {

        match self {
            Self::Number(n1) => {
                match other {
                    Self::Number(n2) => n1 == n2,
                    Self::Bool(b) => *n1 == ((*b as i8) as f32),
                    Self::String(s) => n1.to_string() == *s
                }
            },
            Self::Bool(b) => {
                match other {
                    Self::Number(n2) => ((*b as i8) as f32) == *n2,
                    Self::Bool(b2) => b == b2,
                    Self::String(s) => b.to_string() == s.to_string()
                }
            },
            Self::String(s) => {
                match other {
                    Self::Number(n2) => n2.to_string() == *s,
                    Self::Bool(b2) => b2.to_string() == *s,
                    Self::String(s2) => s == s2
                }
            }
        }
    }
}

impl std::fmt::Display for StackVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackVal::Number(n) => write!(f, "{n}"),
            StackVal::String(s) => write!(f, "{s}"),
            StackVal::Bool(b) => write!(f, "{b}"),
        }
    }
}