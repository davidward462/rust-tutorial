fn print_vector(v: Vec<i32>)
{
    // print entries in vector
    for i in &v
    {
        print!("{i} ");
    }
    println!("");
}

fn main()
{
    let mut v:Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);

    let mut u:Vec<i32> = Vec::new();
    u.push(1);
    u.push(2);
    u.push(3);
    u.push(4);
    u.push(5);
    u.push(6);

    print_vector(v);

    // update entries in vector
    for i in &mut u
    {
        *i = *i * 10;
        print!("{i} ");
    }

    println!("");

}
