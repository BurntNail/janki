use crate::item::Fact;
use std::io::{Read, Write};
use thiserror::Error as TError;

///Custom Error type for parsing CSVs using `thiserror`
#[derive(TError, Debug)]
pub enum CSVParseError {
    #[error("Not enough columns per row to fill a fact - on input {0:?}")]
    NotEnoughCols(String),
    #[error("Too many columns - ambiguous how to fill a fact - on input {0:?}")]
    TooManyCols(String),
    #[error("Error reading in file: {0}")]
    ReadError(#[from] std::io::Error),
}

///Function to read in a `Vec<Fact>` from a set path.
///
///Can return an [`std::error::Error`] if either the file cannot be read or parsed
#[instrument(skip(reader))]
pub fn read_in(mut reader: impl Read) -> Result<Vec<Fact>, CSVParseError> {
    info!("Reading in");

    let mut contents = Default::default();
    reader.read_to_string(&mut contents)?;

    Ok(read_in_string(contents)?)
}

#[instrument]
fn read_in_string(contents: String) -> Result<Vec<Fact>, CSVParseError> {
    info!("Reading in Raw");

    let mut v = vec![];

    for line in contents.trim().lines() {
        let mut els: Vec<String> = line.split(",").map(ToString::to_string).collect();
        if els.len() < 2 {
            return Err(CSVParseError::NotEnoughCols(line.to_string()));
        } else if els.len() > 2 {
            return Err(CSVParseError::TooManyCols(line.to_string()));
        }

        v.push(Fact::new(
            els.remove(0).trim().to_string(),
            els.remove(0).trim().to_string(),
        ))
    }

    info!("Read in {} facts", v.len());
    Ok(v)
}

///Writes out a `Vec<Fact>` to a set path, overwriting any current contents.
///
///Can return an [`std::io::Error`] if the file cannot be written
#[instrument(skip(writer, db), fields(db_len = ?db.len()))]
pub fn write_out(mut writer: impl Write, mut db: Vec<Fact>) -> Result<(), std::io::Error> {
    db.dedup();
    info!("Writing list");

    let tbw: String = db
        .into_iter()
        .map(|f| format!("{},{}\n", f.term, f.definition))
        .collect();

    write!(writer, "{tbw}")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        csv::{read_in_string, write_out, CSVParseError},
        item::Fact,
        string_wrapper::StringWrapper,
    };

    fn f(a: impl Into<String>, b: impl Into<String>) -> Fact {
        Fact::new(a.into(), b.into())
    }

    #[test]
    pub fn test_read_in() {
        {
            let fine_contents = r#"a,b
c,d
1,0
hello world,goodbye1
👍,:thumbs_up:
Привет,мир
"#;
            let fine_answers = vec![
                f("a", "b"),
                f("c", "d"),
                f("1", "0"),
                f("hello world", "goodbye1"),
                f("👍", ":thumbs_up:"),
                f("Привет", "мир"),
            ];
            assert!(matches!(
                read_in_string(fine_contents.to_string()),
                Ok(ans) if ans == fine_answers
            ));
        }

        {
            let too_few = r#"a,b
a
c,d
"#;
            let nec = read_in_string(too_few.to_string());
            if let Err(CSVParseError::NotEnoughCols(s)) = nec {
                if s != String::from("a") {
                    panic!("Incorrect Data: {s}");
                }
            } else {
                panic!("Doesn't match! {nec:?}");
            }
        }

        {
            let too_many = r#"a,b
a,b,c
a,b
"#;
            let tmc = read_in_string(too_many.to_string());
            if let Err(CSVParseError::TooManyCols(s)) = tmc {
                if s != String::from("a,b,c") {
                    panic!("Incorrect Data: {s}")
                }
            } else {
                panic!("Doesn't match! {tmc:?}");
            }
        }
    }

    #[test]
    pub fn test_write_out() {
        let unicode = vec![
            f("a", "b"),
            f("c", "d"),
            f("1", "0"),
            f("hello world", "goodbye1"),
            f("👍", ":thumbs_up:"),
            f("Привет", "мир"),
        ];
        let mut unicode_st = StringWrapper::default();
        let correct_unicode = r#"a,b
c,d
1,0
hello world,goodbye1
👍,:thumbs_up:
Привет,мир
"#;
        write_out(&mut unicode_st, unicode).unwrap();
        assert_eq!(unicode_st.to_inner(), correct_unicode.to_string());
    }
}
