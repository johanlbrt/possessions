fn main() {
    let s1 = String::from("hello");

    let (s2, len) = string_lenght(s1);

    println!("{} {}",s2,len);
}

fn string_lenght(str: String) -> (String, usize) {
    let lenght = str.len();
    (str, lenght)
}