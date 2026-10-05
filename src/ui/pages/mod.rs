    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        // TODO: print out epics using get_column_string(). also make sure the epics are sorted by id

        let db_state = self.db.database.read_db()?;

        for (k, v) in db_state.epics.iter() {
            println!("k: {:?}, v: {:?}", k, v);

            let status_mapping = match v.status {
                Open => "Open",
                Closed => "Closed",
                InProgress => "InProgress",
                Resolved => "Resolved",
            };

