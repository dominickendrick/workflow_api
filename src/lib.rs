pub mod card;

pub mod merge_stuff {

    use std::str::from_utf8;

    use crate::card::card::Card;
    use automerge::transaction::CommitOptions;
    use automerge::transaction::Transactable;
    use automerge::AutomergeError;
    use automerge::ObjType;
    use automerge::{Automerge, ROOT};

    pub fn merge_docs(input_doc_1: &Vec<Card>, input_doc_2: &Vec<Card>) -> Option<Vec<Card>> {
        let mut doc1 = Automerge::new();
        let cards = doc1
            .transact_with::<_, _, AutomergeError, _, ()>(
                |_| CommitOptions::default().with_message("Add card".to_owned()),
                |tx| {
                    let cards = tx.put_object(ROOT, "cards", ObjType::List).unwrap();
                    for (index, card) in input_doc_1.iter().enumerate() {
                        let new_card = tx.insert_object(&cards, index, ObjType::Map)?;
                        tx.put(&new_card, "id", card.id.to_string())?;
                        tx.put(&new_card, "title", &card.title)?;
                        tx.put(&new_card, "state", &card.state)?;
                        tx.put(&new_card, "author", &card.author)?;
                        tx.put(&new_card, "editor", &card.editor)?;
                        tx.put(&new_card, "message", &card.message)?;
                    }
                    Ok(cards)
                },
            )
            .unwrap()
            .result;

        let mut doc2 = Automerge::new();
        doc2.merge(&mut doc1).unwrap();

        let binary = doc1.save();
        let mut doc2 = Automerge::load(&binary).unwrap();

        doc1.merge(&mut doc2).unwrap();

        let cards = doc2
            .transact_with::<_, _, AutomergeError, _, ()>(
                |_| CommitOptions::default().with_message("Add card".to_owned()),
                |tx| {
                    let cards = tx.put_object(ROOT, "cards", ObjType::List).unwrap();
                    for (index, card) in input_doc_2.iter().enumerate() {
                        let new_card = tx.insert_object(&cards, index, ObjType::Map)?;
                        tx.put(&new_card, "id", card.id.to_string())?;
                        tx.put(&new_card, "title", &card.title)?;
                        tx.put(&new_card, "state", &card.state)?;
                        tx.put(&new_card, "author", &card.author)?;
                        tx.put(&new_card, "editor", &card.editor)?;
                        tx.put(&new_card, "message", &card.message)?;
                    }
                    Ok(cards)
                },
            )
            .unwrap()
            .result;

        println!("{:?}", doc2.dump());

        for change in doc2.get_changes(&[]).unwrap() {
            let length = doc2.length_at(&cards, &[change.hash]);
            println!("{} {}", change.message().unwrap(), length);
        }
        if let Some(output) = doc2.get(cards, "cards").unwrap() {
            println!("this is output {:?}", output);
            let final_output: Vec<Card> =
                rocket::serde::json::from_str(&output.0.to_string()).unwrap();

            Some(final_output)
        } else {
            Some(vec![])
        }
    }
}
