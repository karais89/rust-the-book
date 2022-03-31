use std::fmt::format;

fn main() {
    // 새로운 빈 문자열 생성
    let mut s = String::new();

    // 문자열 리터럴의 to_string 메서드를 이용해서 문자열 생성
    let data = "문자열초깃값";
    let s = data.to_string();
    let s = "문자열초깃값".to_string();
    // String::from 함수를 이용해 문자열 리터럴에서 String 타입 생성
    let s = String::from("문자열초깃값");
    // push_str 메서드를 이용해 string에 문자열 슬라이스를 덧붙이기
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    // String 타입에 변수의 데이터를 덧붙인 후 해당 변수를 다시 사용
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);

    // 문자열 연결
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 이렇게 하면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다.
    println!("{}", s3);

    // format 매크로
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // 순회 사용
    for c in "안녕하세요".chars() {
        println!("{}", c);
    }

    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }    
}
