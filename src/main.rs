use std::env;
use bitstream_io::{BigEndian, BitReader, BitRead, BitWriter, BitWrite};

const JUMON_MOJI_TABLE: [char; 64] = [
    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く',
    'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た',
    'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね',
    'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み',
    'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り',
    'る', 'れ', 'ろ', 'わ', 'が', 'ぎ', 'ぐ', 'げ',
    'ご', 'ざ', 'じ', 'ず', 'ぜ', 'ぞ', 'だ', 'ぢ',
    'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ',
];

const NAME_MOJI_TABLE: [char; 64] = [
    '０', '１', '２', '３', '４', '５', '６', '７',
    '８', '９', 'あ', 'い', 'う', 'え', 'お', 'か',
    'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ',
    'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
    'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ',
    'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん',
    'っ', 'ゃ', 'ゅ', 'ょ', '゛', '゜', 'ー', '　',
];

#[derive(Clone, Copy, Debug, Default)]
struct GameData {
    // Player name: 4 characters (from `NAME_MOJI_TABLE`, 6 bits each)
    name: [char; 4],
    // Experience: 16 bits
    experience: u16,
    // Gold: 16 bits
    gold: u16,
    // Weapon ID: 3 bits
    weapon: u8,
    // Armor ID: 3 bits
    armor: u8,
    // Shield ID: 2 bits
    shield: u8,
    // やくそう count: 4 bits
    herbs: u8,
    // まほうのカギ count: 4 bits
    keys: u8,
    // Item IDs: 8 slots, 4 bits each
    items: [u8; 8],
    // Progress flags, 5 total, 1 bit each
    progress_flags: [bool; 5],
    // Encryption key, 3 bits
    encryption_key: u8,
    // Checksum, 8 bits
    checksum: u8,
}

impl GameData {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut data = Self::default();

        let mut reader = BitReader::endian(bytes, BigEndian);

        data.items[1] = reader.read::<u8>(4).unwrap();
        data.items[0] = reader.read::<u8>(4).unwrap();

        data.progress_flags[0] = reader.read::<u8>(1).unwrap() != 0;
        data.name[1] = NAME_MOJI_TABLE[reader.read::<u8>(6).unwrap() as usize];
        data.progress_flags[1] = reader.read::<u8>(1).unwrap() != 0;

        data.experience |= reader.read::<u16>(8).unwrap() << 8;

        data.items[5] = reader.read::<u8>(4).unwrap();
        data.items[4] = reader.read::<u8>(4).unwrap();

        data.herbs = reader.read::<u8>(4).unwrap();
        data.keys = reader.read::<u8>(4).unwrap();

        data.gold |= reader.read::<u16>(8).unwrap() << 8;

        data.weapon = reader.read::<u8>(3).unwrap();
        data.armor = reader.read::<u8>(3).unwrap();
        data.shield = reader.read::<u8>(2).unwrap();

        data.encryption_key |= reader.read::<u8>(1).unwrap() << 2;
        data.progress_flags[2] = reader.read::<u8>(1).unwrap() != 0;
        data.name[3] = NAME_MOJI_TABLE[reader.read::<u8>(6).unwrap() as usize];

        data.items[7] = reader.read::<u8>(4).unwrap();
        data.items[6] = reader.read::<u8>(4).unwrap();

        data.name[0] = NAME_MOJI_TABLE[reader.read::<u8>(6).unwrap() as usize];
        data.progress_flags[3] = reader.read::<u8>(1).unwrap() != 0;
        data.encryption_key |= reader.read::<u8>(1).unwrap() << 1;

        data.gold |= reader.read::<u16>(8).unwrap();

        data.items[3] = reader.read::<u8>(4).unwrap();
        data.items[2] = reader.read::<u8>(4).unwrap();

        data.encryption_key |= reader.read::<u8>(1).unwrap();
        data.progress_flags[4] = reader.read::<u8>(1).unwrap() != 0;
        data.name[2] = NAME_MOJI_TABLE[reader.read::<u8>(6).unwrap() as usize];

        data.experience |= reader.read::<u16>(8).unwrap();

        data.checksum = reader.read::<u8>(8).unwrap();

        return data;
    }
}

fn decode(input: &str) -> Result<Vec<u8>, String> {
    // Decode characters
    let mut decrypted_characters: [u8; 20] = [ 0; 20 ];
    let mut last_character_code = 0;
    for (index, character) in input.chars().enumerate() {
        let character_code = JUMON_MOJI_TABLE.iter().position(|&moji| moji == character);
        if character_code.is_none() {
            return Err(format!("Unsupported input character: {}", character));
        }

        let character_code_u8 = u8::try_from(character_code.unwrap()).unwrap();
        let decrypted = character_code_u8.wrapping_sub(last_character_code).wrapping_sub(4) & 0b00111111;
        last_character_code = character_code_u8;

        decrypted_characters[index] = decrypted;
    }

    // Pack characters into bytes
    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    for character in decrypted_characters.iter().rev() {
        writer.write(6, *character).unwrap();
    }
    let mut input_bytes = writer.into_writer();

    // Calculate the correct checksum (XMODEM-CRC)
    let mut crc = 0_u8;
    let mut divisor = 0x8000_u16;
    for byte in input_bytes[..input_bytes.len() - 1].iter() {
        for bit in 0..8 {
            if divisor & 0x8000 != 0 {
                divisor = (divisor << 1) ^ 0x1021;
            } else {
                divisor <<= 1;
            }

            if *byte & (1 << bit) != 0 {
                crc ^= divisor as u8;
            }
        }
    }

    // Confirm that the CRC is correct
    let last_byte = input_bytes.last_mut().unwrap();
    if crc == *last_byte {
        return Ok(input_bytes);
    } else {
        return Err("Invalid CRC".to_string());
    }
}

fn process(input: &str, name: Option<String>) -> Result<String, String> {
    if input.chars().count() != 20 {
        return Err("Input must be 20 characters.".to_string());
    }

    let result = decode(input);
    if result.is_ok() {
        return Ok("The password is already valid.".to_string());
    }

    let mut substitutions: Vec<(String, GameData)> = Vec::new();
    for input_index_to_replace in 0..input.len() {
        for moji_index in 0..JUMON_MOJI_TABLE.len() {
            let mut new_string = String::with_capacity(input.len());
            for (input_index, input_character) in input.chars().enumerate() {
                if input_index == input_index_to_replace {
                    new_string.push(JUMON_MOJI_TABLE[moji_index]);
                } else {
                    new_string.push(input_character);
                }
            }
            if let Ok(decoded) = decode(&new_string) {
                let data = GameData::from_bytes(decoded.as_slice());
                // If the player name is known, ignore any substitutions where it is wrong.
                if let Some(ref player_name) = name {
                    let player_name_chars = player_name.chars().collect::<Vec<char>>();
                    if player_name_chars != &data.name[..] {
                        continue;
                    }
                }
                substitutions.push((String::from(new_string), data));
            }
        }
    }

    if substitutions.len() > 0 {
        let mut output = String::new();
        output += &format!("Found {} substitution(s):\n", substitutions.len());
        for (password, data) in substitutions {
            output += &format!("{}\n{:?}\n\n", password, data);
        }
        return Ok(output.trim_end().to_string());
    }

    return Err("Recovery failed".to_string());
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    args = args[1..].to_vec();

    if args.len() == 0 {
        eprintln!("usage: {} <input>", program);
        return;
    }

    // Parse an optional name option to constrain substitutions.
    let mut name: Option<String> = None;
    if args.len() > 2 {
        if args[0] == "--name" || args[1] == "-n" {
            name = Some(args[1].clone());
            args = args[2..].to_vec();
        }
    }

    // Join all arguments to account for any spacing within the password.
    let input_string = &args.join("").split_whitespace().collect::<String>();
    let result = process(input_string, name);
    match result {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
