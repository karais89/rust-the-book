fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5];
    let word = &s[6..11];
    let hello_world = &s[..];
    
    let mut s = String::from("hello world");
    let word = first_word_index(&s);
    s.clear();
    println!("the first word is: {}", word);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();

    println!("the first word is: {}", word);

    // 불변 참조 & 가변 참조
    let mut s = String::from("hello world!");
    let hello = &s[..5];
    // s.clear();
    println!("the first word is: {}", hello);
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}