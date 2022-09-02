use std::env;
use std::fs;
use std::os::macos::fs::MetadataExt;
use taginode::INode;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: taginode-cli <file> [tag1 tag2...]");
        eprintln!("usage: taginode-cli -t [tag1 tag2...]");
        std::process::exit(1);
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
