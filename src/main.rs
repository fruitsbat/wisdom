#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative, NamedFile};
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
    static ref FOOD_CHAIN: Chain<String> = build_chain("cooking.txt");
    static ref FRAKES_CHAIN: Chain<String> = build_chain("frakes.txt");
    static ref ARGS: Args = Args::parse();
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/cooking")]
fn cooking() -> Template {
    get_wisdom_template_from(FOOD_CHAIN.generate_str())
}

#[get("/frakes")]
fn frakes() -> Template {
    get_wisdom_template_from(FRAKES_CHAIN.generate_str())
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::Config::default();
    config.port = ARGS.port;
    rocket::custom(config)
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index, cooking, frakes])
}

fn get_wisdom_template_from(string: String) -> Template {
    let context: HashMap<&str, String> = [("wisdom", string)].iter().cloned().collect();
    Template::render("wisdom", context)
}

fn build_chain(input: &str) -> Chain<String> {
    println!("Building Chain from \"{}\"...", input);
    let mut new_chain = Chain::new();
    // get input from input file
    let file = File::open(input).expect("No input file!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        new_chain.feed_str(&line.unwrap());
    }
    new_chain
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // Port number for server
    #[clap(short, long, default_value_t = 8000)]
    port: u16,
}
