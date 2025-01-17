use std::io;
//std - Standard Library : using the library
//io - Input/Output - module

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("{}", input);
}
