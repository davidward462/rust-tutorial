fn main()
{
    // ways to create a string
    
    // string literal
    let phrase = "hello there";

    let s = phrase.to_string();
    
    let t = String::from("zeta");

    // appending to string
    let mut u = String::from("One");
    
    // takes string slice as parameter
    u.push_str("Two");

    // takes a single character
    u.push('X');

    println!("{s}\n{t}\n{u}");

    // concatenate existing strings
    let first = String::from("Bob");
    let last = String::from("Smith");
    let full = first + &last;
    println!("{full}");
    // note that 'first' is no longer valid
    
    // format macro
    let a = String::from("Alpha");
    let b = String::from("Beta");
    let c = String::from("Gamma");
    let d = format!("{a} {b} {c}");
    println!("{d}");

}
