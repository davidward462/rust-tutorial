
struct point(i32, i32, i32);

struct color(u32, u32, u32);

struct User {
    active: bool,
    username: String,
    email: String,
    id_number: u64,
}

struct Device {
    category: String,
    id_number: u64,
    name: String,
}

fn build_device(category: String, id_number: u64, name: String) -> Device
{
    Device {
        category,
        id_number,
        name,
    }
}


fn main() {

    let my_user =  User { 
        active: true, 
        username: String::from("userOne"), 
        email: String::from("one@email.com"),
        id_number: 1,
    };

    let c1 = String::from("phone");
    let n1 = String::from("nokia");

    let d1 = build_device(c1, 6508345, n1);

    let val = d1.name;


}
