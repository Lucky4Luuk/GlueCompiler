use common::LiteralType;

use hir::expr::{Expr, BinaryOp};

pub fn build_expr(hir_expr: &Expr) -> String {
    match hir_expr {
        Expr::VariableRef { var } => var.to_string(),
        Expr::FunctionCall { func, args } => {
            let mut tmp = String::new();
            tmp.push_str(func);
            tmp.push_str("(");
            let mut i = 0;
            let l = args.len();
            for arg in args {
                tmp.push_str(&build_expr(arg));
                if i < l-1 {
                    tmp.push_str(", ");
                    i += 1;
                }
            }
            tmp.push_str(")");
            tmp
        },
        Expr::Binary { op, lhs, rhs } => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
            };
            let mut tmp = String::new();
            tmp.push_str(&build_expr(&lhs));
            tmp.push_str(op_str);
            tmp.push_str(&build_expr(&rhs));
            tmp
        }
        Expr::Literal { n } => {
            match n {
                LiteralType::U64(v) => format!("{}", v),
                LiteralType::U32(v) => format!("{}", v),
                LiteralType::U16(v) => format!("{}", v),
                LiteralType::U8(v)  => format!("{}", v),

                LiteralType::I64(v) => format!("{}", v),
                LiteralType::I32(v) => format!("{}", v),
                LiteralType::I16(v) => format!("{}", v),
                LiteralType::I8(v)  => format!("{}", v),

                LiteralType::F64(v) => format!("{}", v),
                LiteralType::F32(v) => format!("{}", v),

                _ => panic!("Literal type is missing or does not exist!")
            }
        },
        _ => unimplemented!(),
    }
}
