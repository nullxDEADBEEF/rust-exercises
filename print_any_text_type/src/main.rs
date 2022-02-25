fn info<T: std::fmt::Debug + ?Sized>(text: &T) {
    println!("{:?}", text);
}

fn main() {
    let string = String::from("Help");
    info(&string);
    info("text");
    info(&string[3..]);
}
