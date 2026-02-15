// fn main(){

//     let cake = bake_cake();

//     println!("{cake}")

// }

// fn bake_cake()-> String {
//     String::from ("chocolate cake")
// }


// fn main() {
//     let is_correct = true;
//     let is_event = is_correct;

//     let is_booked = is_correct;

//     println!("{is_correct},{is_event}, {is_booked}");

//     // no

//     let sushi = "Salmon";
//     let dinner = sushi;

//     let _food = sushi;

//     println!("{sushi},{dinner}");
//     // no

//     let sushi = String::from("shushi");
//     let _dinner = sushi;

//     // println!("{sushi1}{dinner1}");
//     // yes

//     let food = add_to_menu(String::from("salmon"));

//     println!("{food}");
//     menu(&food);
// }

// fn main() {
//     let mut food = String::from ("Salmon");
//     add_to_menu(&mut food);

//     println!("{food}");
//     menu(&food);
// }

// fn add_to_menu(meal: &mut String)  {
//     meal.push_str(" Rice");
// }

// fn menu(meal: &String) {
//     println!("this is menu {meal}");
// }


// fn main(){
//     let mut car = String::from ("BMW");
//     let ref1 = &mut car;
//     let ref2 = &mut car;

//     println!("{ref2}");
// }

// fn main(){
//     let _tea = make_tea();

// }

// fn make_tea() -> String{
//     String::from("british")
// }

// fn main(){
//     let level = (String::from("100"),String::from("200"));
//     let first_level = & level.0;

//     let class= vec![String::from("1"),String::from("1"),String::from("1")];
//     let first = &class[0];

//     println!("this is first {first}, this is class{class:?}");

//     println!("{first_level}, {level:?}");

// }

// fn main(){

//     let mut trip = start_trip();
//     visit_philli(&mut trip);
//     trip.push_str(" and ");
//     visit_ny(&mut trip);
//     trip.push_str(" and ");
//     visit_bst(&mut trip);
//     trip.push('.');

//     show_art(&trip);

// }

// fn start_trip() ->String{
//     String::from("this trip is")
// }

// fn visit_philli(trip:&mut String){
//     trip.push_str(" Phili");
// }

// fn visit_ny(trip:&mut String){
//     trip.push_str("New y");
// }

// fn visit_bst(trip:&mut String){
//     trip.push_str("boston");
// }

// fn show_art(trip: &String){
//     println!("{trip}")
// }

// fn main(){
//     let name = "John Doe";
//     let ref_name = &name;

//     let first_name = &name[0..=3];
//     let last_name = &name[5..];

//     println!("{first_name} {last_name}");
// }

fn main(){
      let name = "John Doe";
    let ref_name = &name;

    let first_name = &name[..4];
    let last_name = &name[5..];

    let full_name = &name[..];

    println!("{first_name} {last_name} \nthis is full name {full_name}");
}