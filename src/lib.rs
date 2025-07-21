pub mod db {
    use std::{fs::File, io::Read, io::Seek, io::Write};

    use anyhow::Result;

    pub async fn get_users() -> Result<Vec<String>> {
        let mut file = File::open("fakedb.txt")?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let users = buf
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        Ok(users)
    }

    pub async fn add_user(user: String) -> Result<Vec<String>> {
        let mut file = File::options().append(true).read(true).open("fakedb.txt")?;
        writeln!(&mut file, "{user}")?;

        file.rewind()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let users = buf
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        Ok(users)
    }
}
