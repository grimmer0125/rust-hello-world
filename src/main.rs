use serde_json; //::{Result, Value};

use std::collections::HashMap;
use std::io;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct Person {
    name: Option<String>, //String,
    age: Option<u8>,      // phones: Vec<String>
}

// let data = r#"
//     {
//         "name": "John Doe",
//         "age": 43,
//         "phones": [
//             "+44 1234567",
//             "+44 2345678"
//         ]
//     }"#;
fn serde_typed_custom_example() -> serde_json::Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe"
        }"#;

    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("person name: {:?}, age:{:?} ", p.name, p.age);

    Ok(())
}

fn main() {
    println!("start");

    serde_typed_custom_example();

    // match untyped_example() {
    //     Ok(_) => {
    //         println!("one");
    //     }
    //     Err(error) => {
    //         println!("The error: {}", error);
    //     }
    // }

    println!("end");
}
