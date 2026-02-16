fn main(){
    struct Staff {
        name: String,
        age: u32,
        is_admin: bool,
    }

    let john = Staff{
        age: 32,
        name: String::from("John Doe"),
        is_admin:false
    };

    let johns_age = john.age;
    let johns_name = &john.name;

    println!("{} is {johns_age} and his admin status is {}", john.name, john.is_admin);


}