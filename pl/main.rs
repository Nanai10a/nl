extern {
    fn entry() -> i32;
}

fn main() {
    unsafe {
        println!("entry(): {}", entry());
    }
}
