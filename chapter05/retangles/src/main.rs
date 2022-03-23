use std::sync::mpsc::Receiver;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // 일반 변수를 사용
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area(width1, height1)
    );

    // 튜플을 사용
    let rect = (30, 50);
    
    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area_tuple(rect)
    );

    // 구조체를 이용
    let rect1 = Rectangle { width: 30, height: 50};
    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area_struct(&rect1)
    );

    println!("{:?}", rect1);

    // 메서드 사용
    println!(
        "사각형의 면적: {} 제곱 픽셀",
        rect1.area()
    );

    // 더 많은 매개변수
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    let rect1 = Rectangle::square(30);
    println!("{:?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}