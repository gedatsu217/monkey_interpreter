fn main() {
    println!("Hello, world!");
    let s = String::from("Hello");
    let a = 1;
    let slice = &s[0..a];
    println!("{}", slice);

}
