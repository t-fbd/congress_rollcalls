use indicatif::{ProgressBar, ProgressStyle};
use crate::responses::*;
use std::io::{self, Write};
use std::fs::File;
use walkdir::WalkDir;
use std::error::Error;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;


lazy_static! {
    pub static ref JSON_FILE_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

/// Inserts a `UnifiedVote` into the `CombinedData` structure based on the specified mode.
pub async fn insert_into_database(
    vote: &UnifiedVote,
    combined_data: &mut CombinedData,
) -> Result<(), Box<dyn Error>> {
    // Ensure the chamber exists
    let chamber_entry = combined_data
        .chambers
        .entry(vote.chamber.to_lowercase())
        .or_insert_with(ChamberData::default);

    // Ensure the congress exists
    let congress_entry = chamber_entry
        .congresses
        .entry(vote.congress_number)
        .or_insert_with(CongressData::default);

    // Ensure the session exists
    let session_entry = congress_entry
        .sessions
        .entry(vote.session_number)
        .or_insert_with(SessionData::default);

    // Find the RollCallData for the given rollcall_number
    if let Some(rollcall_data) = session_entry.rollcalls.iter_mut().find(|rc| rc.rollcall_number == vote.rollcall_number) {
       // Append the vote to the existing RollCallData
       rollcall_data.vote_casts.push(vote.clone());
    } else {
       // Create a new RollCallData and add it to the sessions
       let new_rollcall = RollCallData {
           rollcall_number: vote.rollcall_number,
           vote_date: vote.vote_date.clone(),
           vote_question: vote.vote_question.clone(),
           vote_result: vote.vote_result.clone(),
           vote_casts: vec![vote.clone()],
       };
       session_entry.rollcalls.push(new_rollcall);
    }

    println!(
        "✅ Added vote to {} -> {} -> {} -> {}: {:?}",
        vote.chamber, vote.congress_number, vote.session_number, vote.rollcall_number, vote
    );

    Ok(())
}

/// Processes all JSON vote files and aggregates them into `CombinedData`.
pub async fn process_vote_files_json() -> Result<CombinedData, Box<dyn Error>> {
    let base_path = "data/json";

    // Collect all JSON file paths
    let mut json_files = Vec::new();
    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|s| s.to_str()) == Some("json")
        {
            json_files.push(entry.path().to_path_buf());
        }
    }

    let total_files = json_files.len();
    println!("📂 Total JSON files to process: {}", total_files);

    // Initialize the progress bar
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")?
            .progress_chars("=> "),
    );
    pb.set_message("📊 Processing files");

    let mut combined_data = CombinedData::default();

    // Iterate over the collected files with progress updates
    for path in json_files {
        let path_str = path.to_str().unwrap_or("❓ Invalid path");

        // Extract parts of the path
        // data/json/{congress_number}/{chamber}/{session_number}/{file}.json
        let parts: Vec<&str> = if cfg!(windows) {
            path_str.split('\\').collect()
        } else {
            path_str.split('/').collect()
        };

        if parts.len() != 6 {
            println!("⚠️ Unexpected path format: {}", path_str);
            pb.inc(1);
            continue;
        }

        let congress_number: u16 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid congress number in path: {}", path_str);
                pb.inc(1);
                continue;
            }
        };

        let chamber = parts[3].to_lowercase();
        let session_number: u8 = match parts[4].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid session number in path: {}", path_str);
                pb.inc(1);
                continue;
            }
        };

        let file_name = parts[5];
        let rollcall_number: u32 = match file_name
            .trim_end_matches(".json")
            .split('_')
            .last()
            .unwrap_or("0")
            .parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid rollcall number in file name: {}", file_name);
                pb.inc(1);
                continue;
            }
        };

        // Open and parse the JSON file
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                println!("❌ Failed to open file {}: {}", path_str, e);
                pb.inc(1);
                continue;
            }
        };
        println!("📄 Processing file: {}", path_str);

        match chamber.as_str() {
            "house" => {
                let house_file: HouseFile = match serde_json::from_reader(file) {
                    Ok(hf) => hf,
                    Err(e) => {
                        println!("❌ Failed to parse House JSON in {}: {}", path_str, e);
                        pb.inc(1);
                        continue;
                    }
                };
                let metadata = house_file.rollcall_vote.vote_metadata;

                let vote_data = if let Some(vd) = house_file.rollcall_vote.vote_data {
                    vd.clone()
                } else {
                    println!("⚠️ Missing vote data in file: {}", path_str);
                    VoteDataHouse::default()
                };

                for recorded_vote in vote_data.recorded_vote.unwrap_or_default() {
                    let legislator = match &recorded_vote.legislator {
                        Some(l) => l.clone(),
                        None => {
                            println!("⚠️ Missing legislator information in file: {}", path_str);
                            continue;
                        }
                    };

                    let legislator_name = match &legislator.unaccented_name {
                        Some(name) => name.clone(),
                        _ => "❓ Unknown".to_string(),
                    };

                    let unified_vote = UnifiedVote {
                        congress_number,
                        chamber: chamber.clone(),
                        session_number,
                        rollcall_number,
                        vote_date: metadata
                            .action_date
                            .to_string(),
                        vote_question: metadata
                            .vote_question
                            .to_string(),
                        vote_result: metadata
                            .vote_result
                            .to_string(),
                        legislator_id: legislator
                            .name_id
                            .clone()
                            .unwrap_or_else(|| "None".to_string()),
                        legislator_name,
                        party: legislator.party.clone().unwrap_or_else(|| "None".to_string()),
                        state: legislator.state.clone().unwrap_or_else(|| "None".to_string()),
                        vote_cast: recorded_vote
                            .vote
                            .clone()
                            .unwrap_or_else(|| "None".to_string())
                            .to_string(),
                    };

                    // Insert into database or JSON file based on mode
                    if let Err(e) = insert_into_database(&unified_vote, &mut combined_data).await {
                        println!("❌ Failed to insert vote: {}", e);
                    }
                }
            }
            "senate" => {
                let senate_file: SenateFile = match serde_json::from_reader(file) {
                    Ok(sf) => sf,
                    Err(e) => {
                        println!("❌ Failed to parse Senate JSON in {}: {}", path_str, e);
                        pb.inc(1);
                        continue;
                    }
                };
                let metadata = senate_file.roll_call_vote;

                let congress_number_senate = metadata.congress.parse::<u16>().unwrap_or(congress_number);

                let session_number_senate = metadata.session.parse::<u8>().unwrap_or(session_number);

                let rollcall_number_senate = match &metadata.vote_number {
                    Some(vn) => vn.parse::<u32>().unwrap_or(rollcall_number),
                    None => rollcall_number,
                };

                if let Some(members) = &metadata.members {
                    for member in members.member.clone().unwrap_or_default() {
                        let unified_vote = UnifiedVote {
                            congress_number: congress_number_senate,
                            chamber: chamber.clone(),
                            session_number: session_number_senate,
                            rollcall_number: rollcall_number_senate,
                            vote_date: metadata
                                .vote_date
                                .to_string(),
                            vote_question: metadata
                                .vote_question_text
                                .to_string(),
                            vote_result: metadata
                                .vote_result
                                .clone()
                                .unwrap_or_else(|| "None".to_string()),
                            legislator_id: member
                                .lis_member_id
                                .clone()
                                .unwrap_or_else(|| "None".to_string()),
                            legislator_name: member
                                .member_full
                                .clone()
                                .unwrap_or_else(|| "None".to_string()),
                            party: member.party.clone().unwrap_or_else(|| "None".to_string()),
                            state: member.state.clone().unwrap_or_else(|| "None".to_string()),
                            vote_cast: if let Some(vote) = member.vote_cast {
                                vote.to_string()
                            } else {
                                "None".to_string()
                            },
                        };

                        // Insert into database or JSON file based on mode
                        if let Err(e) = insert_into_database(&unified_vote, &mut combined_data).await {
                            println!("❌ Failed to insert vote: {}", e);
                        }
                    }
                } else {
                    println!("⚠️ No members found in Senate file: {}", path_str);
                }
            }
            _ => {
                println!("⚠️ Unknown chamber: {}", chamber);
            }
        }

        // Increment the progress bar after processing each file
        pb.inc(1);
    }

    // Finish the progress bar
    pb.finish_with_message("✅ Processing complete");

    Ok(combined_data)
}

/// Custom writer to track progress during serialization.
struct ProgressWriter<W: Write> {
    inner: W,
    pb: ProgressBar,
}

impl<W: Write> Write for ProgressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.pb.inc(bytes_written as u64);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Serializes the `CombinedData` into a JSON file with progress indication.
pub fn serialize_combined_data(combined_data: &CombinedData) -> Result<(), Box<dyn Error>> {
    // Step 1: Serialize to a buffer to determine total size
    let serialized = serde_json::to_vec_pretty(combined_data)?;
    let total_size = serialized.len() as u64;

    // Step 2: Initialize the progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({percent}%) {msg}")?
            .progress_chars("=> "),
    );
    pb.set_message("🔄 Serializing data");

    // Step 3: Create the file and wrap it with ProgressWriter
    let file = match File::create("full_data/votes.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Failed to create file 'full_data/votes.json': {}", e);
            return Err(Box::new(e));
        }
    };
    println!("📁 Creating file 'full_data/votes.json'...");

    let mut progress_writer = ProgressWriter {
        inner: file,
        pb: pb.clone(),
    };

    // Step 4: Write the serialized data to the file using ProgressWriter
    if let Err(e) = progress_writer.write_all(&serialized) {
        eprintln!("❌ Failed to write to 'full_data/votes.json': {}", e);
        return Err(Box::new(e));
    }
    if let Err(e) = progress_writer.flush() {
        eprintln!("❌ Failed to flush data to 'full_data/votes.json': {}", e);
        return Err(Box::new(e));
    }

    // Step 5: Finish the progress bar
    pb.finish_with_message("✅ Serialization complete");

    println!("🎉 'full_data/votes.json' has been successfully created.");

    Ok(())
}
