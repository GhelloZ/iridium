use iridium::SqliteStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = SqliteStore::open_default()?;

    Ok(())
}
