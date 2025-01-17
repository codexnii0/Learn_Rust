fn main() {
    let x: i32 = 100;
    println!("x is {}", x);
    //i32 is integer [default]

    let floating_point = 10.9;
    //float is f64 by [default]

    let true_or_false: bool = false;
    //boolean, can be true/false, 0/1

    let letter: char = 'h';
    //character data types

    //ps. you don't actually need to assign since the compiler automatically reads it.

    //tupple
    let tup: (i32, bool, &str) = (10, false, "koni");
    println!("{}", tup.2);
}
