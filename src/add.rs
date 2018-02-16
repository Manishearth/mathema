//! New subcommand.

use crate::prelude::*;

crate fn add(directory: &Path, file: String, force: bool) -> Fallible<()> {
    let repo = &mut MathemaRepository::open(directory)?;

    let file = &PathBuf::from(file);

    let mut cards =
        cards::parse_cards_file(file).with_context(|_| MathemaErrorKind::AccessingFile {
            file: file.display().to_string(),
        })?;

    let repo_path = repo.path_in_repo(file)?;

    let is_new = !repo.database().contains_card_file(&repo_path);

    // If the file has not yet been added, there should be no UUIDs.
    if is_new && !force {
        for c in &cards {
            if c.uuid.is_some() {
                throw!(MathemaErrorKind::PreexistingUUID {
                    file: file.display().to_string(),
                    line: c.start_line,
                });
            }
        }
    }

    // Otherwise, we can assign UUIDs to each card.
    let mut uuids_assigned = 0;
    for c in &mut cards {
        if c.uuid.is_none() {
            uuids_assigned += 1;
            c.uuid = Some(Uuid::fresh());
        }
    }

    // Now we can write the card file back out, with the UUIDs assigned.
    cards::write_cards_file(file, &cards)?;

    // Assuming that was successful, we can update the database.
    if is_new {
        println!("`{}` added to database.", repo_path.display());
        repo.database_mut().card_files.push(repo_path);
    } else {
        println!("`{}` already found in database.", repo_path.display());
    }

    // Finally, write everything back out.
    repo.write_database()?;

    if uuids_assigned == 1 {
        println!("1 new card found.");
    } else {
        println!("{} new cards found.", uuids_assigned);
    }

    Ok(())
}
