use rand::Rng;

fn random_int(min: u32, max: u32) -> usize
{
    rand::thread_rng().gen_range(min..max) as usize 
}

fn main()
{
    let count = 100;
    let mut distribution = [0; 100];
    let mut buckets = [0; 20];
    let mut sum = 0;

    for _i in 0..count
    {
        let mut value = random_int(0, 100);
        sum += value;
        distribution[value] += 1;
    }

    for n in distribution
    {
        //print!("{}", n);
    }

    let average = sum / count;

    println!("\nsum: {}\naverage: {}\n", sum, average);

}
