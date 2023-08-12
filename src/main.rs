fn main() {
    let anything: (u8, &str) = (42, "Hello");
    println!("{}", anything.1);
    let some_text = anything.1;
    println!("{}", some_text);
}
