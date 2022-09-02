use sqlite3::Connection;

pub fn init(db_file: &str) ->  Connection {
    let connection = sqlite3::open(db_file).unwrap();
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
	
	connection
}