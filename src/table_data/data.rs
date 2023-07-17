#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TableDataType {
    String,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OwnedTableData {
    String(String),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
