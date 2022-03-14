fn main() {
    let s1 = String::from("hello");
    let len = calcuate_length(&s1);
    println!("'{}'의 길이는 {}입니다.", s1, len);

    // 가변 참조
    let mut s = String::from("hello");
    change(&mut s);
}

fn calcuate_length(s: &str) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
