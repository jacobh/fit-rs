extern crate fit;

fn main() {
    let fitfile = fit::FitFile::open("examples/data/activity_commute.fit").unwrap();
    let header = fitfile.get_header();
    println!("{:?}", header);
    println!("opened fitfile successfully");
}
