pub struct CFile {}
pub struct Declare {
    name: String,
    typ: CTyp,
}
pub struct DefineFunc {
    name: String,
    params: Vec<(String, CTyp)>,
    statements: Statement,
}
pub enum Statement {
    Return(CExpr),
}
pub enum CExpr {
    CInt(i64),
}
pub enum CTyp {
    Name(String),
}
