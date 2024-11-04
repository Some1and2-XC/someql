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
    // The first dyn is just because it is a trait. Even though we are probably
    // only going to be implementing it once (TableValueContainer).
    // The second dyn is for the actual data type of the inner arr.
    // Even though we say "dyn" we don't have Vec<Box<dyn T>> which is why
    // this is as ugly as it is. We are then left with just a Vec<T>.
    // Good ol' dynamic dispatch for static dispatch I guess?
    /// The inner values of the table. The `String` in the HashMap references the
    /// name of the table column. The TableValueContainer stores the actual data.
    // Actually I think I found a trait bound that can't be fulfilled :/
    pub value: HashMap<String, Arc<dyn ColType>>,

}

pub enum TableKey<T: Primative> {
    Single(T),
    Candidate(Vec<T>),
}

/// A container interface for a table value.
trait ColType {

    fn get_data(&self) -> &Vec<impl Primative>;
    fn is_unique(&self) -> bool;
    fn is_primary_key(&self) -> bool;

}

// We want to use the <T: _> syntax here because we do want static dispatch (I think?)
// This means static dispatch for each table vec and dyn disp for the table itself.
/// This struct represents a column in a table.
pub struct TableValue<T: Primative> {

    pub data: Vec<T>,
    pub unique: bool,
    pub primary_key: bool,

}


impl <T: Primative> ColType for TableValue<T> {

    fn get_data(&self) -> &Vec<T> {
        return &self.data;
    }

    fn is_unique(&self) -> bool {
        return self.unique;
    }

    fn is_primary_key(&self) -> bool {
        return self.primary_key;
    }

}
