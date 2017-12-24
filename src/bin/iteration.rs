pub fn loops() {
    println!("for loop");


    println!("for x in 0..10");
    for x in 0..10 {
        println!("{}", x);
    }

    // infinite loop
    println!("loop");
    loop {
        println!("break" );
        break;
    }
}
