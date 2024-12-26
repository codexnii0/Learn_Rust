fn main() {
    let mut x = 4;
    println!("x is {}", x);

    //name shadowing
    {
        let x = 2;
        println!("x is {}", x);
    }

    //now make the first x mutable so it will work
    x = 5;
    println!("x is {}", x);

    //CONSTANTS (must be all caps)
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    //constants cannot change value unlike le
}
