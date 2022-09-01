//mod main_test;

use sqlite3::Connection;
use sqlite3::Value;

fn main() {
    let connection = sqlite3::open("taginode.db").unwrap();

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS tags (
                `id` INTEGER PRIMARY KEY, 
                `name` TEXT NOT NULL, 
                `inode_num` INTEGER NOT NULL DEFAULT 0, 
                `create_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(name),
                CHECK(name <> '')
            );
            ",
            // INSERT INTO tags (name) VALUES ('Alis');
            // INSERT INTO tags (name) VALUES ('');
            // INSERT INTO tags (name, num) VALUES ('Bob', 69);
        )
        .unwrap();
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS inodes (
                `id` INTEGER PRIMARY KEY, 
                `device` INTEGER NOT NULL,
                `number` INTEGER NOT NULL, 
                `tag_num` INTEGER NOT NULL DEFAULT 0, 
                `create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(device, number),
                CHECK(device <> 0 AND number <> 0)
            );
            ",
            //INSERT INTO inodes (device, number) VALUES (123432, 89234);
        )
        .unwrap();
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS relation_tag_inode (
                `id` INTEGER PRIMARY KEY, 
                `tag_id` INTEGER NOT NULL,
                `inode_id` INTEGER NOT NULL, 
                `create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tag_id, inode_id), 
                CHECK(tag_id <> 0 AND inode_id <> 0)
            );
            ",
            //INSERT INTO relation_tag_inode (tag_id, inode_id) VALUES (123432, 89234);
        )
        .unwrap();

    let tag_names = vec!["beauty", "ikun", "code", "ikun"];
    let inode_numbers = get_inodenums(&connection, &tag_names);
    println!("inode_numbers: {:?}", inode_numbers);
}

pub fn get_inodenums(connection: &Connection, tag_names: &[&str]) -> Vec<i64> {
    let mut inode_numbers = Vec::new();
    let sql_str = format!(
        "
        SELECT DISTINCT b.number, b.device FROM 
        relation_tag_inode a 
        LEFT JOIN inodes b ON a.inode_id = b.id 
        LEFT JOIN tags c ON a.tag_id = c.id
        WHERE c.name in ({}) AND b.id IS NOT NULL
        ", 
        vec!["?"; tag_names.len()].join(",")
    );
    let mut cursor = connection
        .prepare(&sql_str)
        .unwrap()
        .cursor();
    
    let sql_args: Vec<Value> = tag_names.iter().map(|&val| {
        Value::String(val.to_string())
    }).collect();
    cursor.bind(&sql_args).unwrap();
    println!("{}, {:?}", sql_str, sql_args);

    while let Some(row) = cursor.next().unwrap() {
        inode_numbers.push(row[0].as_integer().unwrap());
    }
    return inode_numbers;
}
