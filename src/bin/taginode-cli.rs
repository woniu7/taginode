use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::prelude::MetadataExt;
use std::time::UNIX_EPOCH;
use taginode::INode;

fn usage() {
    eprintln!("Usage: taginode-cli tag <file> [tag1 tag2...]");
    eprintln!("Usage: taginode-cli search [tag1 tag2...]");
    eprintln!("Usage: taginode-cli list tags");
    std::process::exit(1);
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }
    match args[1].as_str() {
        "tag" => tag(),
        "search" => search(),
        "list" => list(),
        _ => usage(),
    }
    Ok(())
}

fn tag() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    if args.len() < 2 {
        usage();
    }
    let files = &args[0..1];
    let tag_names = &args[1..];
    eprintln!("tag_names: {:?}, files: {:?}", tag_names, files);

    let mut db_file = env::var("HOME").unwrap();
    db_file.push_str("/.taginode.db");
    let connection = taginode::sql::init(&db_file);
    for file in files {
        let metadata = fs::metadata(file.to_string());
        let metadata = match metadata {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("{:?}", error);
                continue;
            },
        };
        let btime = metadata.created();
        let btime = match btime {
            Ok(btime) => { 
                match btime.duration_since(UNIX_EPOCH) {
                    Ok(btime) => Some(btime.as_secs()),
                    Err(error) => {
                        eprintln!("Warning: {:?}", error);
                        None
                    }
                }
            },
            Err(error) => {
                eprintln!("Warning: {:?}", error);
                None
            }
        };
        taginode::add(&connection, 
            &vec![ INode{ device: metadata.dev(), number: metadata.ino(), btime } ],
            &tag_names,
        );
    }
}

fn search() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    if args.len() < 1 {
        usage();
    }
    let tag_names = &args[0..];
    let paths = vec!["."];
    eprintln!("tag_names: {:?}, paths: {:?}", tag_names, paths);

    let mut db_file = env::var("HOME").unwrap();
    db_file.push_str("/.taginode.db");
    let connection = taginode::sql::init(&db_file);
    let inodes = taginode::get_inodes(&connection, tag_names);
    let mut dev_inode_map: HashMap<u64, HashMap<u64, &INode>> = HashMap::new();
    for inode in &inodes {
        let inode_map = dev_inode_map.get_mut(&inode.device);
        match inode_map {
            Some(inode_map) => {
                inode_map.insert(inode.number, inode);
            }, 
            None => {
                let mut inode_map = HashMap::new();
                inode_map.insert(inode.number, inode);
                dev_inode_map.insert(inode.device, inode_map);
            }
        };
    }
    for path in paths {
        match process_file(&dev_inode_map, path) {
            Err(error) => eprintln!("{error:?}"),
            _ => (),
        }
    }
}

fn process_file(dev_inode_map: &HashMap<u64, HashMap<u64, &INode>>, f: &str) -> Result<(), Error> {
    let metadata = fs::metadata(f)?;
    if dev_inode_map.get(&metadata.dev()).is_some() {
        let inode_map = dev_inode_map.get(&metadata.dev()).unwrap();
        let mut hit = false;
        match inode_map.get(&metadata.ino()) {
            None => (),
            Some(ino) => {
                let created = (|| -> Result<u64, Error> {
                    match metadata.created()?.duration_since(UNIX_EPOCH) {
                        Ok(btime) => 
                        Ok(btime.as_secs()),
                        Err(error) => 
                        Err(Error::new(ErrorKind::Other, error.to_string())),
                    }
                })();
                if ino.btime == None || created.is_err() || ino.btime.unwrap() == created.unwrap() {
                    hit = true;
                }
            }
        };
        if hit {
            println!("{} ", f);
        }
        if metadata.is_dir() {
            let paths = fs::read_dir(f)?;
            for path in paths {
                match path {
                    Ok(entry) => {
                        if entry.metadata()?.is_symlink() {
                            continue;
                        }
                        let p = entry.path();
                        let p= p.to_str().unwrap();
                        match process_file(&dev_inode_map, p) {
                            Err(error) => eprintln!("{p:?} {error:?}"),
                            _ => (),
                        }
                    }
                    Err(error) => eprintln!("{f} {error:?}"),
                };
            }
        }
    }
    Ok(())
}

fn list() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    if args.len() < 1 && args[0] != "tags" {
        usage();
    }

    let mut db_file = env::var("HOME").unwrap();
    db_file.push_str("/.taginode.db");
    let connection = taginode::sql::init(&db_file);
    let tag_names = taginode::list_tags(&connection);
    println!("{tag_names:?}")
}