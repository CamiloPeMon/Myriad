fn main() {
    inquire::Select::new("What is your favorite color?", vec!["Red", "Blue", "Green"])
        .prompt()
        .unwrap();
    println!("Hello, world!");
}
