fn get_thing() {
    println!("The value of X is an Idiot");
}

fn main() {
    let _ = get_thing(); // -> Here gething() function is called but "_" make sure that we don't need the return value of the function. "_" comes handy in these types of situations wehere we need to avoid side effects from the compiler.
                         // Other examples could be: When only need to call a function where the function will only read a file or click a button. We don't need the return value of these function generally.
}
