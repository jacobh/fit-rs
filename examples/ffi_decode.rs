extern crate fit;

fn main() {
    fit::decode::decode("examples/data/activity_commute.fit");
    println!("opened fitfile successfully");
}
