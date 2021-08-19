#[derive(Debug)]
pub enum LiteralType {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),

    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),

    F64(f64),
    F32(f32),

    Missing,
}
