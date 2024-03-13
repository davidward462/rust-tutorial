fn main() {

    let mut input_integer = 0;

    println!("Converting {input_integer} to binary...");

    let mut binary_result: Vec<i32> = Vec::new();

    if input_integer == 0
    {
        binary_result.push(0);
    }


    while input_integer > 0
    {
        let remainder = input_integer % 2;
        binary_result.push(remainder);
        input_integer = input_integer / 2;
    }

    // print vector
    for value in &binary_result
    {
        print!("{value}");
    }

    println!("");
}


