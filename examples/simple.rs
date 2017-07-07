extern crate fit;

fn main() {
    let fitfile = fit::FitFile::open("examples/data/activity_commute.fit").unwrap();
    println!("opened fitfile successfully");
}
