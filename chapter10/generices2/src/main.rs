// i32 슬라이스라면 어떤 타입이든 전달할 수 있다.


fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // 주어진 리스트에서 가장 큰 숫자를 찾는 프로그램
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result2 = largest(&char_list);
    println!("가장 큰 문자: {}", result2);
}