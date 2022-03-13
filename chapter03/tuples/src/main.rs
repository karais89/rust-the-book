fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    // 인덱스 접근
    println!("첫번째 값 {}", tup.0);
    // 변수 지정 접근
    println!("y의 값 {}", y);
}
