
enum IpAddrType
{
    V4(String),
    V6(String),
}

fn route(ipType: IpAddrType)
{
    println!("route()");
}

fn main() {
    println!("Begin");

    let four = IpAddrType::V4(String::from("192.168.0.0"));
    let six = IpAddrType::V6(String::from("10.10.1.0"));

    route(four);
    route(six);

    let someInt = Some(105);

    let absentInt: Option<i32> = None;

}
