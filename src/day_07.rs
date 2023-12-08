use std::collections::HashMap;

fn _decode_card(card: [u8; 5]) -> String {
    let mut c = Vec::new();

    for i in 0..5 {
        c.push(match card[i] {
            12 => 'A',
            11 => 'K',
            10 => 'Q',
            9 => 'J',
            8 => 'T',
            7 => '9',
            6 => '8',
            5 => '7',
            4 => '6',
            3 => '5',
            2 => '4',
            1 => '3',
            0 => '2',
            _ => '2',
        });
    }

    c.into_iter().collect()
}

fn parse_char(char: char) -> u8 {
    match char {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => 0,
    }
}

fn get_pattern_value(_card: &[u8; 5]) -> u8 {
    let mut card: [u8; 13] = [0; 13];
    for char in _card {
        card[*char as usize] += 1;
    }
    card.sort_by(|a, b| b.cmp(a));

    if card[0] == 1 {
        // high card
        return 0;
    }

    if card[0] == 5 {
        return 6;
    }

    if card[0] == 4 {
        // four of a kind
        return 5;
    }

    if card[0] == 3 {
        if card[1] == 2 {
            // full house
            return 4;
        } else {
            // three of a kind
            return 3;
        }
    }

    if card[0] == 2 {
        if card[1] == 2 {
            // two pair
            return 2;
        } else {
            // one pair
            return 1;
        }
    }

    return 0;
}

fn iterate_indices_for_highest_card(
    card: &mut [u8; 5],
    indices: &mut Vec<usize>,
    depth: usize,
    value: &mut u8,
) {
    let amount = indices.len();

    if depth == amount {
        let new_value = get_pattern_value(card);
        if new_value > *value {
            *value = new_value;
        }
        return;
    }

    for id in 0..13 {
        card[indices[depth]] = id;
        iterate_indices_for_highest_card(card, indices, depth + 1, value);
    }
}

fn get_highest_card(_card: &[u8; 5]) -> u8 {
    let mut card = _card.clone();

    let indices: Vec<usize> = card
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x == 0)
        .map(|(i, _)| i)
        .collect();

    let mut value = get_pattern_value(&card);

    iterate_indices_for_highest_card(&mut card, &mut indices.clone(), 0, &mut value);

    return value;
}

fn parse_card(card: &str) -> u64 {
    let chars = card.chars();
    let mut values: [u8; 5] = [0; 5];

    let mut char_index: usize = 0;
    for char in chars.clone().into_iter() {
        values[char_index] = parse_char(char);
        char_index += 1;
    }

    let mut pattern_value = get_pattern_value(&values);

    // if the card contains J (8) find the highest pattern
    if values.contains(&0) {
        // println!("{:?}", card);
        let new_value = get_highest_card(&values);
        if new_value > pattern_value {
            // println!("new value is higher {} -> {}", pattern_value, new_value);
            pattern_value = new_value;
        }
    }

    let a1: u64 = 1_000_000_000_000_000 * u64::from(pattern_value);
    let a2: u64 = 1_000_000_000_000 * u64::from(values[0]);
    let a3: u64 = 1_000_000_000 * u64::from(values[1]);
    let a4: u64 = 1_000_000 * u64::from(values[2]);
    let a5: u64 = 1_000 * u64::from(values[3]);
    let a6: u64 = 1 * u64::from(values[4]);

    return a1 + a2 + a3 + a4 + a5 + a6;
}

pub fn solve(input: String) -> (u64, u64) {
    let mut card_map: HashMap<&str, u64> = HashMap::new();

    // parse cards
    let mut cards: Vec<(u64, u64)> = input
        .lines()
        .into_iter()
        .map(|s| {
            let words = s.split_whitespace().collect::<Vec<&str>>();

            let card = words.first().unwrap();
            let bid = words.last().unwrap().parse::<u64>().unwrap();

            if card_map.contains_key(card) {
                println!("found in map");
                return (*card_map.get(card).unwrap(), bid);
            }

            let value = parse_card(card);
            card_map.insert(card, value);

            return (value, bid);
        })
        .collect::<Vec<(u64, u64)>>();

    cards.sort_by(|a, b| b.0.cmp(&a.0));

    let mut total_value: u64 = 0;

    let card_amount = cards.len();
    for card_index in 0..card_amount {
        let value = (card_amount - card_index) as u64;
        let card = cards[card_index];
        total_value += value * card.1;
    }

    return (total_value, 0);
}
