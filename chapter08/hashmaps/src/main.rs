use std::collections::HashMap;

fn main() {
    // 새로운 해시 맵 생성
    let mut scores = HashMap::new();
    
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    let field_name = String::from("Favorite color");
    scores.insert(field_name, 50); 
    // 소유권이 넘어가서 에러
    // println!("{} {}", field_name, 50);
    
    // 값 가져오기 Option 타입을 넘겨준다
    let team_name = String::from("블루");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());

    // for 순회 - 소유권을 넘기지 않기 위해 &
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }    

    // 값 덮어쓰기
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("블루"), 25);
    println!("{:?}", scores);

    // 키에 값이 없을때만 추가하기
    scores.entry(String::from("블루")).or_insert(100);
    scores.entry(String::from("yellow")).or_insert(100);
    println!("{:?}", scores);

    // 기존 값에 따라 값 수정하기
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
