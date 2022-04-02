use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;

fn main() {
    println!("Hello, world!");
}

// 이미 제공하는 함수
fn read_username_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 더 짧게 사용 가능
fn read_username_file3() -> Result<String, io::Error> {
    let mut s = String::new();    
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// ? 연산자를 이용한 에러 처리
fn read_username_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 일반적 에러 처리
fn read_username_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}