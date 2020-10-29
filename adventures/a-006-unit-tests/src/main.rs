fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    print!("{}", add(2, 4));
}

// Tests are indicated by the test annotation
// so that the test runner knows to run them
// and are otherwise regular functions
#[test]
fn it_works() {
    // We use the assert_eq macro to compare the result
    // of our function to the test value
    assert_eq!(add(2, 4), 6);
    // TODO: update this test so that it works
    assert_eq!(add(4, 8),);
}
