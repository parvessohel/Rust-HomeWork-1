fn main() {
    #[derive(Debug)]
    struct Vec2 {
        // Structs are declared here.
        x: i32,
        y: i32,
        z: i32,
    }
    let x1 = Vec2 { x: 4, y: 2, z: 0 }; // Here Structs are initialized.
    println!("Value of x is {:?}", x1);
    let x2 = Vec2 { x: 42, ..x1 }; // We can initialize rest of the fields from another struct in this way. This is called Stuct Update Syntax
    println!("Value of x2 is {:?}", x2);
    let x3 = Vec2 { ..x2 }; // All the fields are initialized her in this way.
    println!("Value of x2 is {:?}", x3);
}
