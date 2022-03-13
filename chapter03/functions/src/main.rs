fn main() {
    println!("안녕하세요");

    another_function();
    another_function2(5);
    another_function3(5, 6);

    // 표현식
    let y = {
        let x = 3;
        x + 1
    };
    println!("y의 값: {}", y);

    // 값을 리턴하는 함수
    let x = five();
    println!("x의 값: {}", x);
}

fn another_function() {
    println!("또 다른 함수");
}

fn another_function2(x: i32) {
    println!("x의 값: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);    
}

fn five() -> i32 {
    5
}