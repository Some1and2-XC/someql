use std::{collections::HashMap, fmt::{Debug, Display}, sync::Arc};

// use super::Result;

/// Trait that is implemented on types that are used
pub trait Primitive: Debug + Display { }

impl Primitive for bool { }

impl Primitive for String { }

impl Primitive for isize { }
impl Primitive for usize { }

impl Primitive for i32 { }
impl Primitive for u32 { }

impl Primitive for i64 { }
impl Primitive for u64 { }

pub struct Table {

    pub alias: String,
    /// The inner values of the table. The `String` in the HashMap references the
    /// name of the table column. The TableValueContainer stores the actual data.
    pub value: HashMap<String, Arc<dyn ColTypeErased>>,
    size: usize,

}

impl Table {

    pub fn new(alias: String, value: HashMap<String, Arc<dyn ColTypeErased>>) -> Self {
        return Self {
            alias,
            value,
            size: 0,
        };
    }

    /// Method for inserting a row into the database.
    pub fn insert_row(&mut self, cols: Vec<String>, values: Vec<Arc<dyn ColTypeErased>>) -> Result<(), ()> {

        return Ok(());

    }

    pub fn insert_col(&mut self, k: String, v: Arc<dyn ColTypeErased>) -> Result<(), Arc<dyn ColTypeErased>> {

        if self.value.len() == 0 {
            self.size = v.get_size();
        }
        // Ensures that the size of each column is the same.
        else if self.size != v.get_size() {
            return Err(v);
        }

        // Checks to see if the value already exists
        if self.value.contains_key(&k) {
            return Err(v);
        }

        // Checks the result just in case (this should never happen)
        if self.value.insert(k, v).is_some() {
            panic!("Something went critically wrong! IDK, ya done goofed.");
        };

        return Ok(());

    }

    /*
    pub fn to_string(&self) -> String {

        let mut out_str = self.alias.clone();

        out_str += "\n";

        self.value.keys()

    }
    */

}

pub enum TableKey<T: Primitive> {
    Single(T),
    Candidate(Vec<T>),
}

pub trait ColTypeErased {
    /// Method for getting the underlying data from the table.
    fn get_data(&self) -> Vec<&dyn Primitive>;
    /// Method for getting a string from the underlying data.
    fn to_string(&self) -> String;
    /// Method for getting the length of a column.
    fn get_size(&self) -> usize;
}

impl Debug for dyn ColTypeErased {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.to_string());
    }
}

impl<T: ColType> ColTypeErased for T {

    fn get_data(&self) -> Vec<&dyn Primitive> {
        return ColType::get_data(self).iter().map(|v| v as &dyn Primitive).collect();
    }

    fn to_string(&self) -> String {

        let out_str = self.get_data()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        return format!("[{}]", out_str);

    }

    fn get_size(&self) -> usize {
        return self.get_size();
    }

}

/// A container interface for a table value.
pub trait ColType {

    type Item: Primitive;

    fn get_data(&self) -> &Vec<Self::Item>;
    fn is_unique(&self) -> bool;
    fn is_primary_key(&self) -> bool;
    fn get_size(&self) -> usize;

}

// We want to use the <T: _> syntax here because we do want static dispatch (I think?)
// This means static dispatch for each table vec and dyn disp for the table itself.
/// This struct represents a column in a table.
pub struct TableValue<T: Primitive> {

    /// This value represents the inner data of the column.
    pub data: Vec<T>,
    /// This flag decides if the column should be unique or not.
    pub unique: bool,
    pub primary_key: bool,
    pub auto: bool,

}


impl <T: Primitive + Display> ColType for TableValue<T> {

    type Item = T;

    fn get_data(&self) -> &Vec<T> {
        return &self.data;
    }

    fn is_unique(&self) -> bool {
        return self.unique;
    }

    fn is_primary_key(&self) -> bool {
        return self.primary_key;
    }

    fn get_size(&self) -> usize {
        return self.data.len();
    }

}
