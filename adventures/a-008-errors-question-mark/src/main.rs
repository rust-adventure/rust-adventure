// This code returns a Result from main():
// either Ok(()) or Err(error)
// Refactor this code so that it does not
// use match to return the error. Try to
// find a shorter way to do the same thing.
fn main() -> Result<(), std::num::ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    print!("{}", number);
    Ok(())
}
