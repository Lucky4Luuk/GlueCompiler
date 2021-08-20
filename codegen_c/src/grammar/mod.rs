mod func;
pub use func::*;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

pub fn type_to_c_type(kind: &str) -> &str {
    match kind {
        "u8"  => "uint8_t",
        "u16" => "uint16_t",
        "u32" => "uint32_t",
        "u64" => "uint64_t",

        "i8"  => "int8_t",
        "i16" => "int16_t",
        "i32" => "int32_t",
        "i64" => "int64_t",

        _ => kind
    }
}
