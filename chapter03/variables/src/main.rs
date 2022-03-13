const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    println!("상수 값: {}", MAX_POINTS);

    // shadowed
    let x = x + 1;
    let x = x * 2;    
    println!("x의 값: {}", x);
}