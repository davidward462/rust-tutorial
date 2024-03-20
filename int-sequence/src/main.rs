use rand::Rng;

// generate random i32 between min and max arguments
fn random_int(min:i32, max:i32) -> i32
{
    rand::thread_rng().gen_range(min..max)
}

fn average(vec: &Vec<i32>) -> f32
{
    let mut sum:f32 = 0.0;

    // calculate average of vector
    for i in vec
    {
        sum += *i as f32;
    }

    sum / (vec.len() as f32)
}

fn main() {

    let mut int_seq: Vec<i32> = Vec::new();

    let length = 10;
    for _i in 1..=length
    {
        let value:i32 = random_int(0, 10);
        int_seq.push(value);
    }

    let avg:f32 = average(&int_seq);

    println!("average: {avg}");
    println!("{:?}", int_seq);

}
