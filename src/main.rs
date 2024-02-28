fn main() {
    let s = "Hello";
    println!("The string is: {}, with a length of: {}", &s, find_length(&s));
}

fn find_length(s: &str)->usize{
    s.len()
}
