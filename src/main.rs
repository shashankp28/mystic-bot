use mystic_bot::base::defs::{ Board, GlobalMap, Search };
use std::collections::HashMap;
use std::time::{ Duration, Instant };
use serde_json::Value;
use std::io::{ self, Write };
use std::fs::{ self, File };
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use std::sync::Arc;

const COMPRESSED_OPENING_DB: &[u8] = include_bytes!("./maps/openings.tar.gz");

fn read_opening_db() -> Result<Value, io::Error> {
    let output_dir = "./db";
    let compressed_path = Path::new(output_dir).join("openings.tar.gz");
    let file_path = Path::new(output_dir).join("openingDB.json");

    if !file_path.exists() {
        println!("OpeningDB doesn't exist! Extracting from embedded archive...");

        fs::create_dir_all(output_dir)?;

        let mut file = File::create(&compressed_path)?;
        file.write_all(COMPRESSED_OPENING_DB)?;
        println!("Compressed file written to: {:?}", compressed_path);

        let file = File::open(&compressed_path)?;
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(output_dir)?;
        println!("Compressed file extracted to: {:?}", output_dir);

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
 .--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--. 
/ .. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \
\ \/\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ \/ /
 \/ /`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'\/ / 
 / /\                                                                                                                                                                                                                                    / /\ 
/ /\ \            _____                _____                    _____                _____                    _____                    _____                            _____                   _______               _____             / /\ \
\ \/ /           /\    \              |\    \                  /\    \              /\    \                  /\    \                  /\    \                          /\    \                 /::\    \             /\    \            \ \/ /
 \/ /           /::\____\             |:\____\                /::\    \            /::\    \                /::\    \                /::\    \                        /::\    \               /::::\    \           /::\    \            \/ / 
 / /\          /::::|   |             |::|   |               /::::\    \           \:::\    \               \:::\    \              /::::\    \                      /::::\    \             /::::::\    \          \:::\    \           / /\ 
/ /\ \        /:::::|   |             |::|   |              /::::::\    \           \:::\    \               \:::\    \            /::::::\    \                    /::::::\    \           /::::::::\    \          \:::\    \         / /\ \
\ \/ /       /::::::|   |             |::|   |             /:::/\:::\    \           \:::\    \               \:::\    \          /:::/\:::\    \                  /:::/\:::\    \         /:::/~~\:::\    \          \:::\    \        \ \/ /
 \/ /       /:::/|::|   |             |::|   |            /:::/__\:::\    \           \:::\    \               \:::\    \        /:::/  \:::\    \                /:::/__\:::\    \       /:::/    \:::\    \          \:::\    \        \/ / 
 / /\      /:::/ |::|   |             |::|   |            \:::\   \:::\    \          /::::\    \              /::::\    \      /:::/    \:::\    \              /::::\   \:::\    \     /:::/    / \:::\    \         /::::\    \       / /\ 
/ /\ \    /:::/  |::|___|______       |::|___|______    ___\:::\   \:::\    \        /::::::\    \    ____    /::::::\    \    /:::/    / \:::\    \            /::::::\   \:::\    \   /:::/____/   \:::\____\       /::::::\    \     / /\ \
\ \/ /   /:::/   |::::::::\    \      /::::::::\    \  /\   \:::\   \:::\    \      /:::/\:::\    \  /\   \  /:::/\:::\    \  /:::/    /   \:::\    \          /:::/\:::\   \:::\ ___\ |:::|    |     |:::|    |     /:::/\:::\    \    \ \/ /
 \/ /   /:::/    |:::::::::\____\    /::::::::::\____\/::\   \:::\   \:::\____\    /:::/  \:::\____\/::\   \/:::/  \:::\____\/:::/____/     \:::\____\        /:::/__\:::\   \:::|    ||:::|____|     |:::|    |    /:::/  \:::\____\    \/ / 
 / /\   \::/    / ~~~~~/:::/    /   /:::/~~~~/~~      \:::\   \:::\   \::/    /   /:::/    \::/    /\:::\  /:::/    \::/    /\:::\    \      \::/    /        \:::\   \:::\  /:::|____| \:::\    \   /:::/    /    /:::/    \::/    /    / /\ 
/ /\ \   \/____/      /:::/    /   /:::/    /          \:::\   \:::\   \/____/   /:::/    / \/____/  \:::\/:::/    / \/____/  \:::\    \      \/____/          \:::\   \:::\/:::/    /   \:::\    \ /:::/    /    /:::/    / \/____/    / /\ \
\ \/ /               /:::/    /   /:::/    /            \:::\   \:::\    \      /:::/    /            \::::::/    /            \:::\    \                       \:::\   \::::::/    /     \:::\    /:::/    /    /:::/    /             \ \/ /
 \/ /               /:::/    /   /:::/    /              \:::\   \:::\____\    /:::/    /              \::::/____/              \:::\    \                       \:::\   \::::/    /       \:::\__/:::/    /    /:::/    /               \/ / 
 / /\              /:::/    /    \::/    /                \:::\  /:::/    /    \::/    /                \:::\    \               \:::\    \                       \:::\  /:::/    /         \::::::::/    /     \::/    /                / /\ 
/ /\ \            /:::/    /      \/____/                  \:::\/:::/    /      \/____/                  \:::\    \               \:::\    \                       \:::\/:::/    /           \::::::/    /       \/____/                / /\ \
\ \/ /           /:::/    /                                 \::::::/    /                                 \:::\    \               \:::\    \                       \::::::/    /             \::::/    /                               \ \/ /
 \/ /           /:::/    /                                   \::::/    /                                   \:::\____\               \:::\____\                       \::::/    /               \::/____/                                 \/ / 
 / /\           \::/    /                                     \::/    /                                     \::/    /                \::/    /                        \::/____/                 ~~                                       / /\ 
/ /\ \           \/____/                                       \/____/                                       \/____/                  \/____/                          ~~                                                               / /\ \
\ \/ /                                                                                                                                                                                                                                  \ \/ /
 \/ /   ASCII Art: www.asciiart.eu                                                                                                                                                                                                       \/ / 
 / /\.--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--./ /\ 
/ /\ \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \/\ \
\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `' /
 `--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'                                                                       
                                                                                                            MYSTIC BOT                                
                                                                                                Ready to make your move? Let's play!
                                                                                        Input: "<fen position in quotes>" [Time limit in ms]
                                                                                                        Type `exit` to quit
"#;

    println!("{}", logo);

    // Initialize the opening database
    let opening_db = match read_opening_db() {
        Ok(data) => Arc::new(data),
        Err(e) => {
            eprintln!("Error initializing opening database: {}", e);
            return;
        }
    };
    GlobalMap::init();
    loop {
        println!("Mystic Bot Ready!\n\n");
        print!("MysticBotCli> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Custom argument parsing to handle quoted FEN strings
        let mut args: Vec<String> = vec![];
        let mut temp = String::new();
        let mut in_quotes = false;

        for token in input.split_whitespace() {
            if token.starts_with('"') {
                in_quotes = true;
                temp.clear();
                temp.push_str(&token[1..]); // Exclude opening quote
                temp.push(' ');
            } else if token.ends_with('"') {
                temp.push_str(&token[..token.len() - 1]); // Exclude closing quote
                args.push(temp.trim().to_string()); // Clone the trimmed value
                temp.clear();
                in_quotes = false;
            } else if in_quotes {
                temp.push_str(token);
                temp.push(' ');
            } else {
                args.push(token.to_string());
            }
        }

        if in_quotes {
            println!("Error: Unmatched quotes in input.");
            continue;
        }

        if args.is_empty() {
            println!("Invalid input. Usage: <FEN Position in Quotes> [Time Limit in ms]");
            continue;
        }

        let fen_position = &args[0];
        let time_limit = if args.len() > 1 {
            args[1].parse::<u64>().ok().map(Duration::from_millis)
        } else {
            Some(Duration::from_secs(5)) // Default to 5 seconds
        };

        if let Some(time_limit) = time_limit {
            match Board::from_fen(fen_position) {
                Some(board) => {
                    let memory: HashMap<u64, f64> = HashMap::new();
                    let mut search = Search {
                        board,
                        memory,
                        opening_db: Arc::clone(&opening_db),
                        num_nodes: 0,
                        max_depth: 3,
                        num_prunes: 0,
                    };

                    // Search in opening DB first
                    if let Some(next) = search.search_opening_db() {
                        println!("Next move found in opening database");
                        println!("\nBest next move: {}", next.get_next_uci());
                    } else {
                        let start_time = Instant::now();
                        let next_board = search.best_next_board(time_limit, &start_time);

                        if let Some(next) = next_board {
                            println!("\nBest next move: {}", next.get_next_uci());
                        } else {
                            println!("No valid moves found.");
                        }
                    }
                }
                None => {
                    println!("Error parsing FEN position: {}", fen_position);
                }
            }
        } else {
            println!("Invalid time limit.");
        }
    }
}
