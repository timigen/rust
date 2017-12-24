pub fn loops() {
    println!("for loop");


    println!("simple for loop prints 0 - 9 to console");
    for x in 0..10 {
        println!("{}", x);
    }

    println!("infinite loop");
    loop {
        println!("infinite loop - break" );
        break;
    }
}
