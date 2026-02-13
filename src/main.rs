
fn add_age(mut age: i32 ){
    age+=1;
    println!("{age}")
}


fn prnt_name(name:String){
    println!("{name}");
}
fn main() {
    let age = 32;
    add_age(age);

    let name = String::from ("Joseph");
    prnt_name(name);



}
