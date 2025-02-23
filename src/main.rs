use std::time::Duration;
use std::io::{ self, Write };
use std::collections::HashMap;
use mystic_bot::base::defs::{ Board, GlobalMap, Search };

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
                                                                                    Input: "<fen position in quotes>" [Time limit in ms] [fen history path]
                                                                                                        Type `exit` to quit
"#;

    println!("{}", logo);

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
            return;
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
            return;
        }

        if args.is_empty() {
            println!(
                "Invalid input. Usage: \"<FEN Position in Quotes>\" [Time Limit in ms] [FEN History Path]"
            );
            return;
        }

        let fen_position = &args[0];
        let time_limit = if args.len() > 1 {
            args[1].parse::<u64>().ok().map(Duration::from_millis)
        } else {
            Some(Duration::from_secs(5)) // Default to 5 seconds
        };

        let fen_history_path = if args.len() > 2 { args[2].clone() } else { String::from("") };

        if let Some(time_limit) = time_limit {
            match Board::from_fen(fen_position) {
                Some(board) => {
                    let memory: HashMap<u128, u32> = HashMap::new();
                    let mut search = Search {
                        board,
                        memory,
                        num_nodes: 0,
                        max_depth: 3,
                        num_prunes: 0,
                    };
                    search.process_fen_history(&fen_history_path);
                    println!("History Path: {:?}", fen_history_path);
                    println!("Position History: {:?}", search.memory);

                    // Search in opening DB first
                    if let Some(next) = search.search_opening_db() {
                        println!("Next move found in opening database");
                        println!("Best next move: {}", next.get_next_uci());
                    } else {
                        let next_board = search.best_next_board(time_limit);

                        if let Some(next) = next_board {
                            println!("Best next move: {}", next.get_next_uci());
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
