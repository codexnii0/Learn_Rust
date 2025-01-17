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

    //arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);

    //ps. the difference between tupple and array, in tupple you can assign different data types while in array you can only assign one.
}
