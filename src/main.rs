fn main(){
    struct Staff {
        name: String,
        age: u32,
        is_admin: bool,
    }

    let mut john = Staff{
        age: 32,
        name: String::from("John Doe"),
        is_admin:false
    };

    let johns_age = john.age;

    john.name = String::from("John David");
    john.age =33;
    john.is_admin = true;



    println!("{} is {} and his admin status is {}", john.name, john.age, john.is_admin);


}