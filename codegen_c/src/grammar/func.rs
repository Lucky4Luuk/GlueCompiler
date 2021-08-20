use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn build_signature(hir_func: &hir::func::Func) -> String {
    //TODO: Handle visibility
    let mut hasher = DefaultHasher::new();
    let name = &hir_func.name;
    name.hash(&mut hasher);
    let hashed_name = hasher.finish();
    let args = match &hir_func.args {
        None => String::new(),
        Some(args) => {
            let mut tmp = String::new();
            let mut i = 0;
            let l = args.args.len();
            for arg in &args.args {
                tmp.push_str(super::type_to_c_type(&arg.0));
                tmp.push_str(" ");
                tmp.push_str(&arg.1);
                if i < l-1 {
                    tmp.push_str(", ");
                    i += 1;
                }
            }
            tmp
        }
    };

    let return_type = match &hir_func.ret_args {
        None => "void".to_string(),
        Some(ret_args) => {
            match ret_args {
                hir::func::FuncReturnArgs::MultiReturn(ret_args) => {
                    let hashed_ret_name = format!("{}_ret_struct", hashed_name);
                    let return_struct = format!(r#"struct {} {{
    int32_t a;
}};"#, hashed_ret_name);
                    hashed_ret_name
                },
                hir::func::FuncReturnArgs::SingleReturn(kind) => super::type_to_c_type(kind).to_string()
            }
        }
    };

    let signature = format!("{} {}({})", return_type, name, args);
    dbg!(&signature);
    signature
}

pub fn build_func(hir_func: &hir::func::Func) -> String {
    let func_signature = build_signature(hir_func);
    let mut func = String::from(&func_signature);
    func.push_str("{\n");

    for hir_stmt in &hir_func.code_block.stmts {
        func.push_str(&super::build_stmt(&hir_stmt));
        func.push_str("\n");
    }

    func.push_str("}\n");

    func
}
