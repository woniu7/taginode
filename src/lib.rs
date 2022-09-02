pub mod sql;

use sqlite3::Connection;
use sqlite3::Value;

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
    // println!("{}, {:?}", sql_str, sql_args);

    while let Some(row) = cursor.next().unwrap() {
        inode_numbers.push(row[0].as_integer().unwrap());
    }
    return inode_numbers;
}

// pub fn add(connection: &Connection, inode_numbers: &[i32], tag_names: &[&str]) {
// }