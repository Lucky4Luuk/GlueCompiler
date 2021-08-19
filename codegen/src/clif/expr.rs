use cranelift::prelude::types::*;
use cranelift::prelude::FunctionBuilder;
use cranelift::prelude::InstBuilder;

use hir::expr::{BinaryOp, Expr};
use common::LiteralType;

use super::variable::{VariableMap, temp_type_map};

pub fn build_expr(builder: &mut FunctionBuilder, hir_expr: &Expr, var_map: &mut VariableMap) -> cranelift::prelude::Value {
    match hir_expr {
        Expr::Literal { n } => {
            match n {
                LiteralType::I64(v) => builder.ins().iconst(I64, *v),
                LiteralType::I32(v) => builder.ins().iconst(I32, *v as i64),
                LiteralType::I16(v) => builder.ins().iconst(I16, *v as i64),
                LiteralType::I8(v) => builder.ins().iconst(I8, *v as i64),
                _ => panic!("Literal type not handled yet!")
            }
        },
        Expr::VariableRef { var } => {
            builder.use_var(var_map.get_var(&var).expect("Variable has not been declared yet!"))
        },
        Expr::Binary { op, lhs, rhs } => {
            let arg1 = build_expr(builder, lhs, var_map);
            let arg2 = build_expr(builder, rhs, var_map);

            //TODO: Only handles integer math
            match op {
                BinaryOp::Add => builder.ins().iadd(arg1, arg2),
                BinaryOp::Sub => builder.ins().isub(arg1, arg2),
                BinaryOp::Mul => builder.ins().imul(arg1, arg2),
                // BinaryOp::Div => builder.ins().idiv(arg1, arg2),
                BinaryOp::Div => panic!("Integer division not supported yet!")
            }
        },
        Expr::Missing => panic!("Expression is missing!"),
        _ => panic!("Expr case not handled yet!")
    }
}
