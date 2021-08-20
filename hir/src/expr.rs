use syntax::SyntaxKind;
use common::LiteralType;

#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary { op: BinaryOp, lhs: Box<Self>, rhs: Box<Self> },
    Literal { n: LiteralType },
    Unary { op: UnaryOp, expr: Box<Self> },
    VariableRef { var: String },
    FunctionCall { func: String, args: Vec<Expr> },
}

impl Expr {
    pub fn lower(ast: Option<ast::Expr>) -> Self {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => Self::lower_binary(ast),
                ast::Expr::Literal(ast) => Self::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => Expr::lower(ast.expr()),
                ast::Expr::UnaryExpr(ast) => Self::lower_unary(ast),
                ast::Expr::VariableRef(ast) => Self::VariableRef { var: ast.name() },
                ast::Expr::FunctionCall(ast) => {
                    let args_raw = ast.args();
                    let mut args = Vec::new();
                    for arg_raw in args_raw {
                        args.push(Expr::lower(Some(arg_raw)));
                    }
                    Self::FunctionCall { func: ast.name(), args: args }
                },
            }
        } else {
            Self::Missing
        }
    }

    fn lower_binary(ast: ast::BinaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Asterix => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        Self::Binary {
            op,
            lhs: Box::new(Expr::lower(ast.lhs())),
            rhs: Box::new(Expr::lower(ast.rhs())),
        }
    }

    fn lower_unary(ast: ast::UnaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        Self::Unary {
            op,
            expr: Box::new(Expr::lower(ast.expr())),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}
