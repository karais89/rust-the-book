#[derive(Debug)]
enum UsState {
    Alabama, Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarte from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);   

    // if let 사용
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("{:?}주의 25센트 동전!", state);
    } else {
        count += 1;
    }
    println!("{}", count);

    // if let 사용2
    if let Some(5) = five {
        println!("five is {:?}", Some(5));
    }

    // if let 사용3 - 바인딩
    if let Some(i) = five {
        println!("five is {:?}", i);
    }

    // unwrap 사용 - 9 장에서 배움
    println!("five is {:?}", five.unwrap());    
}