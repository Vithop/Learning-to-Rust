fn main() {
    println!("Hello, World!");
    let mut a = String::from("Mathuran");
    for _i in 1..=100 {
        a.push_str("1");
    }
    println!("{}",a);
}