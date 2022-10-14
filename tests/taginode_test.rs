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
		INode{ device: 16777220, number: 12951634036, btime: None },
	];
	taginode::add(&connection, &inodes, &["basketball", "chicken"]);
    }

    let tag_names = vec![ "ikun", "basketball", "ikun", "chicken"];
    let inodes = taginode::get_inodes(&connection, &tag_names);
    let expect = vec![INode { device:16777220, number: 12951634036, btime: None }];
    for (i, e) in inodes.iter().enumerate() {
    	assert_eq!(e.device, expect[i].device);
    	assert_eq!(e.number, expect[i].number);
    }
}