use std::slice::Iter;
use std::slice::IterMut;

use crate::table_data::OwnedTableData;

use super::row::ImmutRow;
use super::row::MutRow;

use super::row::MutableRows;
#[derive(Debug, Clone)]
pub struct ImmutVecRows<'a> {
    columns: Vec<Iter<'a, OwnedTableData>>,
    names: Vec<String>,
}

#[derive(Debug)]
pub struct MutVecRows<'a> {
    columns: Vec<IterMut<'a, OwnedTableData>>,
    names: Vec<String>,
}

impl<'a> ImmutVecRows<'a> {
    pub fn new(columns: Vec<Iter<'a, OwnedTableData>>, names: Vec<String>) -> Self {
        Self { columns, names }
    }
}

impl<'a> MutVecRows<'a> {
    pub fn new(columns: Vec<IterMut<'a, OwnedTableData>>, names: Vec<String>) -> Self {
        Self { columns, names }
    }
}

impl<'a> Iterator for ImmutVecRows<'a> {
    type Item = ImmutRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut row = Vec::new();

        for iter in &mut self.columns {
            match iter.next() {
                Some(data) => row.push(data),
                None => return None,
            }
        }
        let row = ImmutRow::new(row, self.names.clone());
        Some(row)
    }
}

impl<'a> Iterator for MutVecRows<'a> {
    type Item = MutRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut row = Vec::new();

        for iter in &mut self.columns {
            match iter.next() {
                Some(data) => row.push(data),
                None => return None,
            }
        }
        let row = MutRow::new(row, self.names.clone());
        Some(row)
    }
}
