pub fn test(path: &str) {
    println!("Hello, {}!", path);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
