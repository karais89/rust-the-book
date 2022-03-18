#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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