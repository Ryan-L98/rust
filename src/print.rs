pub fn run() {
    //printing
    println!("Hello from main!");

    //formatting
    println!("Format print strings: {}", "by using {}",);

    //positional arguments
    println!("Cool positional formatting, {0} {1} {1} {0}", "ping", "pong");

    //named arguments
    println!("The name of this argument is argument -> {}", argument = "I am the argument");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder for debug traits
    println!("{:?}", ("i am in a tuple that is printed out using {:?}", 12, true))
}
