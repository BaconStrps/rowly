use crate::{
    column::ColumnId,
    rows::row::{ImmutableRows, MutableRows},
};

pub trait Table<'a> {
    type ColumnType;
    fn new() -> Self;

    fn new_with(columns: Vec<Self::ColumnType>) -> Self;

    fn add_column(&mut self, column: Self::ColumnType) -> &mut Self;

    fn get_column<'s, T>(&'a self, column: T) -> &Self::ColumnType
    where
        T: Into<ColumnId<'s>>;

    fn get_column_mut<'s, T>(&'a mut self, column: T) -> &mut Self::ColumnType
    where
        T: Into<ColumnId<'s>>;

    fn rows(&'a self) -> impl ImmutableRows<'a>;

    fn rows_mut(&'a mut self) -> impl MutableRows<'a>;
}
