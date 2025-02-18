fn main() {
    #[derive(Debug)]
    struct Vec2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let v = Vec2 { x: 4, y: 2, z: 0 };
    let Vec2 { x, y, z } = v; // Structs as well as Tuples can be destructered like this.
    println!("Value of x is {:?}", v);

    let Vec2 { x, .. } = v; // Values can also be dropped while destructering like here y and z's values are dropped.
    println!("The value of Vec2 is {:?}", x);
}
