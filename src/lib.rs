pub mod sql;

use std::collections::HashSet;
use sqlite3::Connection;
use sqlite3::Value;

#[derive(Debug)]
pub struct INode {
    pub device: u64,
    pub number: u64,
}

pub fn get_inodes(connection: &Connection, tag_names: &[&str]) -> Vec<INode> {
    let mut h = HashSet::new();
    for tag_name in tag_names {
        h.insert(tag_name);
    }
    let tag_names: Vec<&&str> = h.into_iter().collect();

    let mut inodes = Vec::new();
    let sql_str = format!(
        "
        SELECT DISTINCT b.device, b.number FROM 
        relation_tag_inode a 
        LEFT JOIN inodes b ON a.inode_id = b.id 
        LEFT JOIN tags c ON a.tag_id = c.id
        WHERE c.name IN ({}) 
        GROUP BY b.id HAVING COUNT(b.id) = {}
        ", 
        vec!["?"; tag_names.len()].join(","),
        tag_names.len(),
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
        inodes.push(INode {
            device: row[0].as_integer().unwrap() as u64,
            number: row[1].as_integer().unwrap() as u64,
        });
    }
    return inodes;
}

pub fn add(connection: &Connection, inodes: &[INode], tag_names: &[&str]) {
    {
        let sql_str = format!(
            "
            INSERT OR IGNORE INTO tags(name) VALUES({}); 
            ", 
            vec!["?"; tag_names.len()].join("), (")
        );
        let mut cursor = connection
            .prepare(&sql_str)
            .unwrap()
            .cursor();
        
        let mut sql_args = Vec::new();
        for tag_name in tag_names {
            sql_args.push(Value::String(tag_name.to_string()));
        }
        cursor.bind(&sql_args).unwrap();
        while let Some(_) = cursor.next().unwrap() {}
    }
    {
        let sql_str = format!(
            "
            INSERT OR IGNORE INTO inodes(device,number) VALUES({}); 
            ", 
            vec!["?,?"; inodes.len()].join("), (")
        );
        let mut cursor = connection
            .prepare(&sql_str)
            .unwrap()
            .cursor();
        
        let mut sql_args = Vec::new();
        for inode in inodes {
            sql_args.push(Value::Integer(inode.device as i64));
            sql_args.push(Value::Integer(inode.number as i64));
        }
        cursor.bind(&sql_args).unwrap();
        while let Some(_) = cursor.next().unwrap() {}
    }
    let mut tag_ids = vec![0;0];
    {
        let sql_str = format!(
            "
            SELECT DISTINCT id FROM tags WHERE name IN ({}); 
            ", 
            vec!["?"; tag_names.len()].join(",")
        );
        let mut cursor = connection
            .prepare(&sql_str)
            .unwrap()
            .cursor();
        
        let mut sql_args = Vec::new();
        for tag_name in tag_names {
            sql_args.push(Value::String(tag_name.to_string()));
        }
        cursor.bind(&sql_args).unwrap();
        while let Some(row) = cursor.next().unwrap() {
            tag_ids.push(row[0].as_integer().unwrap());
        }
    }
    let mut inode_ids = vec![0;0];
    {
        let sql_str = format!(
            "
            SELECT DISTINCT id FROM inodes WHERE (device, number) IN (VALUES({})); 
            ", 
            vec!["?,?"; inodes.len()].join("), (")
        );
        let mut cursor = connection
            .prepare(&sql_str)
            .unwrap()
            .cursor();
        
        let mut sql_args = Vec::new();
        for inode in inodes {
            sql_args.push(Value::Integer(inode.device as i64));
            sql_args.push(Value::Integer(inode.number as i64));
        }
        cursor.bind(&sql_args).unwrap();

        while let Some(row) = cursor.next().unwrap() {
            inode_ids.push(row[0].as_integer().unwrap());
        }
    }

    {
        let sql_str = format!(
            "
            INSERT OR IGNORE INTO relation_tag_inode(tag_id,inode_id) VALUES({}); 
            ", 
            vec!["?,?"; tag_ids.len()*inode_ids.len()].join("), (")
        );
        let mut cursor = connection
            .prepare(&sql_str)
            .unwrap()
            .cursor();
        
        let mut sql_args = Vec::new();
        for inode_id in inode_ids {
            for tag_id in &tag_ids {
                sql_args.push(Value::Integer(*tag_id));
                sql_args.push(Value::Integer(inode_id));
            }
        }
        cursor.bind(&sql_args).unwrap();
        while let Some(_) = cursor.next().unwrap() {}
    }
}

pub fn list_tags(connection: &Connection) -> Vec<String> {
    let sql_str = "SELECT DISTINCT name FROM tags"; 
    let mut cursor = connection
        .prepare(&sql_str)
        .unwrap()
        .cursor();

    let mut tag_names = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        tag_names.push(row[0].as_string().unwrap().to_owned());
    }
    return tag_names;
}