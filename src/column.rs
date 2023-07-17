use std::{
    collections::VecDeque,
    slice::{Iter, IterMut},
};

use crate::table_data::OwnedTableData;

pub trait Column<'a> {
    fn name(&self) -> &str;

    fn data<T>(&'a self) -> impl Iterator<Item = &'a T>
    where
        OwnedTableData: AsRef<T>,
        T: 'a;

    fn data_mut<T>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        OwnedTableData: AsMut<T>,
        T: 'a;

    fn push(&mut self, data: OwnedTableData) -> Result<(), ColumnError>;

    fn remove(&mut self) -> Result<(), ColumnError>;

    fn extend(&mut self, data: Vec<OwnedTableData>) -> Result<(), ColumnError> {
        for val in data.into_iter() {
            self.push(val)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ColumnError {}

pub enum ColumnId<'a> {
    Index(usize),
    Name(&'a str),
}

impl<'a> From<&'a str> for ColumnId<'a> {
    fn from(s: &'a str) -> Self {
        ColumnId::Name(s)
    }
}

impl From<usize> for ColumnId<'_> {
    fn from(u: usize) -> Self {
        ColumnId::Index(u)
    }
}

pub struct VecColumn {
    name: String,
    data: Vec<OwnedTableData>,
}

impl VecColumn {
    pub fn new<D, V>(name: &str, data: D) -> Self
    where
        D: IntoIterator<Item = V>,
        V: Sized + Into<OwnedTableData>,
    {
        VecColumn {
            name: name.to_string(),
            data: data.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn new_boxed<D, V>(name: &str, data: D) -> Box<Self>
    where
        D: IntoIterator<Item = V>,
        V: Sized + Into<OwnedTableData>,
    {
        Box::new(VecColumn {
            name: name.to_string(),
            data: data.into_iter().map(|v| v.into()).collect(),
        })
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, OwnedTableData> {
        self.data.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, OwnedTableData> {
        self.data.iter_mut()
    }
}

impl<'a> Column<'a> for VecColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn data<T>(&'a self) -> ImmutColumnIter<'_, T>
    where
        OwnedTableData: AsRef<T>,
    {
        // Box::new(self.data.iter().map(|v| v.into_immutable_table_data()))
        ImmutColumnIter::new(self.data.iter())
    }

    fn data_mut<T>(&'a mut self) -> MutColumnIter<'_, T>
    where
        OwnedTableData: AsMut<T>,
    {
        MutColumnIter::new(self.data.iter_mut())
    }

    fn push(&mut self, data: OwnedTableData) -> Result<(), ColumnError> {
        self.data.push(data);
        Ok(())
    }

    fn remove(&mut self) -> Result<(), ColumnError> {
        self.data.pop();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ImmutColumnIter<'a, T>
where
    OwnedTableData: AsRef<T>,
{
    data: VecDeque<&'a OwnedTableData>,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for ImmutColumnIter<'a, T>
where
    OwnedTableData: AsRef<T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front().map(|v| v.as_ref())
    }
}

impl<'a, T> ImmutColumnIter<'a, T>
where
    OwnedTableData: AsRef<T>,
{
    fn new<I>(iterator: I) -> Self
    where
        I: Iterator<Item = &'a OwnedTableData>,
    {
        ImmutColumnIter {
            data: iterator.map(|v| v.into()).collect(),
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct MutColumnIter<'a, T>
where
    OwnedTableData: AsMut<T>,
{
    data: VecDeque<&'a mut OwnedTableData>,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for MutColumnIter<'a, T>
where
    OwnedTableData: AsMut<T>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front().map(|v| v.as_mut())
    }
}

impl<'a, T> MutColumnIter<'a, T>
where
    OwnedTableData: AsMut<T>,
{
    fn new<I>(iterator: I) -> Self
    where
        I: Iterator<Item = &'a mut OwnedTableData>,
    {
        MutColumnIter {
            data: iterator.collect(),
            phantom: std::marker::PhantomData,
        }
    }
}
