#[derive(Debug)]
#[allow(unused)]

struct Pc{
    name: String,
    year: u32,
    core: String,
    memory: u32,
    drive: u32,
}

impl Pc {
    fn new(name:String, year:u32, memory:u32, drive:u32, core:String)->Self{
        Self { name, year, memory, drive, core }
    }
}

impl Pc {
    fn upgrade_memory(&mut self, new_mem: u32)->&mut Self{
        self.memory = new_mem;
        self
    }

    fn upgrade_drive(&mut self, new_drive: u32) -> &mut Self{
        self.drive = new_drive;
        self
    }

    fn upgrade_core(&mut self, new_core:String)-> &mut Self{
        self.core=new_core;
        self
    }
}
fn main(){
    let mut mac= Pc::new(String::from("mac"), 2018, 4, 256, String::from("i5"));



    println!("specs: {mac:#?}");

    mac.upgrade_core(String::from("i7"))
    .upgrade_drive(500)
    .upgrade_memory(16);

    println!("new specs: {mac:#?}");
}