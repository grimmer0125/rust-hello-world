use serde_json; //::{Result, Value};

use std::collections::HashMap;
use std::io;

use serde::{Deserialize, Deserializer};

// serialize_with ref:
// https://serde.rs/field-attrs.html
// https://serde.rs/string-or-struct.html
// https://stackoverflow.com/questions/44331037/how-can-i-distinguish-between-a-deserialized-field-that-is-missing-and-one-that
// https://github.com/serde-rs/json/issues/167
// https://github.com/serde-rs/json/issues/317
// https://github.com/serde-rs/json/issues/513
// https://github.com/serde-rs/json/issues/45
// https://stackoverflow.com/questions/52611244/how-can-i-deserialize-a-type-where-all-the-fields-are-default-values-as-a-none-i
// https://users.rust-lang.org/t/serde-deserialize-custom-implementation/17124/4

// TODO: it is not called actually !!!
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(None)
}

#[derive(Deserialize)]
struct Person {
    // #[serde(default)]
    // #[serde(deserialize_with = "deserialize_optional_field")]
    name: Option<String>, //Option<String>, //String,
                          // age: u8,
                          // phones: Vec<String>,
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
    println!("person name: {:?} ", p.name);

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
