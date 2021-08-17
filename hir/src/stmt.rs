use crate::expr::Expr;
use crate::code_block::CodeBlock;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
    CodeBlock(Box<CodeBlock>),
}

impl Stmt {
    pub fn lower(ast: ast::Stmt) -> Option<Self> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Self::VariableDef {
                name: ast.name()?.text().to_string(),
                value: Expr::lower(ast.value()),
            },
            ast::Stmt::Expr(ast) => Self::Expr(Expr::lower(Some(ast))),
            ast::Stmt::CodeBlock(ast) => Self::CodeBlock(Box::new(CodeBlock::lower(ast)))
        };

        Some(result)
    }
}
