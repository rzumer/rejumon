mod dq1;
use std::env;

fn process_dq1(
    input: &str,
    name: Option<String>,
    progress_flags: Option<u8>,
    keep_checksum: bool,
) -> Result<String, String> {
    if let Ok(result) = dq1::decode_jumon(input) {
        let data = dq1::GameData::from_bytes(result.as_slice());
        return Ok(format!(
            "The password is already valid:\n\n{}",
            dq1::tabulate_game_data(vec![(input.to_string(), data)], input)
        ));
    }

    let mut substitutions: Vec<(String, dq1::GameData)> = Vec::new();
    let max_index = if keep_checksum { input.len() - 1 } else { input.len() };
    for input_index_to_replace in 0..max_index {
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
                // If progress flags are specified, ignore any substitutions where they change.
                if let Some(flags) = progress_flags {
                    let decoded_flags =
                        data.progress_flags.iter().fold(0, |acc, &b| (acc << 1) | b as u8);
                    if flags != decoded_flags {
                        continue;
                    }
                }
                // Exclude codes that generate an invalid item.
                if data.items.contains(&(dq1::ITEM_TABLE.len() as u8 - 1)) {
                    continue;
                }
                // Exclude codes that generate more than the maximum amount of herbs or keys.
                if data.herbs > 6 || data.keys > 6 {
                    continue;
                }

                substitutions.push((new_string.clone(), data));
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
        eprintln!("usage: {} [--name <name>] [--flags <flags>] [--keep-checksum] <input>", program);
        return;
    }

    // Parse optional options to constrain substitutions.
    let mut name: Option<String> = None;
    let mut progress_flags: Option<u8> = None;
    let mut keep_checksum: bool = false;
    while args.len() >= 2 {
        let arg: &str = &args[0];
        match arg {
            "--name" | "-n" => {
                name = Some(args[1].clone());
                args = args[2..].to_vec();
            }
            "--flags" | "-f" => {
                progress_flags = args[1].clone().parse::<u8>().ok();
                args = args[2..].to_vec();
            }
            "--keep-checksum" | "-k" => {
                keep_checksum = true;
                args = args[1..].to_vec();
            }
            _ => {
                break;
            }
        }
    }

    // Join all arguments to account for any spacing within the password.
    let input_string = &args.join("").split_whitespace().collect::<String>();
    let input_length = input_string.chars().count();
    match input_length {
        20 => {
            // DQ1
            let result = process_dq1(input_string, name, progress_flags, keep_checksum);
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
