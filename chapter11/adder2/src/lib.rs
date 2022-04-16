pub fn add_two(a: i32) -> i32 {
    internal_addr(a, 2)
}

fn internal_addr(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_addr(2, 2));
    }
}
