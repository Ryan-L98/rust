pub fn run() {
	let person: (&str , &str, i8) = ("Ryan" , "here", 23);

	println!("{} is from {} and is {}", person.0, person.1, person.2)
}