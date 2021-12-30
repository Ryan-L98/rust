pub fn run() {
	let mut hello = String::from("Hello");

	hello.push('v');
	
	println!("{} has {} characters and its capacity is {}", hello, hello.len(), hello.capacity());

	let chars : Vec<char> = hello.chars().collect();
	for charac in chars {
		println!("{}", charac);
	}

}