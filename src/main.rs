mod dq1;
mod dq2;
use std::env;

use colored::Colorize;

fn split_dakuten(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            'が' => result.push_str("か゛"),
            'ぎ' => result.push_str("き゛"),
            'ぐ' => result.push_str("く゛"),
            'げ' => result.push_str("け゛"),
            'ご' => result.push_str("こ゛"),
            'ざ' => result.push_str("さ゛"),
            'じ' => result.push_str("し゛"),
            'ず' => result.push_str("す゛"),
            'ぜ' => result.push_str("せ゛"),
            'ぞ' => result.push_str("そ゛"),
            'だ' => result.push_str("た゛"),
            'ぢ' => result.push_str("ち゛"),
            'づ' => result.push_str("つ゛"),
            'で' => result.push_str("て゛"),
            'ど' => result.push_str("と゛"),
            'ば' => result.push_str("は゛"),
            'び' => result.push_str("ひ゛"),
            'ぶ' => result.push_str("ふ゛"),
            'べ' => result.push_str("へ゛"),
            'ぼ' => result.push_str("ほ゛"),
            'ぱ' => result.push_str("は゜"),
            'ぴ' => result.push_str("ひ゜"),
            'ぷ' => result.push_str("ふ゜"),
            'ぺ' => result.push_str("へ゜"),
            'ぽ' => result.push_str("ほ゜"),
            _ => result.push(c),
        }
    }
    result
}

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

    Err("Recovery failed.".to_string())
}

fn process_dq2(
    input: &str,
    name: Option<String>,
    progress_flags: Option<u8>,
    keep_checksum: bool,
) -> Result<String, String> {
    if let Ok(result) = dq2::decode_jumon(input) {
        if let Ok(data) = dq2::GameData::from_bytes(result.as_slice()) {
            return Ok(format!(
                "The password is already valid:\n\n{}",
                dq2::tabulate_game_data(vec![(input.to_string(), data)], input)
            ));
        }
    }

    let mut substitutions: Vec<(String, dq2::GameData)> = Vec::new();
    let max_index = if keep_checksum { input.len() - 1 } else { input.len() };
    for input_index_to_replace in 0..max_index {
        for moji in &dq2::JUMON_MOJI_TABLE {
            let mut new_string = String::with_capacity(input.len());
            for (input_index, input_character) in input.chars().enumerate() {
                if input_index == input_index_to_replace {
                    new_string.push(*moji);
                } else {
                    new_string.push(input_character);
                }
            }
            if let Ok(decoded) = dq2::decode_jumon(&new_string) {
                if let Ok(data) = dq2::GameData::from_bytes(decoded.as_slice()) {
                    // If the player name is known, ignore any substitutions where it is wrong.
                    if let Some(ref player_name) = name {
                        let player_name_chars = player_name.chars().collect::<Vec<char>>();
                        if player_name_chars != data.hero_name {
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
                    // Exclude codes that generate an invalid hero name.
                    if data.hero_name.iter().any(|&c| c == '\0') {
                        continue;
                    }
                    // Exclude codes that generate an invalid location.
                    if dq2::LOCATION_TABLE.get(data.location as usize).is_none() {
                        continue;
                    }
                    // Exclude codes that generate more than the maximum amount of experience.
                    if data.hero_experience > 1000000
                        || data.prince_experience > 1000000
                        || data.princess_experience > 1000000
                    {
                        continue;
                    }

                    substitutions.push((new_string.clone(), data));
                }
            }
        }
    }

    if !substitutions.is_empty() {
        return Ok(format!(
            "Found {} substitution(s):\n\n{}",
            substitutions.len(),
            dq2::tabulate_game_data(substitutions, input)
        ));
    }

    Err("Recovery failed.".to_string())
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
                name = Some(split_dakuten(&args[1]));
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
    let result: Result<String, String> =
        if input_string.chars().all(|c| dq1::JUMON_MOJI_TABLE.contains(&c)) && input_length == 20 {
            let dq1_res = process_dq1(input_string, name.clone(), progress_flags, keep_checksum);
            if dq1_res.is_err() {
                println!("{}", "DQ2".to_owned().bold().red());
                process_dq2(input_string, name, progress_flags, keep_checksum)
            } else {
                println!("{}", "DQ1".to_owned().bold().purple());
                dq1_res
            }
        } else if input_length > 0
            && input_length <= 52
            && input_string.chars().all(|c| dq2::JUMON_MOJI_TABLE.contains(&c))
        {
            println!("{}", "DQ2".to_owned().bold().red());
            process_dq2(input_string, name, progress_flags, keep_checksum)
        } else {
            Err("Invalid input.".to_string())
        };

    match result {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
