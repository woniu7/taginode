use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::os::unix::prelude::MetadataExt;
use walkdir::WalkDir;
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
    // let connection = taginode::sql::init("taginode.db");
    // if args[1] == "-t" {
    //     let metadata = fs::metadata(".")?;
    //     let tag_names: Vec<&str> = args[2..].iter().map(|val| {
    //         val.as_str()
    //     }).collect();
    //     let inode_numbers = taginode::get_inodenums(&connection, metadata.st_dev(), &tag_names);
    //     println!("{:?}", inode_numbers);
    // } else {
    //     let metadata = fs::metadata(args[1].to_string())?;
    //     let tag_names: Vec<&str> = args[2..].iter().map(|val| {
    //         val.as_str()
    //     }).collect();
    //     taginode::add(&connection, 
    //         &vec![ INode{ device: metadata.st_dev(), number: metadata.st_ino() } ],
    //         &tag_names,
    //     );
    // }
    Ok(())
}

fn tag() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args[2..].iter().map(|val| {
        val.as_str()
    }).collect();
    let connection = taginode::sql::init("taginode.db");

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
    let connection = taginode::sql::init("taginode.db");

    if args.len() < 1 {
        usage();
    }
    let tag_names = &args[0..];
    let paths = vec!["."];
    println!("tag_names: {:?}, paths: {:?}", tag_names, paths);

    let inodes = taginode::get_inodes(&connection, tag_names);
    let inode_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for inode in inodes {
        inode_map.entry(key)
        let n = HashSet::new();
        let inode_set = &mut *inode_map.get(&inode.device).unwrap_or(&n);
        inode_set.insert(inode.number);
    }
    for path in paths {
        let metadata = fs::metadata(path);
        let metadata = match metadata {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("{:?}", error);
                continue;
            },
        };
        let inode_set = inode_map.get(&metadata.dev());


        for entry in WalkDir::new(path) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    eprintln!("{:?}", error);
                    continue;
                },
            };
            let metadata = entry.metadata();
            let metadata = match metadata {
                Ok(metadata) => metadata,
                Err(error) => {
                    eprintln!("{:?}", error);
                    continue;
                },
            };
            
            
            // println!("{}", entry.path().display());
        }
    }
}
