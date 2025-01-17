use super::ReportSpan;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TypBody {
    Bool,
    Char,
    String,
    Symbol,
    Rational,
    Float,
    Int,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Syntax,
    Void,
    Vector(Box<Typ>),
    // homogeneous list: (list t)
    List(Box<Typ>),
    // (pair a b)
    Pair(Box<Typ>, Box<Typ>),
    Record(Vec<(String, Typ)>),
    Func { params: Vec<Typ>, result: Box<Typ> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Typ {
    pub span: ReportSpan,
    pub body: TypBody,
}

impl TypBody {
    pub fn with_span(self, span: ReportSpan) -> Typ {
        Typ { span, body: self }
    }
}

impl PartialEq for Typ {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}

impl Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}
impl Display for TypBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypBody::*;
        match self {
            Bool => write!(f, "bool"),
            Char => write!(f, "char"),
            String => write!(f, "string"),
            Symbol => write!(f, "symbol"),
            Rational => write!(f, "rational"),
            Float => write!(f, "float"),
            Int => write!(f, "int"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            U8 => write!(f, "u8"),
            U16 => write!(f, "u16"),
            U32 => write!(f, "u32"),
            U64 => write!(f, "u64"),
            Syntax => write!(f, "syntax"),
            Void => write!(f, "void"),
            Vector(typ) => write!(f, "(array {})", typ),
            List(typ) => write!(f, "(list {})", typ),
            Pair(a, b) => write!(f, "(pair {} {})", a, b),
            Record(_vec) => todo!(),
            Func { params, result } => {
                for (i, typ) in params.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", typ)?;
                    } else {
                        write!(f, " {}", typ)?;
                    }
                }
                write!(f, " -> {}", result)
            }
        }
    }
}
