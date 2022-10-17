use taginode::INode;

#[test]
fn t() {
    let connection = taginode::sql::init(":memory:");
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

    let tag_names = vec![ "ikun", "basketball", "ikun", "chicken"];
    let inodes = taginode::get_inodes(&connection, &tag_names);
	println!("{:?}", inodes);
    let expect = vec![INode { device:16777220, number: 12951634036, btime: Some(1665935055) }];
    for (i, e) in inodes.iter().enumerate() {
    	assert_eq!(e.device, expect[i].device);
    	assert_eq!(e.number, expect[i].number);
    	assert_eq!(e.btime,  expect[i].btime);
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
	assert_eq!(expect, tags)
    }

}