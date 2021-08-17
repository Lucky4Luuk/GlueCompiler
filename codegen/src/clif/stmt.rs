use cranelift::prelude::types::*;
use cranelift::prelude::FunctionBuilder;
use cranelift::prelude::InstBuilder;

use hir::stmt::Stmt;

use super::variable::{VariableMap, temp_type_map};

pub fn build_stmt(builder: &mut FunctionBuilder, hir_stmt: &Stmt, var_map: &mut VariableMap) -> bool {
    match hir_stmt {
        Stmt::VariableDef { name, value, type_string } => {
            var_map.add_var(name);
            let var_type = temp_type_map(type_string);
            builder.declare_var(var_map.get_var(name).expect("Unreachable!"), var_type);

            //Define the variable in the block
            let crane_value = builder.ins().iconst(I32, 5); //TODO: HARDCODED
            builder.def_var(var_map.get_var(name).expect("Unreachable!"), crane_value);

            println!("Variable definition: `{} {} = {:?}`", type_string, name, value);

            false
        },
        Stmt::Return(expr) => {
            let ret_val = builder.use_var(var_map.get_var("b").expect("Unreachable!")); //TODO: HARDCODED
            builder.ins().return_(&[ret_val]);
            true
        }
        _ => unimplemented!(),
    }
}
