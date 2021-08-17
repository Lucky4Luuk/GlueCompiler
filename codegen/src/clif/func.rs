use cranelift::codegen::entity::EntityRef;
use cranelift::codegen::ir::types::*;
use cranelift::codegen::ir::{AbiParam, ExternalName, Function, InstBuilder, Signature};
use cranelift::codegen::isa::CallConv;
use cranelift::codegen::settings;
use cranelift::codegen::verifier::verify_function;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext, Variable};

fn temp_func_name_map(func_ident: &str) -> ExternalName {
    ExternalName::user(0,0) //should be indices into a symbol table
}

fn temp_var_map(type_ident: &str) -> Type {
    match type_ident {
        "u8"  => I8,
        "u16" => I16,
        "u32" => I32,
        "u64" => I64,

        "i8"  => I8,
        "i16" => I16,
        "i32" => I32,
        "i64" => I64,
        _ => panic!("unsupported type! attempted conversion: `{}`", type_ident)
    }
}

pub fn build_func(call_conv: CallConv, hir_func: &hir::func::Func) {
    use hir::func::FuncReturnArgs;

    let mut sig = Signature::new(call_conv);
    if let Some(args) = &hir_func.args {
        dbg!(hir_func);
        panic!("Function arguments are not yet supported!");
    }
    if let Some(ret_args) = &hir_func.ret_args {
        match ret_args {
            FuncReturnArgs::SingleReturn(var_type_string) => {
                let var_type = temp_var_map(&var_type_string);
                sig.returns.push(AbiParam::new(var_type));
            },
            FuncReturnArgs::MultiReturn(ret_args_vec) => {
                todo!();
            }
        }
    }

    //Our signature is now constructed, so we can continue to actual function building!
    let mut fn_builder_ctx = FunctionBuilderContext::new();
    let name = temp_func_name_map(&hir_func.name);
    let mut func = Function::with_name_signature(name, sig);

    let flags = settings::Flags::new(settings::builder());
    let res = verify_function(&func, &flags);
    println!("{}", func.display(None));
    if let Err(errors) = res {
        panic!("{}", errors);
    }
}
