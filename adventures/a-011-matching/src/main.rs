fn print_num(num: u8) {
    match num {
        0 => print!("one"),
        1 => print!("two"),
        2 => print!("three"),
        3 => print!("four"),
        4 => print!("five"),
        n => print!("Can't format num {}", n),
    }
}
fn main() {
    for num in 0..5 {
        print_num(num);
    }
}
