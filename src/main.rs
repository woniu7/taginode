mod main_test;

fn main() {
    let connection = sqlite3::open("taginode.db").unwrap();

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS tags (
                `id` INTEGER PRIMARY KEY, 
                `name` TEXT NOT NULL UNIQUE CHECK(name <> ''), 
                `num` INTEGER NOT NULL DEFAULT 0, 
                `create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
                `device` INTEGER NOT NULL CHECK(device <> 0),
                `number` INTEGER NOT NULL CHECK(number <> 0), 
                `num` INTEGER NOT NULL DEFAULT 0, 
                `create_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(device, number)
            );
            INSERT INTO inodes (device, number) VALUES (123432, 89234);
            ",
        )
        .unwrap();
    connection
        .iterate("SELECT * FROM tags", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}

pub fn get_inodenums(tag_ids: &[i32]) -> Vec<i32> {
    let mut inodenums = Vec::new();
    for tag_id in tag_ids {
        inodenums.push(tag_id + 1);
    }
    return inodenums;
}


