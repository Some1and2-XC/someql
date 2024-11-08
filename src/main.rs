use std::{collections::HashMap, sync::Arc};

use someql::library::*;
use tables::{Primitive, TableValue, ColType};

fn main() {

    // let k = tables::TableKey::Single(Arc::new(52i32));

    let mut table = tables::Table::new(
        "Users".into(),
        HashMap::new(),
    );

    table.insert_col("id".into(), Arc::new(TableValue {
        data: vec![
            0i32,
            1i32,
        ],
        unique: true,
        primary_key: true,
        auto: true,
    })).unwrap();

    table.insert_col("f_name".into(), Arc::new(TableValue {
        data: vec![
            "Hello".to_string(),
            "Hi".to_string(),
        ],
        unique: false,
        primary_key: false,
        auto: false,
    })).unwrap();

    let input = r"#
        CREATE TABLE Users (

            id INEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            username VARCHAR(255),

        );
    #";

    for v in table.value.values() {
        println!("{}", v.to_string());
    }

    let txt = "Hello";

    let tn = ast::TreeNode::new(Arc::new(txt));
    println!("Value: {:?}", tn.data);


}
