fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn it_not_works() {
        assert_eq!(1 + 1, 3);
    }
}
