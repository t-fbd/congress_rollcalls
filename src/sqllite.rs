// sqllite.rs

use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Error, Result};
use std::fs::File;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::task;
use sha2::{Sha256, Digest}; 

use crate::responses::*;

/// Function to initialize the database and create tables with appropriate constraints
async fn initialize_database(conn: &Connection) -> Result<()> {
    
    Ok(())
}

pub async fn generate_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}


async fn analyze_house_vote(
    tx: &rusqlite::Transaction<'_>,
    info_pulled: &(u16, u8, u32, String, u16),
    vote: &RollCallVoteHouse
) -> Result<()> {
    




    Ok(())
}


/// Asynchronous function to process vote files and insert into SQLite
/// If `single_file` is provided, only that file will be processed.
/// Otherwise, all JSON files in the `data/json` directory will be processed.
pub async fn process_vote_files_sql(single_file: Option<&str>) -> Result<()> {
    let base_path = "data/json";

    // Determine the list of JSON files to process
    let json_files: Vec<std::path::PathBuf> = match single_file {
        Some(file_path) => {
            let path = std::path::PathBuf::from(file_path);
            if path.is_file()
                && path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("json"))
                    .unwrap_or(false)
            {
                vec![path]
            } else {
                println!("⚠️ Provided file is not a valid JSON file: {}", file_path);
                vec![]
            }
        }
        None => {
            // Collect all JSON file paths
            WalkDir::new(base_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().and_then(|s| s.to_str()) == Some("json")
                })
                .map(|e| e.into_path())
                .collect()
        }
    };

    let total_files = json_files.len();

    if total_files == 0 {
        println!("📭 No JSON files to process.");
        return Ok(());
    }

    println!("📂 Total JSON files to process: {}", total_files);

    // Initialize the progress bar
    let pb = ProgressBar::new(total_files as u64);
    let pb_style = match ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
    {
        Ok(style) => style,
        Err(_) => {
            println!("❌ Failed to create progress bar style");
            return Err(Error::InvalidParameterName(
                "Failed to create progress bar style".to_string(),
            ));
        }
    };
    pb.set_style(pb_style.progress_chars("=> "));
    pb.set_message("Processing files");

    let pb_up = pb.clone();

    // Clone the list for moving into the blocking task
    let json_files_clone = json_files.clone();

    // Spawn a blocking task to handle database operations
    let handle = async move {
        // Initialize the database connection
        let mut conn = Connection::open("full_data/votes.db")?;
        initialize_database(&conn).await?;

        // Begin a transaction for batch insertion
        let tx = conn.transaction()?;

        for path in json_files_clone {
            let path_str = path.to_str().unwrap_or("Invalid path");

            // Extract parts of the path
            // data/json/{congress_number}/{chamber}/{session_number}/{file}.json
            let parts: Vec<&str> = if cfg!(windows) {
                path_str.split('\\').collect()
            } else {
                path_str.split('/').collect()
            };

            if parts.len() != 6 {
                println!("📁 Unexpected path format: {}", path_str);
                pb_up.inc(1);
                continue;
            }

            let congress_number: u16 = match parts[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Invalid congress number in path: {}", path_str);
                    pb_up.inc(1);
                    continue;
                }
            };

            let chamber = parts[3].to_lowercase();
            let session_number: u8 = match parts[4].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Invalid session number in path: {}", path_str);
                    pb_up.inc(1);
                    continue;
                }
            };

            let file_name = parts[5];
            // Extract the rollcall number and the year from the file name
            // Format: {year}_{rollcall_number}.json
            
            let (year, rollcall_number): (u16, u32) = match file_name
                .trim_end_matches(".json")
                .split('_')
                .collect::<Vec<&str>>()
                .as_slice()
            {
                [year, rollcall] => match (year.parse(), rollcall.parse()) {
                    (Ok(y), Ok(r)) => (y, r),
                    _ => {
                        println!("❌ Invalid year or rollcall number in file name: {}", file_name);
                        pb_up.inc(1);
                        continue;
                    }
                },
                _ => {
                    println!("❌ Invalid file name format: {}", file_name);
                    pb_up.inc(1);
                    continue;
                }
            };
            
            let info_pulled: (u16, u8, u32, String, u16) = (congress_number, session_number, rollcall_number, chamber.to_string(), year);

            // Open and parse the JSON file
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    println!("❌ Failed to open file {}: {}", path_str, e);
                    pb_up.inc(1);
                    continue;
                }
            };
            println!("🔍 Processing file: {}", path_str);

            match chamber.as_str() {
                "house" => {
                    let vote_parent: HouseFile = match serde_json::from_reader(file) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("❌ Failed to parse JSON file {}: {}", path_str, e);
                            pb_up.inc(1);
                            continue;
                        }
                    };

                    let vote = vote_parent.rollcall_vote;

                    crate::sql_house::get_house_vote(&tx, &info_pulled, &vote).await?;

                }
                "senate" => {
                    let vote_parent: SenateFile = match serde_json::from_reader(file) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("❌ Failed to parse JSON file {}: {}", path_str, e);
                            pb_up.inc(1);
                            continue;
                        }
                    };

                    let vote = vote_parent.roll_call_vote;

                    let vote_info = crate::sql_senate::get_senate_vote(&tx, &info_pulled, &vote).await;

                    let (congress_number, session_number, rollcall_number, chamber, year, vote_hash, vote_date, vote_modify_date, vote_question, vote_question_text, vote_document_text, vote_result, vote_title, majority_requirement, documents, amendments, yay, nay, present, absent, tie_breaker_name, tie_breaker_paired, senate_members, gen_members) = vote_info.clone();

                    let vote_metadata = (
                        congress_number,
                        session_number,
                        rollcall_number,
                        chamber.clone(),
                        year,
                        vote_hash.clone(),
                        vote_date.clone(),
                        vote_modify_date.clone(),
                        vote_question.clone(),
                        vote_question_text.clone(),
                        vote_document_text.clone(),
                        vote_result.clone(),
                        vote_title.clone(),
                        documents.clone(),
                        amendments.clone(),
                        majority_requirement.clone(),
                        (   yay,
                            nay,
                            present,
                            absent,
                            tie_breaker_name.clone(),
                            tie_breaker_paired.clone()
                        )
                    );

                    let count_data = (
                        vote_hash.clone(),
                        congress_number,
                        session_number,
                        rollcall_number,
                        chamber.clone(),
                        year,
                        yay,
                        nay,
                        present,
                        absent,
                        tie_breaker_name.clone(),
                        tie_breaker_paired.clone()
                    );

                    let member_data = (
                        vote_hash.clone(),
                        congress_number,
                        session_number,
                        rollcall_number,
                        chamber.clone(),
                        year,
                        senate_members.clone(),
                        gen_members.clone()
                    );

                    println!("{}", congress_number);
                    println!("{}", session_number);
                    println!("{}", rollcall_number);
                    println!("{}", chamber);
                    println!("{}", year);
                    println!("{}", vote_hash);
                    println!("{}", vote_date);
                    println!("{}", vote_modify_date);
                    println!("{}", vote_question);
                    println!("{}", vote_question_text);
                    println!("{}", vote_document_text);
                    println!("{}", vote_result);
                    println!("{}", vote_title);
                    println!("{}", majority_requirement);
                    for doc in documents {
                        println!("{}", serde_json::to_string_pretty(&doc).unwrap());
                    }
                    for amend in amendments {
                        println!("{}", serde_json::to_string_pretty(&amend).unwrap());
                    }
                    println!("{}", yay);
                    println!("{}", nay);
                    println!("{}", present);
                    println!("{}", absent);
                    println!("{}", tie_breaker_name);
                    println!("{}", tie_breaker_paired);
                    println!("{:#?}", senate_members);
                    println!("{:#?}", gen_members);

                    // Insert the vote metadata
                    crate::sql_senate::insert_vote_metadata_senate(&tx, vote_metadata).await;
                    // Insert the vote count data
                    crate::sql_senate::insert_vote_count_senate(&tx, count_data).await;
                    // Insert the member data
                    crate::sql_senate::insert_vote_members_senate(&tx, member_data).await;

                    
                }
                _ => {
                    println!("❌ Invalid chamber in path: {}", path_str);
                    pb_up.inc(1);
                    continue;
                }
            }

            pb_up.inc(1);
        }

        // Commit the transaction
        tx.commit()?;
        Ok::<(), Error>(println!("📦 Database transaction committed successfully."))
    };

    // Await the blocking task
    match handle.await {
        Ok(result) => result,
        Err(e) => {
            println!("❌ Task panicked: {}", e);
            return Err(Error::InvalidQuery);
        }
    }

    pb.finish_with_message("✅ Processing complete");

    Ok(())
}
