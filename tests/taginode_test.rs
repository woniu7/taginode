use taginode::INode;

#[test]
fn t() {
    let connection = taginode::sql::init("fk.db");
    {
        let inodes = vec![
            INode{ device: 16777220, number: 12951634006, btime: None },
            INode{ device: 16777220, number: 12951634036, btime: None },
        ];
        taginode::add(&connection, &inodes, &["ikun", "basketball", "man"]);
    }
    {
        let inodes = vec![
            INode{ device: 16777221, number: 12951634006, btime: None },
            INode{ device: 16777220, number: 12951634036, btime: Some(1665935055) },
        ];
        taginode::add(&connection, &inodes, &["basketball", "chicken"]);
    }

    {
        let tag_names = vec![ "ikun", "basketball", "ikun", "chicken"];
        let inodes = taginode::get_inodes_by_tags(&connection, &tag_names);
            println!("{:?}", inodes);
        let expect = vec![INode { device:16777220, number: 12951634036, btime: Some(1665935055) }];
        assert_eq!(inodes.len(), expect.len());
        for (i, e) in inodes.iter().enumerate() {
            assert_eq!(e.device, expect[i].device);
            assert_eq!(e.number, expect[i].number);
            assert_eq!(e.btime,  expect[i].btime);
        }
    }
    {
        let inodes = taginode::get_inodes_by_tags(&connection, &vec![ "basketball"]);
            println!("{:?}", inodes);
        let expect = vec![
            INode { device:16777220, number: 12951634006, btime: None },
            INode { device:16777220, number: 12951634036, btime: Some(1665935055) },
            INode { device:16777221, number: 12951634006, btime: None },
        ];
        assert_eq!(inodes.len(), expect.len());
        for (i, e) in inodes.iter().enumerate() {
            assert_eq!(e.device, expect[i].device);
            assert_eq!(e.number, expect[i].number);
            assert_eq!(e.btime,  expect[i].btime);
        }
    }
    {
        let tag_names = taginode::list_tags(&connection);
        let expect = vec!["basketball", "chicken", "ikun", "man"];
        let expect: Vec<String> = expect.iter().map(|s| s.to_string()).collect();
        assert_eq!(expect, tag_names);
    }

    {
        let tags = taginode::get_tags(&connection, 
            INode{ device: 16777220, number: 12951634036, btime: Some(1665935055) },
        );
        let expect = vec![ "ikun", "basketball", "man", "chicken"];
        let expect: Vec<String> = expect.iter().map(|s| s.to_string()).collect();
        assert_eq!(expect, tags);
    }

    {
        let tag_names = taginode::list_tags(&connection);
        let expect = vec!["basketball", "chicken", "ikun", "man"];
        let expect: Vec<String> = expect.iter().map(|s| s.to_string()).collect();
        assert_eq!(expect, tag_names);
    }
}

#[test]
fn t_usage() {
	use std::collections::{BTreeMap};
	use taginode::opt::OptArg;
    let opt_check = BTreeMap::from([
        (b'f', (OptArg::Mandatory("/.taginode.db"), "-f <db>        specify db path to store data, default ~/.taginode.db"  )),
        (b'd', (            OptArg::Mandatory("."), "-d <directory> specify path to search file by tags, default \".\""     )),
        (b'v', (                      OptArg::None, "-v             verbose"                                                )),
    ]);
    let usage = taginode::opt::usage(&opt_check);
	println!("{usage}");
}

#[test]
fn t_opt() {
	use std::collections::{BTreeMap};
	use taginode::opt::OptArg;
    let opt_check = BTreeMap::from([
        (b'f', (OptArg::Mandatory("/.taginode.db"), "-f <db>        specify db path to store data, default ~/.taginode.db"  )),
        (b'd', (            OptArg::Mandatory("."), "-d <directory> specify path to search file by tags, default \".\""     )),
        (b'v', (                      OptArg::None, "-v             verbose"                                                )),
    ]);

    let args: Vec<String> = vec![
		String::from("taginode-cli"),
		// String::from("-f"),
		// String::from("/tmp/a.db"),
		String::from("search"),
		String::from("-d/home/"),
		String::from("hello,world"),
		// String::from("-f"),
		// String::from("/home/a.db"),
	];
    let (options, operands) = 
        taginode::opt::get_opt_per(&args[1..], &opt_check).unwrap_or_else(|err| {
            eprintln!("{}: {}",args[0], err);
            std::process::exit(1);
        });
		
	println!("{:?}", options);
	println!("{:?}", operands);
}