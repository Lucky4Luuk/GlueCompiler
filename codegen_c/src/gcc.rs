use std::process::Command;

use crate::grammar::build_func;

pub fn compile(triple: String, hir: hir::Root) {
    let mut result = String::new();
    result.push_str("#include <stdint.h>\n");
    for hir_func in &hir.functions {
        let func_string = build_func(hir_func);
        result.push_str(&func_string);
        result.push_str("\n");
    }

    println!("{}", &result);

    //Save to disk
    std::fs::create_dir("c_target");
    std::fs::write("c_target/output.c", result);

    //Call gcc
    let output = Command::new("gcc").args(&["-o", "output", "c_target/output.c"]).output().expect("Failed to run command gcc!");
    // println!("GCC output:\n{:?}", output);
    if output.status.success() {
        println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    } else {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
        //TODO: Handle errors
    }
}
