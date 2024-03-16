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
    for i in 1..=length
    {
        int_seq.push(i);
    }

    let avg:f32 = average(&int_seq);

    println!("average: {avg}");
    println!("{:?}", int_seq);

}
