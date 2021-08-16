use parser::parse;

fn main() {
    let code = include_str!("../func.glue");
    let root = parse(code);
    println!("Debug tree:\n{}\n", root.debug_tree());

    let ast_root = ast::Root::cast(root.syntax()).expect("Failed to parse!");
    // dbg!(ast_root.funcs().collect::<Vec<_>>());
    // dbg!(ast_root.funcs().filter_map(|func| func.name()).collect::<Vec<_>>());
    // dbg!(ast_root.stmts().filter_map(|stmt| if let ast::Stmt::VariableDef(var_def) = stmt {
    //     Some(var_def.name())
    // } else {
    //     None
    // }).collect::<Vec<_>>());
    // println!("stmts: {:?}", ast_root.stmts().collect::<Vec<_>>());
    // let names = ast_root.stmts().filter_map(|stmt| if let ast::Stmt::VariableDef(var_def) = stmt {
    //     Some(var_def.name())
    // } else {
    //     None
    // }).collect::<Vec<_>>();

    let hir_root = hir::lower(ast_root);
    dbg!(hir_root.statements);
    dbg!(hir_root.functions);
}
