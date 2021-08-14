use parser::parse;

fn main() {
    let code = include_str!("../test.glue");
    let root = parse(code);
    println!("Debug tree:\n{}\n", root.debug_tree());

    
}
