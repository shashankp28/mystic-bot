use mystic_bot::base::defs::{ Board, Search };
use std::collections::HashMap;
use std::time::{ Duration, Instant };
use serde_json::Value;
use std::io::{ self, Write };
use std::fs::{ self, File };
use std::path::Path;
use reqwest::blocking::get;
use flate2::read::GzDecoder;
use tar::Archive;

fn read_opening_db() -> Result<Value, io::Error> {
    let output_dir = "./db";
    let compressed_name = "openings.tar.gz";
    let compressed_path = Path::new(output_dir).join(compressed_name);
    let file_name = "openingDB.json";
    let file_path = Path::new(output_dir).join(file_name);

    if !file_path.exists() {
        println!("OpeningDB doesn't exist! Downloading...");
        let file_id = "1TCGGGKb9dtn_GhQcOp94V4f9AvFt_E-c";
        let url = format!("https://drive.google.com/uc?export=download&id={}", file_id);

        let response = get(&url).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let content = response.bytes().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        fs::create_dir_all(output_dir)?;

        let mut file = File::create(&compressed_path)?;
        file.write_all(&content)?;
        println!("Compressed file downloaded to: {:?}", compressed_path);

        let file = File::open(&compressed_path)?;
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(output_dir)?;
        println!("Compressed file extracted to: {:?}", compressed_path);

        fs::remove_file(&compressed_path)?;
    }

    let file_content = fs::read_to_string(&file_path)?;
    let json_data: Value = serde_json
        ::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(json_data)
}

fn main() {
    let logo =
        r#"
 ▓██████████████▓▒  ▒▓█▓▒  ▒▓█▓▒   ▓███████▓ ▒▓████████▓ ▒▓█▓▒  ▒▓██████▓▒        ▒▓███████▓▒   ▒▓██████▓  ▒▓████████▓▒
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓  ▒▓█▓▒  ▒▓█▓▒ ▓█▓▒           ▒▓█▓▒    ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒       ▒▓█▓▒  ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒   ▒▓█▓▒     
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓  ▒▓█▓▒  ▒▓█▓▒ ▓█▓▒           ▒▓█▓▒    ▒▓█▓▒ ▒▓█▓▒              ▒▓█▓▒  ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒   ▒▓█▓▒     
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓   ▒▓██████▓▒   ▓██████▓▒     ▒▓█▓▒    ▒▓█▓▒ ▒▓█▓▒              ▒▓███████▓   ▒▓█▓▒  ▒▓█▓▒   ▒▓█▓▒     
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓     ▒▓█▓▒            ▒▓█▓▒   ▒▓█▓▒    ▒▓█▓▒ ▒▓█▓▒              ▒▓█▓▒  ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒   ▒▓█▓▒     
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓     ▒▓█▓▒            ▒▓█▓▒   ▒▓█▓▒    ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒       ▒▓█▓▒  ▒▓█▓▒ ▒▓█▓▒  ▒▓█▓▒   ▒▓█▓▒     
 ▓█▓▒  ▒▓█▓▒  ▒▓█▓     ▒▓█▓▒    ░▒▓███████▓▒    ▒▓█▓▒    ▒▓█▓▒  ▒▓██████▓▒        ▒▓███████▓▒   ▒▓██████▓▒    ▒▓█▓▒     
                                                                                                                                                                                                     
                                    ♚ ♞ MYSTIC BOT ♟ ♖                                
                            Ready to make your move? Let's play!
                    Input: <JSON file to read/update> [Time limit in ms]
                                    Type `exit` to quit
"#;

    println!("{}", logo);

    // Initialize the opening database
    let opening_db = match read_opening_db() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error initializing opening database: {}", e);
            return;
        }
    };

    println!( "Mystic Bot Ready!\n" );
    loop {
        print!("MysticBotCli> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() < 1 {
            println!("Invalid input. Usage: <JSON File to Update> [Time Limit in ms]");
            continue;
        }

        let file_path = args[0];
        let time_limit = (
            if args.len() > 1 {
                args[1].parse::<u64>().ok().map(Duration::from_millis)
            } else {
                Some(Duration::from_secs(5)) // Default to 5 seconds
            }
        ).unwrap();

        match Board::from_file(file_path) {
            Ok(board) => {
                // Check if board.hash() exists in the opening_db
                let board_hash = board.hash(); // Assuming `board` has a `hash` method
                if let Some(entry) = opening_db.get(board_hash.to_string()) {
                    println!("Found in opening database: {:?}", entry);
                } else {
                    println!("Hash not found: {:?}", board_hash.to_string())
                }

                let memory: HashMap<u64, f64> = HashMap::new();
                let mut search: Search = Search {
                    board,
                    memory,
                    // opening_db,
                    num_nodes: 0,
                    max_depth: 5,
                    num_prunes: 0,
                };

                let start_time = Instant::now();
                let next_board = search.best_next_board(time_limit, &start_time);

                if let Some(next) = next_board {
                    next.save_board(file_path);
                    println!( "New Board Saved Successfully!\n" );
                }
            }
            Err(e) => {
                println!("Error loading board: {}", e);
            }
        }
    }
}
