use std::{collections::HashMap, sync::Arc};

use someql::library::*;
use tables::{Primative, TableValue, TableValueContainer};

fn main() {

    let k = tables::TableKey::Single(Arc::new(52i32));

    let mut table = tables::Table {
        alias: "Users".into(),
        value: HashMap::new(),
    };

    let table_col = Arc::new(TableValue {
        data: vec![0i32],
        unique: false,
        primary_key: true,
    });

    table.value.insert("id".into(), table_col);


    let input = r"#
        CREATE TABLE Users (

            id INEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            username VARCHAR(255),

        );
    #";

    let txt = "Hello";

    let tn = ast::TreeNode::new(Arc::new(txt));
    println!("Value: {:?}", tn.data);


}
