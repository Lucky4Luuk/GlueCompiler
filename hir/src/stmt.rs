use crate::expr::Expr;
use crate::code_block::CodeBlock;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: String, value: Expr, type_string: String },
    Expr(Expr),
    CodeBlock(Box<CodeBlock>),
    Return(Expr),
}

impl Stmt {
    pub fn lower(ast: ast::Stmt) -> Option<Self> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Self::VariableDef {
                name: ast.name()?.text().to_string(),
                value: Expr::lower(ast.value()),
                type_string: ast.kind()?.text().to_string(),
            },
            ast::Stmt::Expr(ast) => Self::Expr(Expr::lower(Some(ast))),
            ast::Stmt::CodeBlock(ast) => Self::CodeBlock(Box::new(CodeBlock::lower(ast))),
            ast::Stmt::Return(ast) => Self::Return {
                0: Expr::lower(ast.value())
            }
        };

        Some(result)
    }
}
