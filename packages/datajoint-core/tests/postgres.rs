use datajoint_core::{
    common::DatabaseType,
    connection::{Connection, ConnectionSettings},
    results::TableRow,
};

#[test]
fn run_test() {
    let mut settings = ConnectionSettings::new();
    settings.database_type = DatabaseType::Postgres;
    settings.username = "postgres".to_string();
    settings.port = 5678;
    settings.password = "password".to_string();
    settings.database_name = "postgres".to_string();
    settings.use_tls = Some(true);
    let mut con = Connection::new(settings);
    con.connect().unwrap();

    con.execute_query("insert into tweet (text, owner_id) values ('hello world', 1234);");

    let cursor = &mut con.fetch_query("select * from tweet");
    let cursor = cursor;

    let rows: Vec<TableRow> = cursor.rest();
    for row in rows {
        for col in row.columns() {
            match row.try_decode(col) {
                Ok(data) => {
                    println!("{}", data)
                }
                Err(_) => {}
            }
        }
    }

    con.disconnect()
}
