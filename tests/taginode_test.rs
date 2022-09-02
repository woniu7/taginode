use taginode::INode;

#[test]
fn t() {
    let connection = taginode::sql::init(":memory:");
    {
	let inodes = vec![
		INode{ device: 16777220, number: 12951634006 },
		INode{ device: 16777220, number: 12951634036 },
	];
	taginode::add(&connection, &inodes, &["ikun", "basketball", "man"]);
    }
    {
	let inodes = vec![
		INode{ device: 16777221, number: 12951634006 },
		INode{ device: 16777220, number: 12951634036 },
	];
	taginode::add(&connection, &inodes, &["ikun", "basketball", "chicken"]);
    }

    let tag_names = vec![ "ikun", "basketball", "ikun", "chicken"];
    let inode_numbers = taginode::get_inodenums(&connection, 16777220, &tag_names);
    assert_eq!(inode_numbers, vec![12951634036]);
}