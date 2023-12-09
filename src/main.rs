mod day7;
mod utils;

fn main() {
    println!(
        "\nAnswer for day 7 part 1 is: {:?}",
        day7::problem(false).unwrap()
    );
    println!(
        "\nAnswer for day 7 part 2 is: {:?}",
        day7::problem(true).unwrap()
    );
}
