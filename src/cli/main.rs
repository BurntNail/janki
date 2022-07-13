use janki::{db::AnkiGame, item::Fact, storage::FileStorage};
use std::{collections::HashMap, io::stdin, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage: FileStorage = "./janki_db.json".into();

    let map = {
        let mut hm = HashMap::new();
        for i in 1..11 {
            hm.insert(i, Duration::from_secs(i as u64 * 30));
        }
        hm
    };

    let mut anki = AnkiGame::new(storage, map)?;

    let mut input = String::new();
    loop {
        //TODO: this but w/ clap
        input.clear();
        println!("Add, Test or Exit - [atE]: ");
        stdin().read_line(&mut input)?;

        match input.trim() {
            "a" => {
                let mut term = String::new();
                let mut def = String::new();
                println!("Enter a term: ");
                stdin().read_line(&mut term)?;
                println!("Add a definition: ");
                stdin().read_line(&mut def)?;

                anki.add_card(Fact::new(term.trim(), def.trim()));
            }

            "t" => {
                let mut item = anki.get_card();
                let mut answer = String::new();

                println!("What is the definition of {}", item.term);
                stdin().read_line(&mut answer)?;

                if answer.trim() == item.definition {
                    println!("Correct!");
                    item.was_succesful = Some(true);
                } else {
                    println!("Wrong - the answer is {}", item.definition);
                    item.was_succesful = Some(false);
                }
            }
            _ => break,
        }

        println!("\n\n\n");
    }

    Ok(())
}
