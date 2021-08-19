use cranelift::prelude::types::*;
use cranelift::prelude::FunctionBuilder;
use cranelift::prelude::InstBuilder;

use hir::expr::Expr;
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
        }
        _ => panic!("Expr case not handled yet!")
    }
}
