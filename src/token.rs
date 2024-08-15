use strum::EnumIter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum Operator {
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Divide,
    /// "%"
    Mod,
    /// "=="
    Eq,
    /// "!="
    Neq,
    /// "<"
    Lt,
    /// "<="
    Leq,
    /// ">"
    Gt,
    /// ">="
    Geq,
    /// "="
    Assign,
    /// "->"
    PointsTo,
    /// "."
    Dot,
    /// "&"
    And,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum Seperator {
    /// "("
    Openpa,
    /// ")"
    Closepa,
    /// "["
    Openbr,
    /// "]"
    Closeba,
    /// "{"
    Begin,
    /// "}"
    End,
    /// ";"
    Semicolon,
    /// ","
    Comma,
    /// "..."
    Ellipsis,
    /// End of file
    Eof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstVar {
    /// const int
    Cint(i32),
    /// const char
    Cchar(char),
    /// const string
    Cstring(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum KeyWord {
    Char,
    Short,
    Int,
    Void,
    Struct,
    If,
    Else,
    For,
    Continue,
    Break,
    Return,
    Sizeof,
    /// "__align"
    Align,
    /// "__cdecl"
    Cdecl,
    /// "__stdcall"
    Stdcall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Op(Operator),
    Sep(Seperator),
    Cvar(ConstVar),
    Kw(KeyWord),
    Ident(String),
}

impl Token {
    pub fn get_string(&self) -> String {
        match self {
            Token::Op(op) => match op {
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Star => "*",
                Operator::Divide => "/",
                Operator::Mod => "%",
                Operator::Eq => "==",
                Operator::Neq => "!=",
                Operator::Lt => "<",
                Operator::Leq => "<=",
                Operator::Gt => ">",
                Operator::Geq => ">=",
                Operator::Assign => "=",
                Operator::PointsTo => "->",
                Operator::Dot => ".",
                Operator::And => "&",
            }
            .to_string(),
            Token::Sep(sep) => match sep {
                Seperator::Openpa => "(",
                Seperator::Closepa => ")",
                Seperator::Openbr => "[",
                Seperator::Closeba => "]",
                Seperator::Begin => "{",
                Seperator::End => "}",
                Seperator::Semicolon => ";",
                Seperator::Comma => ",",
                Seperator::Ellipsis => "...",
                Seperator::Eof => "End of file",
            }
            .to_string(),
            Token::Cvar(c) => match c {
                ConstVar::Cint(x) => x.to_string(),
                ConstVar::Cchar(c) => c.to_string(),
                ConstVar::Cstring(s) => s.clone(),
            },
            Token::Kw(kw) => match kw {
                KeyWord::Char => "char",
                KeyWord::Short => "short",
                KeyWord::Int => "int",
                KeyWord::Void => "void",
                KeyWord::Struct => "struct",
                KeyWord::If => "if",
                KeyWord::Else => "else",
                KeyWord::For => "for",
                KeyWord::Continue => "continue",
                KeyWord::Break => "break",
                KeyWord::Return => "return",
                KeyWord::Sizeof => "sizeof",
                KeyWord::Align => "__align",
                KeyWord::Cdecl => "__cdecl",
                KeyWord::Stdcall => "__stdcall",
            }
            .to_string(),
            Token::Ident(s) => s.clone(),
        }
    }
}
