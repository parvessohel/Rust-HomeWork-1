fn idiot(x: String) -> String {
    let size = x.len().to_string(); // We can also call a function with a dot like here.
    size
}

fn main() {
    let x = (10, 11);
    println!("The first value of x is {}", x.0); // This is how we access a field value in Rust using dot.
    let sopa: String = "quickerthancoffee".to_string();
    let size = idiot(sopa);
    println!("Length of Sopa is {}", size)
}
