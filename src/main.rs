/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch

 
Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.
 
Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.
 
Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.
 
Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.
 
Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String.
 
Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.
 
Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).
 
Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

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

}

