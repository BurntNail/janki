use janki::{
    file_storage::NamedFileStorage,
    game::{default_sag, AnkiGame},
    item::Fact,
};
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage: NamedFileStorage = "./janki_db.json".into();

    let map = default_sag();

    let mut anki = AnkiGame::new(storage, map)?;

    let mut input = String::new();
    loop {
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
                let mut item = anki.get_item_guard().unwrap().0; //not important for cli version
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
