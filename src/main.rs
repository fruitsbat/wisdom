#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use markov::Chain;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref CHAIN: Chain<String> = build_chain();
}

#[get("/")]
fn index() -> String {
    CHAIN.generate_str()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

fn build_chain() -> Chain<String> {
    println!("Building Chain...");
    let mut new_chain = Chain::new();
    // get input from input file
    let file = File::open("./input").expect("No input file!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        new_chain.feed_str(&line.unwrap());
    }
    new_chain
}
