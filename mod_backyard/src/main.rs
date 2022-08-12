use crate::garden::vegetables::Asparagus;

pub mod garden;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
