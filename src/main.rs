#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
#[macro_use]
extern crate lazy_static;
use clap::Parser;
use markov::Chain;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref CHAIN: Chain<String> = build_chain();
    static ref ARGS: Args = Args::parse();
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, String> =
        [("wisdom", CHAIN.generate_str())].iter().cloned().collect();
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::Config::default();
    config.port = ARGS.port;
    rocket::custom(config)
        .attach(Template::fairing())
        .mount("/", routes![index])
}

fn build_chain() -> Chain<String> {
    println!("Building Chain from \"{}\"...", ARGS.input);
    let mut new_chain = Chain::new();
    // get input from input file
    let file = File::open(&ARGS.input).expect("No input file!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        new_chain.feed_str(&line.unwrap());
    }
    new_chain
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // Input file to make markov chain from
    #[clap(short, long)]
    input: String,

    // Port number for server
    #[clap(short, long, default_value_t = 8000)]
    port: u16,
}
