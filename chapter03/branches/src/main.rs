fn main() {
    let number = 3;
    
    if number < 5 {
        println!("조건이 일치합니다. :)");
    } else {
        println!("조건이 일치하지 않습니다. :(");    
    }

    // if 표현식
    let condition = true;
    let number = if condition {
        5        
    } else {
        6
    };
    println!("number의 값: {}", number);
}
