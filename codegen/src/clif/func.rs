use std::collections::HashMap;

use cranelift::codegen::entity::EntityRef;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::ir::{AbiParam, ExternalName, Function, FuncRef, InstBuilder, Signature};
use cranelift::codegen::settings;
use cranelift::codegen::isa::CallConv;
use cranelift::codegen::verifier::verify_function;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext, Variable};

use cranelift_object::ObjectModule;

use super::stmt::build_stmt;
use super::variable::{VariableMap, temp_func_name_map, temp_type_map};

pub struct FuncRefMap {
    func_lut: HashMap<usize, String>,
    funcs: HashMap<String, FuncRef>,
    next_idx: usize,
}

impl FuncRefMap {
    pub fn new() -> Self {
        Self {
            func_lut: HashMap::new(),
            funcs: HashMap::new(),
            next_idx: 0,
        }
    }

    pub fn add_func(&mut self, name: &str, func: FuncRef) {
        self.func_lut.insert(self.next_idx, name.to_string());
        self.funcs.insert(name.to_string(), func);
        self.next_idx += 1;
    }

    pub fn get_func(&self, name: &str) -> Option<FuncRef> {
        match self.funcs.get(name) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

pub fn create_func(flags: settings::Flags, call_conv: CallConv, hir_func: &hir::func::Func) -> Function {
    use hir::func::FuncReturnArgs;

    let mut sig = Signature::new(call_conv);
    if let Some(args) = &hir_func.args {
        dbg!(hir_func);
        panic!("Function arguments are not yet supported!");
    }
    if let Some(ret_args) = &hir_func.ret_args {
        match ret_args {
            FuncReturnArgs::SingleReturn(var_type_string) => {
                let var_type = temp_type_map(&var_type_string);
                sig.returns.push(AbiParam::new(var_type));
            },
            FuncReturnArgs::MultiReturn(ret_args_vec) => {
                todo!();
            }
        }
    }
    let fname = temp_func_name_map(&hir_func.name);
    Function::with_name_signature(fname, sig)
}

pub fn build_func(flags: settings::Flags, call_conv: CallConv, func: &mut Function, hir_func: &hir::func::Func, module: &ObjectModule) {
    //Our signature is now constructed, so we can continue to actual function building!
    let mut fn_builder_ctx = FunctionBuilderContext::new();
    let fname = temp_func_name_map(&hir_func.name);

    {
        let mut builder = FunctionBuilder::new(func, &mut fn_builder_ctx);

        //Block: https://en.wikipedia.org/wiki/Basic_block
        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);

        // let mut scope_var_lut: HashMap<usize, String> = HashMap::new();
        // let mut scope_vars: HashMap<String, Variable> = HashMap::new();
        // let mut scope_var_next_idx: usize = 0;
        let mut scope_vars = VariableMap::new();

        builder.switch_to_block(block0);
        // builder.seal_block(block0);
        /*
        {
            let _ = builder.block_params(block)[0]; //get the first argument passed (TODO)
        }
        */

        'stmts: for hir_stmt in &hir_func.code_block.stmts {
            if build_stmt(&mut builder, hir_stmt, &mut scope_vars, module) { break 'stmts; }
        }

        builder.seal_all_blocks();
        builder.finalize();
    }

    let res = verify_function(&func, &flags);
    println!("{}", func.display(None));
    if let Err(errors) = res {
        panic!("{}", errors);
    }
}
