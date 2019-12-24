pub const DECK_SIZE: i64 = 10007;
type Deck = Vec<i64>;

pub fn create_cards(n: i64) -> Deck {
    (0..n).collect()
}

fn cut(deck: Deck, n: i64) -> Deck {
    let len = deck.len() as i64;
    let cut_point = if n < 0 { len + n } else { n };
    let mut new_deck = Vec::with_capacity(deck.len());

    let deck_itr = (cut_point..len).chain(0..cut_point);
    for i in deck_itr {
        new_deck.push(deck[i as usize]);
    }

    new_deck
}

fn deal_into_new_stack(mut deck: Deck) -> Deck {
    deck.reverse();
    deck
}

fn deal_with_increment(deck: Deck, n: i64) -> Deck {
    let mut new_deck = vec![0; deck.len()];
    for i in 0..new_deck.len() {
        let offset = (i * (n as usize)) % deck.len();
        new_deck[offset] = deck[i];
    }

    new_deck
}

#[derive(Debug)]
enum Instructions {
    NewStack,
    Increment(i64),
    Cut(i64),
}

fn parse_instructions(instructions: &str) -> Vec<Instructions> {
    instructions
        .trim()
        .lines()
        .map(|line| {
            let inst = line.trim();
            let components: Vec<&str> = inst.split(' ').collect();
            if components[0] == "cut" {
                let n = components[1].parse().unwrap();
                Instructions::Cut(n)
            } else if components[0] == "deal" && components[3] == "stack" {
                Instructions::NewStack
            } else if components[0] == "deal" && components[2] == "increment" {
                let n = components[3].parse().unwrap();
                Instructions::Increment(n)
            } else {
                panic!("Unexpected line {}", inst);
            }
        })
        .collect()
}

pub fn shuffle(instructions: &str, deck_size: i64) -> Deck {
    let mut deck = create_cards(deck_size);
    let instructions = parse_instructions(instructions);
    for instruction in instructions {
        deck = match instruction {
            Instructions::Cut(n) => cut(deck, n),
            Instructions::NewStack => deal_into_new_stack(deck),
            Instructions::Increment(n) => deal_with_increment(deck, n),
        };
    }

    deck
}

pub fn shuffle_track_card(instructions: &str, deck_size: i64, card: i64) -> i64 {
    let instructions = parse_instructions(instructions);
    let mut card_loc = card; // Assume deck starts out in order
    for instruction in instructions {
        card_loc = match instruction {
            Instructions::Cut(n) => {
                let cut_point = if n < 0 { deck_size + n } else { n };

                if card_loc < cut_point {
                    deck_size - cut_point + card_loc
                } else {
                    card_loc - cut_point
                }
            }
            Instructions::NewStack => deck_size - card_loc - 1,
            Instructions::Increment(n) => (card_loc * n) % deck_size,
        };
    }

    card_loc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let instructions = "
        deal with increment 7
        deal into new stack
        deal into new stack
        ";

        let deck = shuffle(instructions, 10);
        let expected = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        assert_eq!(deck, expected);
    }

    #[test]
    fn example_2() {
        let instructions = "deal into new stack
        cut -2
        deal with increment 7
        cut 8
        cut -4
        deal with increment 7
        cut 3
        deal with increment 9
        deal with increment 3
        cut -1";

        let deck = shuffle(instructions, 10);
        let expected = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        assert_eq!(deck, expected);
    }

    #[test]
    fn twentytwo_1() {
        let deck = shuffle(PROGRAM_22, DECK_SIZE);
        let pos = deck.iter().position(|&x| x == 2019).unwrap();
        assert_eq!(pos, 7665);
    }

    #[test]
    fn twentytwo_1_b() {
        let loc = shuffle_track_card(PROGRAM_22, DECK_SIZE, 2019);
        assert_eq!(loc, 7665);
    }

    #[test]
    fn twentytwo_2() {
        //let loc = shuffle_track_card(PROGRAM_22, DECK_SIZE, 2019);
        //assert_eq!(loc, 7665);
    }

    const PROGRAM_22: &str = "deal with increment 73
    cut -8387
    deal with increment 41
    cut 190
    deal with increment 4
    cut 6396
    deal with increment 47
    cut -9579
    deal with increment 47
    cut -1296
    deal with increment 2
    cut 3807
    deal with increment 75
    cut 8267
    deal with increment 53
    cut 5108
    deal with increment 20
    cut -62
    deal with increment 63
    cut 4435
    deal into new stack
    deal with increment 2
    cut 8436
    deal with increment 52
    cut 8420
    deal with increment 70
    cut -7602
    deal with increment 39
    cut 6737
    deal into new stack
    cut -3549
    deal with increment 63
    deal into new stack
    cut -2925
    deal with increment 59
    cut -9525
    deal with increment 12
    deal into new stack
    deal with increment 7
    cut 4619
    deal with increment 27
    cut 7141
    deal with increment 69
    cut 5221
    deal with increment 19
    cut 4288
    deal into new stack
    deal with increment 64
    cut -1618
    deal with increment 63
    cut -9384
    deal with increment 24
    deal into new stack
    deal with increment 54
    cut 429
    deal into new stack
    cut 2190
    deal with increment 28
    cut -4420
    deal with increment 10
    cut 6968
    deal with increment 34
    cut 8566
    deal with increment 4
    cut 8979
    deal with increment 58
    deal into new stack
    deal with increment 17
    deal into new stack
    cut -3775
    deal with increment 72
    cut 3378
    deal with increment 40
    cut -7813
    deal into new stack
    deal with increment 26
    deal into new stack
    cut 5504
    deal with increment 64
    deal into new stack
    cut 3592
    deal with increment 13
    cut 4123
    deal into new stack
    deal with increment 67
    deal into new stack
    cut 1943
    deal with increment 72
    cut -5205
    deal into new stack
    deal with increment 12
    cut 1597
    deal with increment 10
    cut 4721
    deal with increment 36
    cut 3379
    deal into new stack
    cut -5708
    deal with increment 61
    cut 6852";
}
