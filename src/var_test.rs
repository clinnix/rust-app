pub mod var_test {

    #[warn(dead_code)]
    pub fn test() {
        println!("Hello, world");
        // let x = 5;
    }

    pub fn num_str_test() {
        let a: u32 = "42".parse().expect("Not a number");
        println!("{}",a);

    }
}
