use crate::{column::ColumnId, table_data::OwnedTableData};

pub trait ImmutableRows<'a>: Iterator<Item = ImmutRow<'a>> {}

pub trait MutableRows<'a>: Iterator<Item = MutRow<'a>> {}

#[derive(Debug, Clone)]
pub struct ImmutRow<'a> {
    columns: Vec<&'a OwnedTableData>,
    names: Vec<String>, // matches name to index
}

impl<'a> ImmutRow<'a> {
    pub fn new(columns: Vec<&'a OwnedTableData>, names: Vec<String>) -> Self {
        Self { columns, names }
    }

    pub fn get<'s, T, I>(&'a self, id: I) -> Option<&'a T>
    where
        OwnedTableData: AsRef<T>,
        I: Into<ColumnId<'s>>,
    {
        match id.into() {
            ColumnId::Index(i) => Some(self.columns.get(i)?.as_ref()),
            ColumnId::Name(n) => {
                let index = self.names.iter().position(|x| x == n)?;
                Some(self.columns.get(index)?.as_ref())
            }
        }
    }
}

#[derive(Debug)]
pub struct MutRow<'a> {
    columns: Vec<&'a mut OwnedTableData>,
    names: Vec<String>, // matches name to index
}

impl<'a> MutRow<'a> {
    pub fn new(columns: Vec<&'a mut OwnedTableData>, names: Vec<String>) -> Self {
        Self { columns, names }
    }

    pub fn get<'s, T, I>(&'a mut self, id: I) -> Option<&'a mut T>
    where
        OwnedTableData: AsMut<T>,
        I: Into<ColumnId<'s>>,
    {
        match id.into() {
            ColumnId::Index(i) => Some(self.columns.get_mut(i)?.as_mut()),
            ColumnId::Name(n) => {
                let index = self.names.iter().position(|x| x == n)?;
                Some(self.columns.get_mut(index)?.as_mut())
            }
        }
    }
}

impl<'a, T> ImmutableRows<'a> for T where T: Iterator<Item = ImmutRow<'a>> {}
impl<'a, T> MutableRows<'a> for T where T: Iterator<Item = MutRow<'a>> {}
