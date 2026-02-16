fn main(){
    let mut cereals:[String;5] = [String::from("cookie Crips"),String::from("Cinnamon Toast Crunch"),String::from("Frosted Flakes"),String::from("Cocoa Puffs"),String::from("Captain Crunch")];

    let first_two = &cereals[0..=1];
    println!("{first_two:?}");

    let mid_three = &cereals[1..=3];
    println!("{mid_three:?}");

    let last_three = &mut cereals[2..];
    println!("{last_three:?}");

    last_three[2] = String::from ("Lucky Charms");

    println!("this is cereals {cereals:?}");

    let cookie_crips = &cereals[0];
    let cookie = &cookie_crips[0..7];

    println!("this is {cookie}");

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];

    println!("this is {puffs}")

}

