
extern crate clap;
extern crate rustbreak;

use std::process::exit;
use std::env;
use clap::{Arg, App};
use rustbreak::Database;

static STOREPATH: &'static str = "/home/asui/.config/pathstore";

fn main() {
    let matches = App::new("Shell Bookmarks")
        .version("0.1.0")
        .author("Sickeler K. <k.sickeler@gmail.com>")
        .about("Bookmark path by names")
        .arg(Arg::with_name("set")
             .short("s")
             .long("set")
             .value_name("name")
             .help("Sets the current path ")
             .takes_value(true))
        .arg(Arg::with_name("get")
             .short("g")
             .long("get")
             .value_name("name")
             .help("Go to path by name")
             .takes_value(true))
        .arg(Arg::with_name("remove")
             .short("r")
             .long("remove")
             .help("Remove the whole store")
             .takes_value(false))
        .arg(Arg::with_name("delete")
             .short("d")
             .long("delete")
             .help("Delete entry from store")
             .takes_value(true))
        .get_matches();


    match matches.value_of("set") {
        Some(n) => set(n),
        _ => {},
    }

    match matches.value_of("get") {
        Some(n) => get(n),
        _ => {},
    }

    match matches.is_present("remove") {
        true => remove(),
        _ => {},
    }

    match matches.value_of("delete") {
        Some(n) => delete(n),
        _ => {},
    }

    unreachable!()
}

fn set(name: &str) -> ! {
    let db = Database::<String>::open(STOREPATH)
        .unwrap_or_else(|e| {
            print!("{:?}", e);
            exit(1)});

    let path = env::current_dir()
        .map(|p| db.insert(name, p))
        .map_err(|e| {
            println!("{:?}", e);
            ()
        })
        .and_then(|_| db.flush()
                  .map_err(|e| {
                      println!("{:?}", e);
                      ()
                  }));


    match path {
        Ok(_) => exit(0),
        _ => exit(1),
    }
}

fn get(name: &str) -> ! {
    let path = Database::<String>::open(String::from(STOREPATH))
        .and_then(|d| d.retrieve::<String, str>(name));

    match path {
        Ok(n) => print!("{}", n),
        _ => exit(1),
    }

    exit(0)
}

fn remove() -> ! {
    use std::fs::remove_file;
    let res = remove_file(STOREPATH);

    match res {
        Ok(_) => exit(0),
        _ => exit(1),
    }
}

fn delete(name: &str) -> ! {
    let db = Database::<String>::open(String::from(STOREPATH))
        .unwrap_or_else(|e| {
            print!("{:?}", e);
            exit(1)});

    let res = db.delete(name)
        .map_err(|e| {
            println!("{:?}", e);
            ()
        })
    .and_then(|_| db.flush()
              .map_err(|e| {
                  println!("{:?}", e);
                  ()
              }));

    match res {
        Ok(_) => exit(0),
        _ => exit(1),
    }
}
