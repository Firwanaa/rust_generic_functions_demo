trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;

impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Checked in as Poilot");
    }
    fn process(&self) {
        println!("Poilot enters the cockpit");
    }
}

struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Checked in as Passenger");
    }
    fn process(&self) {
        println!("Passenger takes a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in");
    }
    fn process(&self) {
        println!("Cargo moved to storage");
    }
}

fn checkin_t(obj: impl CheckIn) {
    obj.check_in();
}
fn process_t(obj: impl CheckIn) {
    obj.process();
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    println!("Hello, world!");

    let pilot = Pilot;
    let passenger = Passenger;
    let cargo = Cargo;

    // checkin_t(pilot);
    checkin_t(passenger);

    // process_t(cargo);

    process_item(cargo);
    process_item(pilot);
}
