use super::*;

pub(super) fn continue_statement(input: &str) -> Res<&str, Stmt> {
    context(
        "continue_statement",
        tuple((tag("continue"), ws0, tag(";"))),
    )(input).map(|(next, val)| (next, Stmt::Continue))
}
