use std::sync::Arc;

use someql::library::*;

fn main() {


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
