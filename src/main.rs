use congress_rolls::endpoints::CongressEndpoint as Congress;
use congress_rolls::client::RollCallClient as Client;
use congress_rolls::{sqllite, json_master};
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
    }

    match args[1].as_str() {
        "download_xml" => {
            // Example command: congress_rolls download_xml house 116 1 10
            println!("📥 Starting XML download...");
            xml_download().await;
            println!("✅ XML download completed!");
            Ok(())
        }
        "process_votes" => {
            // Example command: congress_rolls process_votes json
            if args.len() < 3 {
                println!("❗ Usage: congress_rolls process_votes <json|sql> <optional: file_path>");
                println!("💡 Notes: The file path is optional and can only be used with the SQL mode for adding new data to the database.");
                println!("📄 The JSON file must be fully reconstructed with the addition of new data.");
                std::process::exit(1);
            }

            let mode = &args[2];
            if mode != "json" && mode != "sql" {
                println!("🚫 Invalid mode specified. Use 'json' or 'sql'.");
                std::process::exit(1);
            }

            let file_path = if args.len() == 4 {
                Some(args[3].clone())
            } else {
                None
            };

            // Process the vote files based on the specified mode
            if mode == "json" {
                println!("📝 Processing votes in JSON mode...");
                let data = json_master::process_vote_files_json().await?;
                println!("✅ Vote processing completed successfully.");

                // Serialize the combined data to a JSON file
                json_master::serialize_combined_data(&data)?;
                println!("📁 Master JSON file 'votes.json' created successfully.");
            }

            if mode == "sql" {
                println!("💾 Starting SQL processing...");
                sqllite::process_vote_files_sql(file_path.as_deref()).await?;
            }

            println!("🎉 Vote processing completed successfully.");

            Ok(())
        }
        _ => {
            usage();
            Ok(())
        }
    }
}


/// Displays the usage instructions and exits the program.
fn usage() {
    let message = "
📜 Usage:
    congress_rolls download_xml <chamber> <congress_number> <session> <max_roll>
    congress_rolls process_votes <json|sql> <optional: file_path>
            - json: Process votes and create a master JSON file
            - sql: Process votes and add data to the SQLite database
                - file_path: Optional file path for adding new data to the database, this is only used with the SQL mode

📝 Examples:
    congress_rolls download_xml house 116 1 10
    congress_rolls process_votes json
    congress_rolls process_votes sql data/json/118/house/2/2024_1.json
    ";
    println!("{}", message);
    std::process::exit(1);
}

/// Asynchronously downloads XML files for the specified roll calls.
async fn xml_download() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 6 {
        usage();
    }

    let chamber = &args[2];
    let congress_number = args[3].parse::<u32>().expect("❌ Invalid congress number");
    let session = args[4].parse::<u32>().expect("❌ Invalid session number");
    let max_roll = args[5].parse::<u32>().expect("❌ Invalid max roll number");

    let client = Client::new(false);

    for roll in 1..=max_roll {
        let congress = Congress::new(chamber, congress_number, session, roll);

        // Create directories
        let base_dir = format!("data/xml/{}/{}", congress.number, chamber.to_lowercase());
        let session_dir = format!("{}/{}", base_dir, congress.session);

        // Check if the session directory exists
        // If it does, print nothing anc continue
        // If it doesn't, create the directories
        let path = std::path::Path::new(&session_dir);
        let exists = path.exists();

        // Create directories if they don't exist
        if let Err(e) = std::fs::create_dir_all(&session_dir) {
            eprintln!("❌ Failed to create directories {}: {}", session_dir, e);
            continue;
        }

        if !exists {
            println!("📁 Created directories: {}", session_dir);
        }

        let file_name = format!("{}/{}_{}.xml", session_dir, congress.to_year(), congress.roll_call);

        // Skip if file already exists
        if std::path::Path::new(&file_name).exists() {
            println!("⚠️ {} already exists", file_name);
            continue;
        }

        // Fetch data
        match client.fetch_data(&congress.to_string()).await {
            Ok(response) => {
                if response.contains("Webmaster") || response.contains("redirect") {
                    println!("🛑 Possible error: Redirect or webmaster blocking authentication for {}", file_name);
                    continue;
                }

                // Write to file
                match std::fs::write(&file_name, response) {
                    Ok(_) => println!("📄 {} written successfully.", file_name),
                    Err(e) => eprintln!("❌ Error writing {}: {}", file_name, e),
                }
            }
            Err(e) => {
                eprintln!("❌ Error fetching data for {}: {}", file_name, e);
            }
        }
    }
}

