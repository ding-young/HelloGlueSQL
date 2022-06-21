use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Contributor;",
        "CREATE TABLE Contributor (name TEXT);",
        "INSERT INTO Contributor VALUES ('SeoYoung');",
        "SELECT * from Contributor;",
        "ALTER TABLE Contributor ADD COLUMN age INTEGER NULL;",
        "UPDATE Contributor SET age=22 WHERE name='SeoYoung';",
        "SELECT * from Contributor;"
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}