fn main() {
    // loop {
    //     println!("다시 실행!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is = {}", result);

    // for
    let a = [10, 20, 30, 40,  50];
    for element in a.iter() {
        println!("요소의값: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}
