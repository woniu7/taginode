use std::env;
use std::fs;
use taginode::INode;
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::windows::fs::MetadataExt;

fn usage() {
    eprintln!("Usage: taginode-cli tag -t \"tag1,tag2...\" <file>");
    eprintln!("Usage: taginode-cli search -t \"tag1,tag2...\" [path...]");
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
    let connection = taginode::sql::init("taginode.db");
    if args[1] == "-t" {
        let metadata = fs::metadata(".")?;
        let tag_names: Vec<&str> = args[2..].iter().map(|val| {
            val.as_str()
        }).collect();
        let inode_numbers = taginode::get_inodenums(&connection, metadata.st_dev(), &tag_names);
        println!("{:?}", inode_numbers);
    } else {
        let metadata = fs::metadata(args[1].to_string())?;
        let tag_names: Vec<&str> = args[2..].iter().map(|val| {
            val.as_str()
        }).collect();
        taginode::add(&connection, 
            &vec![ INode{ device: metadata.st_dev(), number: metadata.st_ino() } ],
            &tag_names,
        );
    }
    Ok(())
}

fn tag() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|val| {
        val.as_str()
    }).collect();
    let connection = taginode::sql::init("taginode.db");
    let mut index_t = 0;
    for (i , e) in args.iter().enumerate() {
        match *e {
            "-t" => index_t = i,
            _ => (),
        }
    }
    if index_t == 2 { 
        usage(); 
    };
    let tag_names;
    let files;
    if index_f > index_t {
        tag_names = &args[1+index_t..index_f];
        files = &args[1+index_f..];
    } else {
        tag_names = &args[1+index_t..];
        files = &args[1+index_f..index_t];
    }
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
            &vec![ INode{ device: metadata.st_dev(), number: metadata.st_ino() } ],
            &tag_names,
        );
    }
}

fn search() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|val| {
        val.as_str()
    }).collect();
    let connection = taginode::sql::init("taginode.db");

    let mut index_t = 0;
    for (i , e) in args.iter().enumerate() {
        match *e {
            "-t" => index_t = i,
            _ => (),
        }
    }
    if !(index_f == 2 || index_t == 2) { usage(); };
    let tag_names;
    let files;
    if index_f > index_t {
        tag_names = &args[1+index_t..index_f];
        files = &args[1+index_f..];
    } else {
        tag_names = &args[1+index_t..];
        files = &args[1+index_f..index_t];
    }
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
            &vec![ INode{ device: metadata.st_dev(), number: metadata.st_ino() } ],
            &tag_names,
        );
    }
}
