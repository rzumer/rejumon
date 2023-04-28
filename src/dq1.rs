use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};
use prettytable::{row, Cell, Row, Table};

pub(crate) const JUMON_MOJI_TABLE: [char; 64] = [
    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た',
    'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み',
    'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'が', 'ぎ', 'ぐ', 'げ',
    'ご', 'ざ', 'じ', 'ず', 'ぜ', 'ぞ', 'だ', 'ぢ', 'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ',
];

pub(crate) const NAME_MOJI_TABLE: [char; 64] = [
    '０', '１', '２', '３', '４', '５', '６', '７', '８', '９', 'あ', 'い', 'う', 'え', 'お', 'か',
    'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
    'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ',
    'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん', 'っ', 'ゃ', 'ゅ', 'ょ', '゛', '゜', 'ー', '　',
];

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GameData {
    // Player name: 4 characters (from `NAME_MOJI_TABLE`, 6 bits each)
    pub name: [char; 4],
    // Experience: 16 bits
    pub experience: u16,
    // Gold: 16 bits
    pub gold: u16,
    // Weapon ID: 3 bits
    pub weapon: u8,
    // Armor ID: 3 bits
    pub armor: u8,
    // Shield ID: 2 bits
    pub shield: u8,
    // やくそう count: 4 bits
    pub herbs: u8,
    // まほうのカギ count: 4 bits
    pub keys: u8,
    // Item IDs: 8 slots, 4 bits each
    pub items: [u8; 8],
    // Progress flags, 5 total, 1 bit each
    pub progress_flags: [bool; 5],
    // Encryption key, 3 bits
    pub encryption_key: u8,
    // Checksum, 8 bits
    pub checksum: u8,
}

impl GameData {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
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

        data
    }
}

pub(crate) fn decode_jumon(input: &str) -> Result<Vec<u8>, String> {
    // Decode characters
    let mut decrypted_characters: [u8; 20] = [0; 20];
    let mut last_character_code = 0;
    for (index, character) in input.chars().enumerate() {
        let character_code = JUMON_MOJI_TABLE.iter().position(|&moji| moji == character);
        if character_code.is_none() {
            return Err(format!("Unsupported input character: {}", character));
        }

        let character_code_u8 = u8::try_from(character_code.unwrap()).unwrap();
        let decrypted =
            character_code_u8.wrapping_sub(last_character_code).wrapping_sub(4) & 0b00111111;
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
        Ok(input_bytes)
    } else {
        Err("Invalid CRC".to_string())
    }
}

pub(crate) fn tabulate_game_data(data: Vec<(String, GameData)>) -> String {
    // Create the table headers
    let mut table = Table::new();
    let header_row = row![
        "Player Name",
        "Experience",
        "Gold",
        "Weapon",
        "Armor",
        "Shield",
        "Herbs",
        "Keys",
        "Items",
        "Progress Flags",
        "Checksum"
    ];
    table.add_row(header_row.clone());

    // Iterate over each `(String, GameData)` tuple and add its information to the table
    for (label, game_data) in data {
        // Add the label as a single row spanning the entire table width, to keep it compact
        table.add_row(Row::new(vec![Cell::new(&label).with_hspan(header_row.len())]));

        // Add the `GameData` object to a new row as individual cells
        let mut cells = Vec::new();
        cells.push(Cell::new(&game_data.name.iter().collect::<String>()));
        cells.push(Cell::new(&game_data.experience.to_string()));
        cells.push(Cell::new(&game_data.gold.to_string()));
        cells.push(Cell::new(&game_data.weapon.to_string()));
        cells.push(Cell::new(&game_data.armor.to_string()));
        cells.push(Cell::new(&game_data.shield.to_string()));
        cells.push(Cell::new(&game_data.herbs.to_string()));
        cells.push(Cell::new(&game_data.keys.to_string()));
        cells.push(Cell::new(
            &game_data.items.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(""),
        ));
        cells.push(Cell::new(
            &game_data
                .progress_flags
                .iter()
                .map(|&x| if x { "1" } else { "0" })
                .collect::<Vec<_>>()
                .join(""),
        ));
        cells.push(Cell::new(&game_data.checksum.to_string()));

        table.add_row(Row::new(cells));
    }

    // Return the table as a `String`
    table.to_string()
}