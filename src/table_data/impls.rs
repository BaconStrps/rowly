use super::*;

impl From<&str> for OwnedTableData {
    fn from(s: &str) -> Self {
        OwnedTableData::String(s.to_string())
    }
}

impl From<String> for OwnedTableData {
    fn from(s: String) -> Self {
        OwnedTableData::String(s)
    }
}

impl From<i32> for OwnedTableData {
    fn from(i: i32) -> Self {
        OwnedTableData::I32(i)
    }
}

impl From<i64> for OwnedTableData {
    fn from(i: i64) -> Self {
        OwnedTableData::I64(i)
    }
}

impl From<f32> for OwnedTableData {
    fn from(f: f32) -> Self {
        OwnedTableData::F32(f)
    }
}

impl From<f64> for OwnedTableData {
    fn from(f: f64) -> Self {
        OwnedTableData::F64(f)
    }
}

// panicking is not a good idea here
// but i'll figure it out later
impl AsRef<f64> for OwnedTableData {
    fn as_ref(&self) -> &f64 {
        match self {
            OwnedTableData::F64(f) => f,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsRef<f32> for OwnedTableData {
    fn as_ref(&self) -> &f32 {
        match self {
            OwnedTableData::F32(f) => f,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsRef<i64> for OwnedTableData {
    fn as_ref(&self) -> &i64 {
        match self {
            OwnedTableData::I64(i) => i,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsRef<i32> for OwnedTableData {
    fn as_ref(&self) -> &i32 {
        match self {
            OwnedTableData::I32(i) => i,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsRef<String> for OwnedTableData {
    fn as_ref(&self) -> &String {
        match self {
            OwnedTableData::String(s) => s,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsRef<str> for OwnedTableData {
    fn as_ref(&self) -> &str {
        match self {
            OwnedTableData::String(s) => s,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<f64> for OwnedTableData {
    fn as_mut(&mut self) -> &mut f64 {
        match self {
            OwnedTableData::F64(f) => f,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<f32> for OwnedTableData {
    fn as_mut(&mut self) -> &mut f32 {
        match self {
            OwnedTableData::F32(f) => f,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<i64> for OwnedTableData {
    fn as_mut(&mut self) -> &mut i64 {
        match self {
            OwnedTableData::I64(i) => i,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<i32> for OwnedTableData {
    fn as_mut(&mut self) -> &mut i32 {
        match self {
            OwnedTableData::I32(i) => i,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<String> for OwnedTableData {
    fn as_mut(&mut self) -> &mut String {
        match self {
            OwnedTableData::String(s) => s,
            _ => panic!("Wrong type"),
        }
    }
}

impl AsMut<str> for OwnedTableData {
    fn as_mut(&mut self) -> &mut str {
        match self {
            OwnedTableData::String(s) => s,
            _ => panic!("Wrong type"),
        }
    }
}
