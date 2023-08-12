fn main() {
    let s1 = String::from("hello");
    // let s2 = s1; || bad because s1 will be not exist
    // we need add s1.clone to save s1
    let s2 = s1.clone();
    println!("{s1}{s2}"); //hellohello
}
