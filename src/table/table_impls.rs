use crate::{
    column::{Column, ColumnId, VecColumn},
    rows::{
        row::{ImmutableRows, MutableRows},
        ImmutVecRows, MutVecRows,
    },
};

use super::Table;

pub struct VecTable {
    // columns: Vec<Box<dyn Column<'a>>>,
    columns: Vec<VecColumn>,
}

impl<'a> Table<'a> for VecTable {
    type ColumnType = VecColumn;

    fn new() -> Self {
        VecTable {
            columns: Vec::new(),
        }
    }

    fn new_with(columns: Vec<Self::ColumnType>) -> Self {
        VecTable { columns }
    }

    fn add_column(&mut self, column: Self::ColumnType) -> &mut Self {
        self.columns.push(column);
        self
    }

    fn get_column<'s, T>(&'a self, column: T) -> &Self::ColumnType
    where
        T: Into<ColumnId<'s>>,
    {
        match column.into() {
            ColumnId::Index(i) => &self.columns[i],
            ColumnId::Name(n) => self.columns.iter().find(|c| c.name() == n).unwrap(),
        }
    }

    fn get_column_mut<'s, T>(&'a mut self, column: T) -> &mut Self::ColumnType
    where
        T: Into<ColumnId<'s>>,
    {
        match column.into() {
            ColumnId::Index(i) => &mut self.columns[i],
            ColumnId::Name(n) => self.columns.iter_mut().find(|c| c.name() == n).unwrap(),
        }
    }

    fn rows(&'a self) -> impl ImmutableRows<'a> {
        let column_iters = self.columns.iter().map(|c| c.iter()).collect();
        let names = self.columns.iter().map(|c| c.name().to_owned()).collect();
        ImmutVecRows::new(column_iters, names)
    }

    fn rows_mut(&'a mut self) -> impl MutableRows<'a> {
        let names = self.columns.iter().map(|c| c.name().to_owned()).collect();
        let column_iters = self.columns.iter_mut().map(|c| c.iter_mut()).collect();
        MutVecRows::new(column_iters, names)
    }
}
