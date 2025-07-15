use std::net::SocketAddr;

use std::{ffi::OsString, fs::{create_dir, read_dir}};

pub fn get_instruments() -> Vec<String> {
    let mut instruments = vec![];
    let index_path = "./sites";

    match read_dir(index_path) {
        Ok(dirs) => {
            for dir in dirs {
                match dir {
                    Ok(entry) => {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if let Ok(files) = read_dir(entry.path()) {
                                    for file in files {
                                        if let Ok(file_entry) = file {
                                            if file_entry.file_name() == OsString::from("index.html") {
                                                match entry.file_name().into_string() {
                                                    Ok(str) => instruments.push(str),
                                                    _ => {},
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },

                    Err(e) => println!("Get DirEntry Error: {e}"),
                }
            }
        },

        Err(e) => {
            println!("Cant read {index_path}: {e}\nCreate it!");
            let _ = create_dir(index_path);
            instruments = get_instruments();
        },
    }

    instruments
}

pub fn print_instruments(addr: &SocketAddr) {
    let mut index = 0;
    println!("Доступные инструменты:");
    
    for instrument in &get_instruments() {
        index += 1;
        println!("{index}. {instrument} - http://{addr}/{instrument}");
    }
}

pub fn parse_referer(referer: String) -> String {
    let mut slash_count = 0;
    let mut result = String::new();

    for i in 0..referer.len() {
        let symb: String = referer[i..=i].into();

        if symb == String::from("/") {
            slash_count += 1;
        }

        if slash_count >= 4 {
            break;
        }

        if slash_count >= 3 {
            result.push_str(&symb);
        }
    }

    result
}