use std::{collections::HashMap, sync::Arc};

use someql::library::*;

fn main() {

    let k = tables::TableKey::Single(Arc::new(52i32));

    let table = tables::Table {
        alias: "Users".into(),
        // value: HashMap::new(),
    };


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
