use parser::parse;

fn main() {
    println!("Hello, world!");

    let code = include_str!("../test.glue");
    let root = parse(code);
    println!("Debug tree:\n{}", root.debug_tree());
}
