use core::{num, panic};

use crate::{split_on_lines, split_on_space};

// THE LESSON WE HAVE LEARNED IS TO NEVER DROP INFORMATION

#[derive(Debug, Clone)]
struct ScratchOffCard {
    stack_position: u32,
    multiplicity: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    point_value: u32,
}

pub fn day_four(file_contents: String) {
    let mut cards = parse_card_stack(split_on_lines(file_contents));
    duplicate_cards(&mut cards);
    // for card in cards {
    //     println!("multiplicity: {}", card.multiplicity)
    // }
    println!("{}", cards.iter().map(|x| x.multiplicity).fold(0, |acc, x| acc + x))
}

fn duplicate_cards(cards: &mut Vec<ScratchOffCard>) {
    for i in 0..cards.len() {
        for _ in 0..cards[i].multiplicity {
            for j in 1..=cards[i].point_value {
                match cards.get_mut(i + j as usize) {
                    Some(lower_card) => lower_card.multiplicity += 1,
                    None => (),
                }
            }
        }
    }
}

// fn primary_mutation(numbers: &mut Vec<u32>) {
//     for i in 0..5 {
//         secondary_mutation(numbers);
//     }
// }

// fn secondary_mutation(same_numbers: &mut Vec<u32>) {
//     match same_numbers.get(0) {
//         Some(element) => element = 0 as u32,
//         None => todo!(),
//     }
// }

fn parse_card_stack(card_stack: Vec<String>) -> Vec<ScratchOffCard> {
    card_stack.iter().map(|card| parse_individual_card(card)).collect::<Vec<ScratchOffCard>>()
}

fn parse_individual_card(card: &String) -> ScratchOffCard{
    let card_parts: Vec<&str> = card.split(":").collect();
    let position: u32 = get_card_position(card_parts[0]);
    let mult = 1;
    let numbers_parts: &str = card_parts[1];
    let numbers_parts: Vec<&str> = numbers_parts.split("|").collect();
    let this_cards_winning_numbers =  match numbers_parts.get(0){
        Some(nums) => get_numbers_from_str(*nums),
        None => panic!("Parsing Error on individual cards - index dne"),
    };
    let this_cards_numbers = match numbers_parts.get(1) {
        Some(nums) => get_numbers_from_str(*nums),
        None => panic!("Parsing Error on individual cards - index dne"),
    };
    ScratchOffCard { stack_position: position,
        multiplicity: 1,
        winning_numbers: this_cards_winning_numbers.clone(),
        card_numbers: this_cards_numbers.clone(),
        point_value: match_count(&this_cards_winning_numbers,&this_cards_numbers) 
    }
}

fn match_count(vec1: &Vec<u32>, vec2: &Vec<u32>) -> u32 {
    vec2.iter().filter(|element| vec1.contains(element)).count() as u32
}


fn get_numbers_from_str(s: &str) -> Vec<u32> {
    split_on_space(&s.to_string()).iter().map(|x| x.parse().unwrap()).collect()
}

fn get_card_position(card_number: &str) -> u32 {
    card_number.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap() - 1 as u32
}

// fn parse_card(card: String) -> ScratchOffCard{
//     let mut card = card.split("|");
//     let winners = match card.next() {
//         Some(numbers) => {split_on_space(numbers.to_string()).iter().map(|x| x.parse().unwrap()).collect::<Vec<u32>>()},
//         None => panic!("No numbers found in winners_string")
//     };
//     let this_cards_numbers = match card.next() {
//         Some(numbers) => {split_on_space(numbers.to_string()).iter().map(|x| x.parse().unwrap()).collect::<Vec<u32>>()},
//         None => panic!("No numbers found in numbers_string")
//     };
//     ScratchOffCard {
//         winning_numbers: winners.clone(),
//         card_numbers: this_cards_numbers.clone(),
//         point_value: get_point_value(winners, this_cards_numbers)
//     }

// }

// fn get_point_value(winners: Vec<u32>, numbers: Vec<u32>) -> u32 {
//     match numbers.iter().filter(|x| winners.contains(x)).count() {
//         0 => 0,
//         n => (2 as u32).pow(n as u32 - 1),
//     }
// }

