mod consts;
mod counter;

use clap::{App, Arg};
use counter::{Counter, Parse};
use io::SeekFrom;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = App::new("counter")
        .version(consts::VERSION)
        .about("Counts stuff, I guess.")
        .arg(
            Arg::with_name("file")
                .long("--file")
                .short("-f")
                .help("The counter file location")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .long("--key")
                .short("-k")
                .help("The value key to be modified")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("AMMOUNT")
                .index(1)
                .help("The ammount to be added, substracted or set")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sub")
                .long("--sub")
                .short("-s")
                .help("Substract value"),
        )
        .arg(Arg::with_name("set").long("--set").help("Set value"))
        .get_matches();

    let action: Result<Option<i32>, Box<dyn Error>> = match args.value_of("AMMOUNT") {
        None => Ok(None),
        Some(val) => {
            let i: i32 = match val.parse() {
                Ok(_i) => _i,
                Err(err) => return Err(err.into()),
            };
            Ok(Some(i))
        }
    };
    if action.is_err() {
        return Err(action.err().unwrap());
    }
    let action = action.unwrap();

    let mut file = get_file(args.value_of("file").unwrap(), action.is_some())?;

    let mut buff: Vec<u8> = vec![];
    file.read_to_end(&mut buff)?;

    let mut counter = Counter::decode(&buff)?;

    if action.is_some() {
        let key = match args.value_of("key") {
            Some(v) => String::from(v),
            None => return Err("no key provided".into()),
        };

        let value = action.unwrap();
        if args.is_present("set") {
            counter.set(key, value)
        } else if args.is_present("sub") {
            counter.modify(key, -value);
        } else {
            counter.modify(key, value);
        }

        file.seek(SeekFrom::Start(0))?;
        file.write(&counter.encode())?;
    }

    Ok(())
}

fn get_file(file_path: &str, write: bool) -> Result<File, Box<dyn Error>> {
    let file_path = Path::new(file_path);
    if file_path.is_dir() {
        return Err("given path points to a directory".into());
    }
    Ok(OpenOptions::new()
        .create(true)
        .read(true)
        .write(write)
        .append(false)
        .open(file_path)?)
}
