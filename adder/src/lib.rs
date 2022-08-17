pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        println!("I am exploration");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // #[should_panic]
    #[ignore]
    fn other() {
        println!("I am other");
        let a = 3 + 1;
        assert_eq!(3 + 1, 3, "The Result is error, result was {}", a);
    }
}
