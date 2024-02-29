use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};
use colored::Colorize;
use prettytable::{row, Cell, Row, Table};
use std::io::ErrorKind::InvalidData;

pub(crate) const JUMON_MOJI_TABLE: [char; 64] = [
    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た',
    'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み',
    'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'が', 'ぎ', 'ぐ', 'げ',
    'ご', 'ざ', 'じ', 'ず', 'ぜ', 'ぞ', 'ば', 'び', 'ぶ', 'べ', 'ぼ', 'ぱ', 'ぴ', 'ぷ', 'ぺ', 'ぽ',
];

pub(crate) const NAME_MOJI_TABLE: [char; 64] = [
    '０', '１', '２', '３', '４', '５', '６', '７', '８', '９', 'あ', 'い', 'う', 'え', 'お', 'か',
    'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
    'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ',
    'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん', 'っ', 'ゃ', 'ゅ', 'ょ', '゛', '゜', '　', '\0',
];

pub(crate) const ITEM_TABLE: [&str; 64] = [
    "（なし）",
    "ひのきのぼう",
    "せいなるナイフ",
    "まどうしのつえ",
    "いかずちのつえ",
    "こんぼう",
    "どうのつるぎ",
    "くさりがま",
    "てつのやり",
    "はやぶさのけん",
    "はがねのつるぎ",
    "おおかなずち",
    "はかいのつるぎ",
    "ドラゴンキラー",
    "ひかりのつるぎ",
    "ロトのつるぎ",
    "いなずまのけん",
    "ぬののふく",
    "みかわしのふく",
    "みずのはごろも",
    "ミンクのコート",
    "かわのよろい",
    "くさりかたびら",
    "あくまのよろい",
    "まほうのよろい",
    "はがねのよろい",
    "ガイアのよろい",
    "ロトのよろい",
    "かわのたて",
    "ちからのたて",
    "はがねのたて",
    "しにがみのたて",
    "ロトのたて",
    "ふしぎなぼうし",
    "てつかぶと",
    "ロトのかぶと",
    "ロトのしるし",
    "ふねのざいほう",
    "つきのかけら",
    "ルビスのまもり",
    "じゃしんのぞう",
    "せかいじゅのは",
    "やまびこのふえ",
    "ラーのかがみ",
    "あまつゆのいと",
    "せいなりおりき",
    "かぜのマント",
    "あくまのしっぽ",
    "まよけのすず",
    "ふっかつのたま",
    "ゴールドカード",
    "ふくびきけん",
    "せいすい",
    "キメラのつばさ",
    "みみせん",
    "きんのかぎ",
    "ぎんのかぎ",
    "ろうやのかぎ",
    "すいもんのかぎ",
    "どくけしそう",
    "やくそう",
    "いのりのゆびわ",
    "しのオルゴール",
    "あぶないみずぎ",
];

pub(crate) const LOCATION_TABLE: [&str; 7] = [
    "ローレシア",
    "サマルトリア",
    "ラダトーム",
    "デルコンダル",
    "ベラヌール",
    "ロンダルキア",
    "ムーンペタ",
];

pub(crate) const CREST_TABLE: [&str; 5] = ["命", "水", "月", "星", "太陽"];

pub(crate) const PROGRESS_FLAG_TABLE: [&str; 7] = [
    "つきのかけら使用済み",
    "すいもんのカギ使用済み",
    "みずのはごろも回収可能",
    "ルプガナの魔物倒し済み",
    "ルプガナのふね取得済み",
    "サマルトリアの王話し済み",
    "ゆうしゃのいずみ到着済み",
];

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GameData {
    // Player name: 4 characters (from `NAME_MOJI_TABLE`, 6 bits each)
    pub hero_name: [char; 4],
    // Item IDs: 8 slots, 4 bits each
    pub hero_items: [u8; 8],
    pub prince_items: [u8; 8],
    pub princess_items: [u8; 8],
    // Experience: 20 bits
    pub hero_experience: u32,
    pub prince_experience: u32,
    pub princess_experience: u32,
    // Character recruitment flags
    pub prince_flag: bool,
    pub princess_flag: bool,
    // Gold: 16 bits
    pub gold: u16,
    // Location, 3 bits
    pub location: u8,
    // Progress flags, 7 total, 1 bit each
    pub progress_flags: [bool; 7],
    // Crest flags, 5 total, 1 bit each
    pub crests: [bool; 5],
    // Encryption key, 4 bits
    pub encryption_key: u8,
    // Checksum, 11 bits
    pub checksum: u16,
}

impl GameData {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = Self::default();

        let mut reader = BitReader::endian(bytes, BigEndian);

        data.checksum |= reader.read::<u16>(5)?;
        data.location = reader.read::<u8>(3)?;

        let mut name_char = 0u8; // heroName[1]
        data.hero_name[2] = NAME_MOJI_TABLE[reader.read::<u8>(6)? as usize];
        name_char |= reader.read::<u8>(2)? << 4;

        data.gold |= reader.read::<u16>(8)? << 8;

        name_char |= reader.read::<u8>(2)? << 1;
        data.hero_name[0] = NAME_MOJI_TABLE[reader.read::<u8>(6)? as usize];

        data.gold |= reader.read::<u16>(8)?;

        name_char |= reader.read::<u8>(1)?;
        data.hero_name[3] = NAME_MOJI_TABLE[reader.read::<u8>(6)? as usize];
        name_char |= reader.read::<u8>(1)? << 3;
        data.hero_name[1] = NAME_MOJI_TABLE[name_char as usize];

        data.encryption_key |= reader.read::<u8>(1)?;
        data.progress_flags[0] = reader.read::<u8>(1)? != 0;
        data.progress_flags[1] = reader.read::<u8>(1)? != 0;
        data.progress_flags[2] = reader.read::<u8>(1)? != 0;
        data.progress_flags[3] = reader.read::<u8>(1)? != 0;
        data.progress_flags[4] = reader.read::<u8>(1)? != 0;
        data.progress_flags[5] = reader.read::<u8>(1)? != 0;
        data.progress_flags[6] = reader.read::<u8>(1)? != 0;

        data.encryption_key |= reader.read::<u8>(3)? << 1;
        data.crests[0] = reader.read::<u8>(1)? != 0;
        data.crests[1] = reader.read::<u8>(1)? != 0;
        data.crests[2] = reader.read::<u8>(1)? != 0;
        data.crests[3] = reader.read::<u8>(1)? != 0;
        data.crests[4] = reader.read::<u8>(1)? != 0;

        // These two bits carry the least 2 significant bits of the final
        // unaligned princess item ID if all characters are carrying 8 items.
        let final_bits = reader.read::<u8>(2)?;
        data.checksum |= reader.read::<u16>(6)? << 5;

        data.hero_experience |= reader.read::<u32>(16)?;
        data.hero_experience |= reader.read::<u32>(4)? << 16;

        let item_count = reader.read::<u8>(4)?;
        for i in 0..item_count {
            *data.hero_items.get_mut(i as usize).ok_or(InvalidData)? = reader.read::<u8>(7)?;
        }

        data.prince_flag = reader.read::<u8>(1)? != 0;
        if data.prince_flag {
            data.prince_experience |= reader.read::<u32>(16)?;
            data.prince_experience |= reader.read::<u32>(4)? << 16;

            let item_count = reader.read::<u8>(4)?;
            for i in 0..item_count {
                *data.prince_items.get_mut(i as usize).ok_or(InvalidData)? =
                    reader.read::<u8>(7)?;
            }

            data.princess_flag = reader.read::<u8>(1)? != 0;
            if data.princess_flag {
                data.princess_experience |= reader.read::<u32>(16)?;
                data.princess_experience |= reader.read::<u32>(4)? << 16;

                let item_count = reader.read::<u8>(4)?;
                for i in 0..item_count {
                    // If all characters are carrying 8 items, the save data is
                    // compressed by setting the 2 least significant bits of the
                    // last item ID to the most significant bits of one of the
                    // checksum bytes. To avoid overreading, first read the
                    // 5 most significant bits of the item ID, then try to
                    // read the 2 least significant bits, and substitute them
                    // with the bits moved earlier in the stream if it fails.
                    let high_bits = reader.read::<u8>(5)?;
                    *data.princess_items.get_mut(i as usize).ok_or(InvalidData)? = high_bits << 2
                        | match reader.read::<u8>(2) {
                            Ok(val) => val,
                            Err(_) => final_bits,
                        };
                }
            }
        }

        Ok(data)
    }
}

pub(crate) fn decode_jumon(input: &str) -> Result<Vec<u8>, String> {
    // Convert input unicode characters to their corresponding 6-bit character codes
    let jumon_chars: Vec<u8> = input
        .chars()
        .map(|c| {
            JUMON_MOJI_TABLE
                .iter()
                .position(|&moji| moji == c)
                .map(|index| index as u8)
                .ok_or_else(|| format!("Unsupported input character: {}", c))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Decrypt characters.
    let mut decrypted = vec![jumon_chars[0]];
    let key = ((jumon_chars[0] & 0b0110) >> 1) + 1;
    for (&prev, &cur) in jumon_chars.iter().zip(jumon_chars.iter().skip(1)) {
        decrypted.push(cur.wrapping_sub(prev).wrapping_sub(key) & 0b0011_1111);
    }

    // Pack characters into bytes
    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    for &character in decrypted.iter() {
        writer.write(6, character).unwrap();
    }
    let mut input_bytes = writer.into_writer();

    // Assemble the expected checksum from the first and ninth bytes
    let checksum_bytes = (input_bytes[0] & 0b1111_1000, input_bytes[8] & 0b0011_1111);
    let expected = (checksum_bytes.0 as u16) >> 3 | (checksum_bytes.1 as u16) << 5;
    // Clear the checksum bits before validating the checksum
    input_bytes[0] ^= checksum_bytes.0;
    input_bytes[8] ^= checksum_bytes.1;

    // Calculate the correct checksum (11-bit CRC, unknown if the format is standard)
    let mut crc = input_bytes.len() as u16 * 0x0101;
    for &byte in input_bytes.iter().rev() {
        let mut divisor = byte;
        for _bit in 0..8 {
            let carry_bit = ((crc >> 8) as u8 ^ divisor) & 0b1000_0000 != 0;
            crc <<= 1;
            divisor <<= 1;
            if carry_bit {
                crc ^= 0x1021;
            }
        }
    }
    // Truncate the computed CRC to 11 bits
    crc &= 0b0000_0111_1111_1111;

    // Set the checksum bytes back to their original values
    input_bytes[0] ^= checksum_bytes.0;
    input_bytes[8] ^= checksum_bytes.1;

    // Confirm that the CRC is correct
    if (crc & 0b0000_0111_1111_1111) == expected {
        Ok(input_bytes)
    } else {
        Err("Invalid CRC".to_string())
    }
}

pub(crate) fn tabulate_game_data(data: Vec<(String, GameData)>, input: &str) -> String {
    // Create the table headers
    let mut table = Table::new();
    let header_row = row![
        "Player Name",
        "Location",
        "Experience",
        "Gold",
        "Items (Hero)",
        "Items (Prince)",
        "Items (Princess)",
        "Crests",
        "Progress Flags",
        "Checksum"
    ];
    table.add_row(header_row.clone());

    // Iterate over each `(String, GameData)` tuple and add its information to the table
    for (label, game_data) in data {
        // Set the color of any substituted characters in the label to red
        let mut formatted_label = String::with_capacity(input.len());
        for (index, input_character) in input.chars().enumerate() {
            let label_character = label.char_indices().nth(index).unwrap().1;
            if input_character == label_character {
                formatted_label.push(input_character);
            } else {
                formatted_label =
                    format!("{}{}", formatted_label, label_character.to_string().red());
            }
        }

        // Add the label as a single row spanning the entire table width, to keep it compact
        table.add_row(Row::new(vec![Cell::new(&formatted_label).with_hspan(header_row.len())]));

        // Add the `GameData` object to a new row as individual cells
        let mut cells = Vec::new();
        cells.push(Cell::new(&game_data.hero_name.iter().collect::<String>()));
        cells.push(Cell::new(LOCATION_TABLE[game_data.location as usize]));
        cells.push(Cell::new(&format!(
            "Hero: {}\nPrince: {}\nPrincess: {}",
            game_data.hero_experience,
            if game_data.prince_flag {
                game_data.prince_experience.to_string()
            } else {
                "N/A".to_string()
            },
            if game_data.princess_flag {
                game_data.princess_experience.to_string()
            } else {
                "N/A".to_string()
            },
        )));
        cells.push(Cell::new(&game_data.gold.to_string()));
        cells.push(Cell::new(
            &game_data
                .hero_items
                .iter()
                .map(|&x| {
                    let equipped = x & 0b100_0000 != 0;
                    if equipped { "E " } else { "  " }.to_owned()
                        + ITEM_TABLE[(x & 0b011_1111) as usize]
                })
                .collect::<Vec<_>>()
                .join("\n"),
        ));
        if game_data.prince_flag {
            cells.push(Cell::new(
                &game_data
                    .prince_items
                    .iter()
                    .map(|&x| {
                        let equipped = x & 0b100_0000 != 0;
                        if equipped { "E " } else { "  " }.to_owned()
                            + ITEM_TABLE[(x & 0b011_1111) as usize]
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ));
        } else {
            cells.push(Cell::new("N/A"));
        }
        if game_data.princess_flag {
            cells.push(Cell::new(
                &game_data
                    .princess_items
                    .iter()
                    .map(|&x| {
                        let equipped = x & 0b100_0000 != 0;
                        if equipped { "E " } else { "  " }.to_owned()
                            + ITEM_TABLE[(x & 0b011_1111) as usize]
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ));
        } else {
            cells.push(Cell::new("N/A"));
        }
        cells.push(Cell::new(
            &game_data
                .crests
                .iter()
                .enumerate()
                .rev()
                .map(|(idx, &val)| if val { "○ " } else { "☓ " }.to_owned() + CREST_TABLE[idx])
                .collect::<Vec<_>>()
                .join("\n"),
        ));
        cells.push(Cell::new(
            &game_data
                .progress_flags
                .iter()
                .enumerate()
                .map(|(idx, &val)| {
                    if val { "○ " } else { "☓ " }.to_owned() + PROGRESS_FLAG_TABLE[idx]
                })
                .collect::<Vec<_>>()
                .join("\n"),
        ));
        cells.push(Cell::new(
            format!("{} (Key: {})", game_data.checksum, game_data.encryption_key).as_str(),
        ));

        table.add_row(Row::new(cells));
    }

    // Return the table as a `String`
    table.to_string()
}
