use datajoint_core;

pub mod mysql_tests {
    // - Make a table with every supported data type 
    // - Insert multiple rows and then select those rows (also use placeholders)
    // - Confirm the rows that were retrieved are equivalent to those that were inserted 

    #[test]
    fn test_successful_connection_to_db() {
        // let settings = "mysql://username:password@tutorial-db.datajoint.io:3306/username_tutorial";
        // let mut con = Connection::new(settings.to_string());
        // con.connect().unwrap();
        // let mut curse = con.raw_query("SELECT * FROM `edwardg_tutorial`.`mouse`");
        // let rows = curse.fetch_all();
        // let r= rows.len();
        // print!("{}",r);choco install docker-desktop
        // print_row_vec(rows);

        let mut conn = datajoint_core::connection::Connection::new(datajoint_core::connection::ConnectionSettings::new());
        conn.settings.username = "root".to_string();
        conn.settings.password = "password".to_string();
        let result = conn.connect();
        assert!(result.is_ok()); 
    }

    #[test]
    fn test_unsuccessful_connection_to_db() {
        let mut conn = datajoint_core::connection::Connection::new(datajoint_core::connection::ConnectionSettings::new());
        conn.settings.username = "root".to_string();
        conn.settings.password = "wrong_password".to_string();
        let result = conn.connect();
        assert!(result.is_err()); 
    }
    
    #[test]
    fn test_insert_retrieve_int() {
        let mut conn = datajoint_core::connection::Connection::new(datajoint_core::connection::ConnectionSettings::new());
        conn.settings.username = "root".to_string();
        conn.settings.password = "password".to_string();
        let result = conn.connect();
        assert!(result.is_ok()); 
    }

    #[test]
    fn test_insert_multiple_rows_without_placeholders() {
        let mut conn = datajoint_core::connection::Connection::new(datajoint_core::connection::ConnectionSettings::new());
        conn.settings.username = "root".to_string();
        conn.settings.password = "password".to_string();
        let result = conn.connect();
        assert!(result.is_ok()); 
    }

    #[test]
    fn test_insert_multiple_rows_with_placeholders() {
        let mut conn = datajoint_core::connection::Connection::new(datajoint_core::connection::ConnectionSettings::new());
        conn.settings.username = "root".to_string();
        conn.settings.password = "password".to_string();
        let result = conn.connect();
        assert!(result.is_ok()); 
    }
}
