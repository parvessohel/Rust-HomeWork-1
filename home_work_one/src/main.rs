fn sh1kari(stupid_char: String, stupid_number: i32) -> (String, i32) {
    println!("{}{}", stupid_char, stupid_number);
    (stupid_char, stupid_number) // Returning a Tuple.
}

fn main() {
    let (stupid_char, stupid_number) = ('x', 42); // Tuples are Destructured and assigned to its corresponding variables.
    println!("{} {}", stupid_char, stupid_number);

    let (stupid_char_one, stupid_number_1) = sh1kari(stupid_char.to_string(), stupid_number); // You can also call a function that returns a Tuple and destructure the result.
    println!("{} {}", stupid_char_one, stupid_number_1);

    let (_, stupid_number) = (",", 42); // You can also throw away a specific value of a Tuple using "_".

    println!("{}", stupid_number);
}
