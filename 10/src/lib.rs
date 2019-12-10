fn ten() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let out = ten();
        assert_eq!(1, out);
    }
}
