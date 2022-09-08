use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Error;
use std::os::unix::prelude::MetadataExt;
use taginode::INode;

fn usage() {
    eprintln!("Usage: taginode-cli tag <file> [tag1 tag2...]");
    eprintln!("Usage: taginode-cli search [tag1 tag2...]");
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
        _ => usage(),
    }
    Ok(())
}

fn tag() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    let mut db_file = env::var("HOME").unwrap();
    db_file.push_str("/.taginode.db");
    let connection = taginode::sql::init(&db_file);

    if args.len() < 2 {
        usage();
    }
    let files = &args[0..1];
    let tag_names = &args[1..];
    println!("tag_names: {:?}, files: {:?}", tag_names, files);

    for file in files {
        let metadata = fs::metadata(file.to_string());
        let metadata = match metadata {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("{:?}", error);
                continue;
            },
        };
        taginode::add(&connection, 
            &vec![ INode{ device: metadata.dev(), number: metadata.ino() } ],
            &tag_names,
        );
    }
}

fn search() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    let mut db_file = env::var("HOME").unwrap();
    db_file.push_str("/.taginode.db");
    let connection = taginode::sql::init(&db_file);

    if args.len() < 1 {
        usage();
    }
    let tag_names = &args[0..];
    // let cur = env::current_dir().unwrap();
    let paths = vec!["."];
    println!("tag_names: {:?}, paths: {:?}", tag_names, paths);

    let inodes = taginode::get_inodes(&connection, tag_names);
    let mut inode_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for inode in inodes {
        let inode_set = inode_map.get_mut(&inode.device);
        match inode_set {
            Some(inode_set) => {
                inode_set.insert(inode.number);
            }, 
            None => {
                let mut inode_set = HashSet::new();
                inode_set.insert(inode.number);
                inode_map.insert(inode.device, inode_set);
            }
        };
    }
    for path in paths {
        match process_file(&inode_map, path) {
            Err(error) => eprintln!("{error:?}"),
            _ => (),
        }
    }
}

fn process_file(inode_map: &HashMap<u64, HashSet<u64>>, f: &str) -> Result<(), Error> {
    let metadata = fs::metadata(f)?;
    match inode_map.get(&metadata.dev()) {
        Some(inode_set) => {
            if None != inode_set.get(&metadata.ino()) {
                println!("{}", f);
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
                            match process_file(&inode_map, p) {
                                Err(error) => eprintln!("{p:?} {error:?}"),
                                _ => (),
                            }
                        }
                        Err(error) => eprintln!("{f} {error:?}"),
                    };
                }
            }
            Ok(())
        },
        _ => Ok(()),
    }
}