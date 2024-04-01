use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Define the Elm function to add two integers
    let elm_code = r#"
module Main exposing (add)

add : Int -> Int -> Int
add x y =
    x + y
"#;

    // Write the Elm code to a file named "add.elm"
    let mut file = File::create("add.elm")?;
    file.write_all(elm_code.as_bytes())?;
    println!("Elm file generated successfully.");
    Ok(())
}
