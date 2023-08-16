fn main() {
    let mut multiplier = 1;

    println!("Multiplication Table of 9:");
    while multiplier <= 10 {
        let result = 9 * multiplier;
        println!("9 * {} = {}", multiplier, result);
        multiplier += 1;
    }
}
