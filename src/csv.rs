use crate::item::Fact;
use std::{error::Error, fs::File, io::Write};

#[instrument]
pub fn read_in(path: &str) -> Result<Vec<Fact>, Box<dyn Error>> {
    info!("Reading in");

    let mut rdr = csv::Reader::from_path(path)?;
    let records: Result<Vec<_>, _> = rdr.records().into_iter().collect();
    Ok(records?
        .into_iter()
        .filter_map(|record| {
            record
                .get(0)
                .and_then(|term| {
                    record
                        .get(1)
                        .map(|def| Some(Fact::new(term.to_string(), def.to_string())))
                })
                .flatten()
        })
        .collect())
}

#[instrument(skip(db), fields(db_len = ?db.len()))]
pub fn write_out(path: &str, mut db: Vec<Fact>) -> Result<(), Box<dyn Error>> {
    db.sort();
    db.dedup();
    info!("Writing list");

    let tbw: String = db
        .into_iter()
        .map(|f| format!("{},{}\n", f.term, f.definition))
        .collect();
    let mut file = File::create(path)?;
    write!(file, "{tbw}")?;

    Ok(())
}
