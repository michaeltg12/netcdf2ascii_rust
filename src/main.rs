// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    path: std::path::PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    match run(&opt.path) {
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
        Ok(()) => {
            std::process::exit(0);
        }
    }
}

fn run(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = netcdf::open(path)?;

    println!("{}", file.name());
    print_group(&file)
}

fn print_group(g: &netcdf::group::Group) -> Result<(), Box<dyn std::error::Error>> {
    println!("Group: {}", g.name());
    println!("Dimensions:");
    for d in g.dimensions() {
        if d.is_unlimited() {
            println!("\t{} : Unlimited ({})", d.name(), d.len());
        } else {
            println!("\t{} : ({})", d.name(), d.len());
        }
    }
    println!("Variables:");
    for v in g.variables() {
        print!("\t{}", v.name());
        print!("(");
        for d in v.dimensions() {
            print!(" {} ", d.name());
        }
        println!(")");
        for a in v.attributes() {
            println!("\t\t{} = {:?}", a.name(), a.value()?);
        }
    }
    println!("Attributes:");
    for a in g.attributes() {
        println!("\t\t{} = {:?}", a.name(), a.value()?);
    }
    for g in g.groups() {
        println!();
        print_group(g)?;
    }

    Ok(())
}

// extern crate netcdf;

// use clap::{Arg, App, SubCommand};
// use netcdf;

// fn main() {
//     let matches = App::new("NetCDF 2 ASCII Converter")
//                           .version("0.1")
//                           .author("Michael Giansiracusa <giansiracumt@ornl.gov>")
//                           .about("Converts netCDF files to flat ASCII csv files, not 2d vars.")
//                           .arg(Arg::with_name("config")
//                                .short("c")
//                                .long("config")
//                                .value_name("FILE")
//                                .help("Sets a custom config file")
//                                .takes_value(true))
//                           .arg(Arg::with_name("INPUT")
//                                .help("Sets the input file to use")
//                                .required(true)
//                                .index(1))
//                           .arg(Arg::with_name("verbose")
//                                .short("v")
//                                .long("verbose")
//                                .multiple(true)
//                                .help("Sets the level of verbosity"))
//                           .subcommand(SubCommand::with_name("test")
//                                       .about("controls testing features")
//                                       .version("0.1")
//                                       .author("Someone E. <someone_else@other.com>")
//                                       .arg(Arg::with_name("debug")
//                                           .short("d")
//                                           .help("print debug information verbosely")))
//                           .get_matches();

//     // Gets a value for config if supplied by user, or defaults to "default.conf"
//     let config = matches.value_of("config").unwrap_or("default.conf");
//     println!("Value for config: {}", config);

//     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//     // required we could have used an 'if let' to conditionally get the value)
//     let file_name = matches.value_of("INPUT").unwrap();
//     println!("Using input file: {}", &file_name);

//     // Vary the output based on how many times the user used the "verbose" flag
//     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//     match matches.occurrences_of("verbose") {
//         0 => println!("No verbose info"),
//         1 => println!("Some verbose info"),
//         2 => println!("Tons of verbose info"),
//         3 | _ => println!("Don't be crazy"),
//     }

//     // You can handle information about subcommands by requesting their matches by name
//     // (as below), requesting just the name used, or both at the same time
//     if let Some(matches) = matches.subcommand_matches("test") {
//         if matches.is_present("debug") {
//             println!("Printing debug info...");
//         } else {
//             println!("Printing normally...");
//         }
//     }

//     // open netcdf file supplied as INPUT
//     let file = netcdf::open(&file_name).unwrap();


//     // more program logic goes here...
// }