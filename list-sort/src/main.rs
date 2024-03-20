use rand::Rng;

fn rand_int(min: i32, max: i32) -> i32
{
    rand::thread_rng().gen_range(min..max)
}

fn main() {

    let value: i32 = rand_int(0, 10);

    println!("{value}");
}
