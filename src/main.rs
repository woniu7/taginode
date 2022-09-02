
fn main() {
    let connection = taginode::sql::init("taginode.db");
    let tag_names = vec!["beauty", "ikun", "code", "ikun"];
    // taginode::add(&connection, &[123, 456], &["ikun", "basketball"]);
    let inode_numbers = taginode::get_inodenums(&connection, &tag_names);
    println!("inode_numbers: {:?}", inode_numbers);
}

