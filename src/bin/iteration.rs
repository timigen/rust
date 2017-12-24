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

    println!("\nloop over int vector [1, 10, 20]");
    let numbers = vec![1, 10, 20];
    for number in numbers.iter() {
        match number {
            & 10 => println!("ten!"),
            _ => println!("{} is NOT ten", number),
        }
    }

    println!("\nloop over string vector ['apple', 'banana', 'pear']");
    let fruits = vec!["apple", "banana", "pear"];
    for fruit in fruits.iter() {
        match fruit {
            & "banana" => println!("banana!"),
            _ => println!("{} is NOT a banana", fruit),
        }
    }
}
