use rand::Rng;

fn rand_int(min: i32, max: i32) -> i32
{
    rand::thread_rng().gen_range(min..max)
}

fn max(vec: &Vec<i32>) -> i32
{
    let mut max: i32 = vec[0];
    for &value in vec
    {
        if value > max
        {
            max = value
        }
    }
    max
}

fn main() {

    println!("-- Sort --");

    let mut seq: Vec<i32> = Vec::new();
    let length = 24;
    let lower_bound = 0;
    let upper_bound = 100;

    // Populate vector with random values
    for _i in 0..length
    {
        seq.push(rand_int(lower_bound, upper_bound));
    }

    // get max 
    let max: i32 = max(&seq);
    
    println!("before: {:?}", seq);
    println!("max: {max}");

    seq.sort();
    
    println!("after: {:?}", seq);

}








