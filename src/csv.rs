use crate::item::Fact;
use std::{error::Error, fs::File, io::Write};

///Function to read in a `Vec<Fact>` from a set path.
///
///Can return an [`std::error::Error`] if either the file cannot be read or parsed
#[instrument]
pub fn read_in(path: &str) -> Result<Vec<Fact>, Box<dyn Error>> {
    info!("Reading in");

    let contents = std::fs::read_to_string(path)?;
    read_in_string(contents)
}

#[instrument]
fn read_in_string (contents: String) -> Result<Vec<Fact>, Box<dyn Error>> {
    info!("Internal reading in");

    let records = contents.split("\n"); //TOOD: actual parsing
    
    Ok(records?
        .into_iter()
        .map(|(term, definition)| {
            Some(Fact::new(term.to_string(), def.to_string()))
        })
        .collect())
}

///Writes out a `Vec<Fact>` to a set path, overwriting any current contents.
/// 
///Can return an [`std::io::Error`] if the file cannot be written
#[instrument(skip(db), fields(db_len = ?db.len()))]
pub fn write_out(path: &str, mut db: Vec<Fact>) -> Result<(), std::io::Error> {
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

#[cfg(test)]
mod tests {
    pub fn test_read_in () {
        let 
    }
}