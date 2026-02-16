    struct Staff {
        name: String,
        age: u32,
        is_admin: bool,
    }
fn main(){


    // let mut john = Staff{
    //     age: 32,
    //     name: String::from("John Doe"),
    //     is_admin:false
    // };

    // let johns_age = john.age;

    // john.name = String::from("John David");
    // john.age =33;
    // john.is_admin = true;



    // println!("{} is {} and his admin status is {}", john.name, john.age, john.is_admin);


    let staff_name =String::from("jane doe");

    let new_staff = add_staff(staff_name, 32,true);

    println!("the new staff {} is {} and admin stats is {}",new_staff.name, new_staff.age, new_staff.is_admin);


}

fn add_staff(name:String, age:u32, is_admin:bool)-> Staff{
    Staff { 
        name, 
        age, 
        is_admin 
    }
}