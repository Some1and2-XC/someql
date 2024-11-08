use std::{collections::HashMap, fmt::{Debug, Display}, panic::UnwindSafe, sync::Arc};

// use super::Result;

struct AutoAble {}

/// Trait that is implemented on types that are used
pub trait Primitive: Debug + Display + UnwindSafe{

    fn get_next(&self) -> i128;
    fn autoable(&self) -> bool;

}

impl Primitive for bool {

    fn autoable(&self) -> bool { return false; }
    fn get_next(&self) -> i128 { panic!("Can't get next value from: {:?}!", self); }

}

impl Primitive for String {

    fn autoable(&self) -> bool { return false; }
    fn get_next(&self) -> i128 { panic!("Can't get next value from: {:?}!", self); }

}

impl Primitive for isize {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128{ return *self as i128 + 1; }

}
impl Primitive for usize {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128 { return *self as i128 + 1; }

}

impl Primitive for i32 {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128 { return *self as i128 + 1; }

}
impl Primitive for u32 {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128 { return *self as i128 + 1; }

}

impl Primitive for i64 {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128 { return *self as i128 + 1; }

}
impl Primitive for u64 {

    fn autoable(&self) -> bool { return true; }
    fn get_next(&self) -> i128 { return *self as i128 + 1; }

}

pub struct Table {

    pub alias: String,
    /// The inner values of the table. The `String` in the HashMap references the
    /// name of the table column. The TableValueContainer stores the actual data.
    pub value: HashMap<String, Arc<dyn ColTypeErased>>,
    size: usize,

}

impl Table {

    /// This method creates an instance of a new table.
    pub fn new(alias: String, value: HashMap<String, Arc<dyn ColTypeErased>>) -> Self {
        return Self {
            alias,
            value,
            size: 0,
        };
    }

    /// Method for inserting a row into the database.
    /// Takes the keys from the `cols` value and adds the `values` values.
    pub fn insert_row(&mut self, cols: Vec<String>, values: Vec<Arc<dyn ColTypeErased>>) -> Result<(), ()> {

        let all_keys: Vec<&String> = self.value.keys().collect();

        for k in cols {


        }


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
    /// Method for setting data at an index.
    fn set_data(&mut self, idx: usize, data: &dyn Primitive) -> Result<(), ()>;
    /// Method for pushing the data to the end of the column.
    fn push(&mut self, data: &dyn Primitive) -> Result<(), ()>;
}

impl Debug for dyn ColTypeErased {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.to_string());
    }
}

impl<T: ColType> ColTypeErased for T {

    fn get_data(&self) -> Vec<&dyn Primitive> {
        return ColType::get_data(self)
            .iter()
            .map(|v| v as &dyn Primitive)
            .collect();
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

    fn set_data(&mut self, idx: usize, data: &dyn Primitive) -> Result<(), ()> {
        todo!()
    }

    fn push(&mut self, data: &dyn Primitive) -> Result<(), ()> {
        todo!()
    }

}

/// A container interface for a table value.
pub trait ColType {

    type Item: Primitive;

    fn get_data(&self) -> &Vec<Self::Item>;
    fn is_unique(&self) -> bool;
    fn is_primary_key(&self) -> bool;
    fn get_size(&self) -> usize;

    fn set_unique(&mut self, value: bool) -> ();
    fn set_primary_key(&mut self, value: bool) -> ();

    fn set_data(&mut self, idx: usize, data: Self::Item) -> Result<(), ()>;
    fn push(&mut self, value: Self::Item) -> ();

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


impl <T: Primitive + Display + std::panic::UnwindSafe> ColType for TableValue<T> {

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

    fn set_unique(&mut self, value: bool) -> () {
        self.unique = value;
    }

    fn set_primary_key(&mut self, value: bool) -> () {
        self.primary_key = value;
    }

    fn set_data(&mut self, idx: usize, data: T) -> Result<(), ()>{

        return match self.data.get_mut(idx) {
            Some(v) => {
                *v = data;
                Ok(())
            },
            None => Err(()),
        };

    }

    fn push(&mut self, data: T) -> () {

        self.data.push(data);

    }

}
