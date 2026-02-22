// enum Os {
//     Windows,
//     Mac,
//     Linux,
// }

// fn main(){
//     let my_pc = Os::Linux;

//     let age = year_release(my_pc);

//     println!("{age}")
// }

// fn year_release(os: Os) -> u32{

//     match os {
//         Os::Windows => 39,
//         Os::Mac => 23,
//         Os::Linux => 34,
//     }
// }
#[derive(Debug)]
enum Breverage{
    Tea,
    Cocoa{ temprature: u32},
    Coffee(String)
}

fn main(){
    let cocoa_temp = 30;
    let coffee = String::from ("english");

    order_breverage(Breverage::Cocoa { temprature: cocoa_temp });

    order_breverage(Breverage::Coffee(coffee));

    order_breverage(Breverage::Tea);
}

fn order_breverage(breverage: Breverage){
    match breverage{
        Breverage::Coffee(_type) => {
            println!("You ordered {:?}", Breverage::Coffee(_type));
        },
        Breverage::Cocoa { temprature } =>{
            println!("You ordered {:#?}", Breverage::Cocoa{temprature});
        },
        Breverage::Tea =>{
            println!("You ordered {:#?}", Breverage::Tea);
        }
    }
}