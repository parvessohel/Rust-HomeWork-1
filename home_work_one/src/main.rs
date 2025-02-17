fn main() {
    #[derive(Debug)]
    struct Vec2 {
        // Structs are declared here.
        x: i32,
        y: i32,
    }
    let x1 = Vec2 { x: 4, y: 2 }; // Here Structs are initialized.
    println!("Value of x is {:?}", x1);
}
