use std::{collections::HashMap, sync::Arc};

/// Trait that is implemented on types that are used
pub trait Primative {}


impl Primative for isize {}
impl Primative for usize {}

impl Primative for i32 {}
impl Primative for u32 {}

impl Primative for Arc<i32> {}
impl Primative for Arc<u32> {}

impl Primative for i64 {}
impl Primative for u64 {}

pub struct Table {

    pub alias: String,
    // / We want this to have various sizes behind the wide Arc pointer
    // pub value: HashMap<String, Arc<TableValue<dyn Primative>>>,

}

pub enum TableKey<T: Primative> {
    Single(T),
    Candidate(Vec<T>),
}

// We want to use the <T: _> syntax here because we do want static dispatch (I think?)
/// This struct represents a column in a table.
pub struct TableValue<T: Primative> {

    pub data: Vec<T>,
    pub unique: bool,
    pub primary_key: bool,

}
