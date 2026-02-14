// fn main(){

//     let cake = bake_cake();

//     println!("{cake}")


// }

// fn bake_cake()-> String {
//     String::from ("chocolate cake")
// }


fn main(){
    let is_correct = true;
    let is_event = is_correct;

    let is_booked = is_correct;

    println!("{is_correct},{is_event}, {is_booked}");


    // no


    let sushi = "Salmon";
    let dinner = sushi;

    let _food = sushi;

    println!("{sushi},{dinner}");
    // no

    let sushi = String::from ("shushi");
    let _dinner = sushi;

    // println!("{sushi1}{dinner1}");
    // yes

    let food = eat_meal(String::from("salmon"));

    println!("{food}")

}

fn eat_meal(meal:String) -> String{
    meal
}