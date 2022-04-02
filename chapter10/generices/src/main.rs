use core::num;


fn main() {
    // 주어진 리스트에서 가장 큰 숫자를 찾는 프로그램
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let list = &number_list[..];
    for i in list {
        println!("{}", i);
    }
}

// i32 슬라이스라면 어떤 타입이든 전달할 수 있다.
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
