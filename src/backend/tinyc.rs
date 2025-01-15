use std::fmt::Display;

pub struct CFile {
    pub declares: Vec<Declare>,
    pub funcs: Vec<DefineFunc>,
}
pub struct Declare {
    pub name: String,
    pub typ: CTyp,
}
pub struct DefineFunc {
    pub name: String,
    pub params: Vec<(String, CTyp)>,
    pub result: CTyp,
    pub statement: CStmt,
}
pub enum CStmt {
    Seq {
        cur: CExpr,
        next: Box<CStmt>,
    },
    Assign {
        name: String,
        expr: CExpr,
        next: Box<CStmt>,
    },
    Return(CExpr),
}
pub enum CExpr {
    CInt(i64),
    Id(String),
}
pub struct CTyp(String);
impl From<&str> for CTyp {
    fn from(value: &str) -> Self {
        CTyp(value.into())
    }
}

impl Display for CFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for decl in &self.declares {
            writeln!(f, "{}", decl)?;
        }
        for func in &self.funcs {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}
impl Display for Declare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.typ, self.name)
    }
}
impl Display for CExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CExpr::*;
        match self {
            CInt(i) => write!(f, "{}", i),
            Id(n) => write!(f, "{}", n),
        }
    }
}
impl Display for CStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CStmt::*;
        write!(f, "  ")?;
        match self {
            Return(e) => write!(f, "return {};", e),
            Assign { name, expr, next } => {
                writeln!(f, "{} = {};", name, expr)?;
                write!(f, "{}", next)
            }
            Seq { cur, next } => {
                writeln!(f, "{};", cur)?;
                write!(f, "{}", next)
            }
        }
    }
}
impl Display for DefineFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}(", self.result, self.name)?;
        for (i, (id, ty)) in self.params.iter().enumerate() {
            if i == 0 {
                write!(f, "{} {}", ty, id)?;
            } else {
                write!(f, ", {} {}", ty, id)?;
            }
        }
        writeln!(f, ") {{")?;
        writeln!(f, "{}", self.statement)?;
        write!(f, "}}")
    }
}
impl Display for CTyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for CFile {
    fn default() -> Self {
        Self {
            declares: Default::default(),
            funcs: Default::default(),
        }
    }
}
