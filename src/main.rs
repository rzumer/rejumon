mod dq1;
use std::env;

fn process_dq1(input: &str, name: Option<String>) -> Result<String, String> {
    if let Ok(result) = dq1::decode_jumon(input) {
        let data = dq1::GameData::from_bytes(result.as_slice());
        return Ok(format!(
            "The password is already valid:\n\n{}",
            dq1::tabulate_game_data(vec![(input.to_string(), data)], input)
        ));
    }

    let mut substitutions: Vec<(String, dq1::GameData)> = Vec::new();
    for input_index_to_replace in 0..input.len() {
        for moji in &dq1::JUMON_MOJI_TABLE {
            let mut new_string = String::with_capacity(input.len());
            for (input_index, input_character) in input.chars().enumerate() {
                if input_index == input_index_to_replace {
                    new_string.push(*moji);
                } else {
                    new_string.push(input_character);
                }
            }
            if let Ok(decoded) = dq1::decode_jumon(&new_string) {
                let data = dq1::GameData::from_bytes(decoded.as_slice());
                // If the player name is known, ignore any substitutions where it is wrong.
                if let Some(ref player_name) = name {
                    let player_name_chars = player_name.chars().collect::<Vec<char>>();
                    if player_name_chars != data.name {
                        continue;
                    }
                }
                substitutions.push((new_formatted_string, data));
            }
        }
    }

    if !substitutions.is_empty() {
        return Ok(format!(
            "Found {} substitution(s):\n\n{}",
            substitutions.len(),
            dq1::tabulate_game_data(substitutions, input)
        ));
    }

    Err("Recovery failed".to_string())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    args = args[1..].to_vec();

    if args.is_empty() {
        eprintln!("usage: {} [--name <name>] <input>", program);
        return;
    }

    // Parse an optional name option to constrain substitutions.
    let mut name: Option<String> = None;
    if args.len() > 2 && (args[0] == "--name" || args[1] == "-n") {
        name = Some(args[1].clone());
        args = args[2..].to_vec();
    }

    // Join all arguments to account for any spacing within the password.
    let input_string = &args.join("").split_whitespace().collect::<String>();
    let input_length = input_string.chars().count();
    match input_length {
        20 => {
            // DQ1
            let result = process_dq1(input_string, name);
            match result {
                Ok(output) => println!("{}", output),
                Err(err) => eprintln!("{}", err),
            }
        }
        52 => {
            // DQ2
            unimplemented!();
        }
        _ => {
            eprintln!("Invalid input length.");
        }
    }
}
