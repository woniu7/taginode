use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::prelude::MetadataExt;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sqlite3::Connection;
use taginode::INode;
use taginode::opt::OptArg;
use taginode::opt::OptCheck;

fn usage(opt_check: &OptCheck) -> impl Fn() {
    let usage_opt = taginode::opt::usage(opt_check);
    move || {
        eprintln!("Usage: taginode-cli [option] tag <file> <tag> \"tag1[,tag2,tag3...]\"");
        eprintln!("Usage: taginode-cli [option] search [-d directory] \"tag1[,tag2,tag3...]\"");
        eprintln!("Usage: taginode-cli [option] list tags");
        eprintln!("Usage: taginode-cli [option] cat <file> [file]...");
        eprintln!("{usage_opt}");
        std::process::exit(1);
    }
}

fn main() -> Result<(), Error>{
    let mut default_db = env::var("HOME").unwrap();
    default_db.push_str("/.taginode.db");

    let opt_check = BTreeMap::from([
        (b'f', (OptArg::Mandatory(default_db.as_str()), "-f <db>        specify db path to store data, default ~/.taginode.db"                           )),
        (b'd', (                OptArg::Mandatory("."), "-d <directory> [search]specify path to search file by tags, default \".\""                      )),
        (b'a', (                          OptArg::None, "-a             [search]ensable cross devices, default only search dev of path specified by -d"  )),
        (b'u', (                          OptArg::None, "-u             [search]output same inode(default remove duplicate item"                         )),
        // (b'v', (                          OptArg::None, "-v             verbose"                                                                    )),
        // (b'V', (                          OptArg::None, "-V             version"                                                                    )),
        // (b'l', (                          OptArg::None, "-l             follow symbolic links instead of symbolic file itself"                      )),
        // (b'5', (                          OptArg::None, "-5             md5 mode instead of inode"                                                  )),
    ]);
    let usage = usage(&opt_check);

    let args: Vec<String> = env::args().collect();
    let (options, operands) = 
        taginode::opt::get_opt_per(&args[1..], &opt_check).unwrap_or_else(|err| {
            eprintln!("{}: {}",args[0], err);
            usage();
            std::process::exit(1);
        });
    if operands.is_empty() {
        usage();
    }

    let db_path = options.get(&b'f').copied().unwrap_or(default_db.as_str());
    let db = taginode::sql::init(&db_path);

    let ret = match operands[0] {
        "tag" => tag(&operands[1..], db),
        "search" => search(&operands[1..], options, db),
        "list" => list(&operands[1..], db),
        "cat" => cat(&operands[1..], db),
        _ => Err(Error::new(ErrorKind::Other, "")),
    };
    if ret.is_err() {
        eprintln!("{}", ret.as_ref().unwrap_err());
        usage();
    }
    ret
}

fn tag(operands: &[&str], db: Connection) -> Result<(), Error> {
    if operands.len() < 2 {
        return err_str("");
    }
    let files = &operands[0..1];
    let tag_names: Vec<&str> = operands[1].split(",").collect();
    eprintln!("tag_names: {:?}, files: {:?}", tag_names, files);

    for file in files {
        let metadata = fs::symlink_metadata(file.to_string());
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
        taginode::add(&db, 
            &vec![ INode{ device: metadata.dev(), number: metadata.ino(), btime } ],
            &tag_names,
        );
    }
    Ok(())
}

fn search(operands: &[&str], options: HashMap<u8, &str>, db: Connection) -> Result<(), Error> {
    if operands.len() != 1 {
        return err_str("");
    }
    let tag_names: Vec<&str> = operands[0].split(",").collect();
    let paths = vec![options.get(&b'd').copied().unwrap_or("")];
    eprintln!("tag_names: {:?}, paths: {:?}", tag_names, paths);

    let inodes = taginode::get_inodes(&db, &tag_names);
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

    let mut occur: Option<HashMap<u64, HashMap<u64, String>>> = match options.get(&b'u') {
        Some(_) => None,
        None => Some(HashMap::new()),
    };
    for path in paths {
        match process_file(&dev_inode_map, path, options.get(&b'a').is_some(), &mut occur) {
            Err(error) => eprintln!("{path}: {error:?}"),
            _ => (),
        }
    }
    Ok(())
}

fn process_file(dev_inode_map: &HashMap<u64, HashMap<u64, &INode>>, f: &str, cross_dev: bool, occur: &mut Option<HashMap<u64, HashMap<u64, String>>>) -> Result<(), Error> {
    let metadata = fs::symlink_metadata(f)?;
    if occur.is_some() {
        let occur = occur.as_mut().unwrap();
        match occur.get_mut(&metadata.dev()) {
            Some(s) => {
                match s.get_mut(&metadata.ino()) {
                    Some(old) => {
                        eprintln!("{}: same file as '{}'",f, old);
                        return Ok(())
                    }, 
                    None => {
                        s.insert(metadata.ino(), f.to_string());
                    },
                }
            },
            None => {
                let mut inode_map = HashMap::new();
                inode_map.insert(metadata.ino(), f.to_string());
                occur.insert(metadata.dev(), inode_map);
            },
        }
    }
    match dev_inode_map.get(&metadata.dev()) {
        Some(inode_map) => {
        match inode_map.get(&metadata.ino()) {
            None => (),
            Some(ino) => {
                let created = get_file_btime(metadata.created());
                if ino.btime == None || created.is_none() || ino.btime.unwrap() == created.unwrap() {
                    println!("{} ", f);
                }
            }
        };
        },
        None if !cross_dev => return Ok(()), 
        _ => (),
    }
    if metadata.is_dir() {
        if metadata.is_symlink() {
            return Ok(())
        }
        let paths = fs::read_dir(f)?;
        for path in paths {
            match path {
                Ok(entry) => {
                    let p = entry.path();
                    let p= p.to_str().unwrap();
                    match process_file(&dev_inode_map, p, cross_dev, occur) {
                        Err(error) => eprintln!("{p}: {error:?}"),
                        _ => (),
                    }
                }
                Err(error) => eprintln!("{f} {error:?}"),
            };
        }
    }
    Ok(())
}

fn list(args: &[&str], db: Connection) -> Result<(), Error> {
    if args.len() < 1 || args[0] != "tags" {
        return err_str("");
    }

    let tag_names = taginode::list_tags(&db);
    for tag_name in tag_names {
        println!("{tag_name:?}")
    }
    Ok(())
}

fn cat(args: &[&str], db: Connection) -> Result<(), Error> {
    if args.len() < 1 {
        return err_str("");
    }
    for path in args {
        let metadata = fs::symlink_metadata(path);
        print!("{}:    ", path);
        match metadata {
            Ok(metadata) => {
                let tag_names = taginode::get_tags(
                    &db, 
                    INode { 
                        device: metadata.dev(), 
                        number: metadata.ino(), 
                        btime: get_file_btime(metadata.created()),
                    },
                );
                println!("{tag_names:?}")
            },
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}

fn get_file_btime(btime: std::io::Result<SystemTime>) -> Option<u64> {
// match btime {
//     Ok(btime) => { 
//         match btime.duration_since(UNIX_EPOCH) {
//             Ok(btime) => Some(btime.as_secs()),
//             Err(error) => {
//                 // eprintln!("Warning: {:?}", error);
//                 None
//             }
//         }
//     },
//     Err(error) => {
//         // eprintln!("Warning: {:?}", error);
//         None
//     }
// }
    if btime.is_ok() && btime.as_ref().unwrap().duration_since(UNIX_EPOCH).is_ok() {
        Some(btime.unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs())
    } else {
        None
    }
}

fn err_str(msg: &str) -> Result<(), Error> {
    return Err(Error::new(ErrorKind::Other, msg));
}