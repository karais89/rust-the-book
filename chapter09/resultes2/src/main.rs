use std::fs::File;

fn main() {
    // let f = File::open("Hello.txt").unwrap();
    let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
}
