fn main() {
    // 빈 벡터 생성
    let v: Vec<i32> = Vec::new();

    // 값을 포함하는 새로운 벡터 생성
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 벡터 값 읽기
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("세 번째 원소: {}", third);

    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다.")
    }

    // 벡터 순회
    for i in &v {
        println!("{}", i);
    }

    // 벡터 순회 - 가변 값 변경
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }

    // 열거자를 이용한 하나의 벡터에 다른 타입의 값 저장
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),        
    ];
    println!("{:?}", row);
    match &row[0] {
        SpreadsheetCell::Int(val) => println!("첫번째 원소의 값은 {}", &val),
        _ => ()
    };
}
