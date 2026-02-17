// #[derive(Debug)]
// #[allow(unused_variables)]
// struct Staff {
//     name: String,
//     age: u32,
//     is_admin: bool,
// }
// fn main() {

//     // let mut john = Staff{
//     //     age: 32,
//     //     name: String::from("John Doe"),
//     //     is_admin:false
//     // };

//     // let johns_age = john.age;

//     // john.name = String::from("John David");
//     // john.age =33;
//     // john.is_admin = true;

//     // println!("{} is {} and his admin status is {}", john.name, john.age, john.is_admin);

//     // let staff_name =String::from("jane doe");

//     // let new_staff = add_staff(staff_name, 32,true);

//     // let new_staff2 = Staff{
//     //     name: String::from("jason bourne"),
//     //     age: 28,
//     //     is_admin: new_staff.is_admin,
//     // };

//     // let _new_staff3 = Staff{
//     //     name: String::from("john wick"),
//     //     ..new_staff2
//     // };

//     // let new_staff4 = Staff{name:new_staff.name.clone(), ..new_staff};
//     // println!("{}", new_staff.name);

//     // println!("the new staff {} is {} and admin stats is {}",new_staff.name, new_staff.age, new_staff.is_admin);

//     let mut new_staff = Staff{
//         name: String::from("jason bourne"),
//         age: 28,
//         is_admin: true,
//     };

//     print_name(&mut new_staff);
//     println!("{}", new_staff.name);

// }

// // fn add_staff(name: String, age: u32, is_admin: bool) -> Staff {
// //     Staff {
// //         name,
// //         age,
// //         is_admin,
// //     }
// // }

// fn print_name(staff: &mut Staff) {
//     println!("{}", staff.name);
//     staff.is_admin = true;
//     println!("{}", staff.is_admin);
//     println!("{staff:?}");
//     println!("{:#?}",staff );
// }

#[derive(Debug)]
#[allow(unused_variables)]
#[allow(unused)]
struct  Songs{
    artist: String,
    title: String,
    release: u32,
    duration: u32,
}

impl Songs {
    fn display_info(&self){
        println!("title {:#?}", self);
    }

    fn increase_len(&mut self, time:u32 ){
        self.duration = self.duration * time;
        println!("{:#?}", self)
    }

    fn compare_len(&self, other_song: &Self)-> bool{
        self.duration > other_song.duration
    }
}
fn main(){
    let mut kendrick = Songs{
        title: String::from("Rich Spirit"),
        artist: String::from("Kdot"),
        release: 2023,
        duration: 3,
    };

        let drake = Songs{
        title: String::from("Gods Plan"),
        artist: String::from("Drake"),
        release: 2023,
        duration: 5,
    };

    kendrick.increase_len(2);
    if kendrick.compare_len(&drake){
        println!("Kendricks {} is longer than drakes {}", kendrick.title, drake.title);
    }else {
        println!("drakes {} is longer than kendrick {}", drake.title, kendrick.title);
        
    }
    drake.compare_len(&kendrick);

}