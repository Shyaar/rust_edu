#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }
}

impl Flight {
    fn change_dest(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.2;
        println!("new price = {}", self.price);
        self
    }

    fn itinerary(&self) {
        println!("origin {} -> destination {}", self.origin, self.destination);
    }
}
fn main() {
    let mut new_flight = Flight::new(
        String::from("jos"), 
        String::from("Abuja"), 
        25000.500,
        120
    );

    new_flight.itinerary();
    new_flight
        .change_dest(String::from("lagos"))
        .increase_price()
        .itinerary();

    let new_flight2 = Flight {
        origin: String::from("Kano"),
        destination: String::from("Adamawa"),
        ..new_flight
    };

    new_flight2.itinerary();

    
}
