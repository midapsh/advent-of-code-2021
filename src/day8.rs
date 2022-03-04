use std::{collections::HashMap, ops::Deref};

const _DUMMY_INPUT: &str = include_str!("data/day8-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day8-real.txt");

fn private_solve_part_1(values: &str) -> String {
    // let (unique_signals, digit_output) = values.lines().map(|line| line.split_once('|'));
    values
        .lines()
        .map(|line| {
            // let (unique_signals, digit_output) = line
            //     .split_once('|')
            //     .map(|(unique_signals, digit_output)| {
            //         (
            //             unique_signals.trim().split(' '),
            //             digit_output.trim().split(' '),
            //         )
            //     })
            //     .unwrap();
            line.split_once('|')
                .map(|(_, digit_output)| digit_output.trim().split(' '))
                .unwrap()
                .filter(|digit_output| match digit_output.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(u_signals, d_output)| {
                    let (signals, outputs) = (
                        u_signals
                            .trim()
                            .split(' ')
                            .map(sort_string)
                            .collect::<Vec<_>>(),
                        d_output
                            .trim()
                            .split(' ')
                            .map(sort_string)
                            .collect::<Vec<_>>(),
                    );
                    let number_map = parse_signals(signals);
                    get_signal_number(outputs, number_map)
                })
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_signals(signals: Vec<String>) -> HashMap<String, String> {
    let mut signal_map: HashMap<usize, String> = HashMap::new();
    for signal in signals.iter().map(|x| x.deref()) {
        match signal.len() {
            2 => signal_map.entry(1).or_insert(signal.to_string()),
            3 => signal_map.entry(7).or_insert(signal.to_string()),
            4 => signal_map.entry(4).or_insert(signal.to_string()),
            7 => signal_map.entry(8).or_insert(signal.to_string()),
            _ => continue,
        };
        if signal_map.len() == 4 {
            break;
        }
    }

    let mut letter_map: HashMap<&str, String> = HashMap::new();

    let a = signal_map[&7].replace(&signal_map[&1].chars().collect::<Vec<char>>()[..], "");
    letter_map.entry("a").or_insert(a);

    let bd = signal_map[&4].replace(&signal_map[&1].chars().collect::<Vec<char>>()[..], "");
    letter_map.entry("bd").or_insert(bd);

    // Numbers available
    // 1 4 7 8
    for supposed_0 in signals.iter().map(|x| x.deref()) {
        if supposed_0.len() == 6 {
            let supposed_d = signal_map[&8]
                .clone()
                .replace(&supposed_0.chars().collect::<Vec<char>>()[..], "");
            let supposed_b = letter_map[&"bd"]
                .clone()
                .replace(&supposed_d.chars().collect::<Vec<char>>()[..], "");

            if supposed_b.len() == 1 {
                letter_map.entry("b").or_insert(supposed_b);
                letter_map.entry("d").or_insert(supposed_d);
                signal_map.entry(0).or_insert(supposed_0.to_string());
                break;
            }
        }
    }

    // Numbers available
    // 0 1 4 7 8
    for supposed_6 in signals.iter().map(|x| x.deref()) {
        if supposed_6.len() == 6 {
            let number_1 = signal_map[&1].clone();
            let supposed_c = signal_map[&8]
                .clone()
                .replace(&supposed_6.chars().collect::<Vec<char>>()[..], "");
            let supposed_f = number_1.replace(&supposed_c.chars().collect::<Vec<char>>()[..], "");

            if supposed_f.len() == 1 {
                letter_map.entry("c").or_insert(supposed_c);
                letter_map.entry("f").or_insert(supposed_f);
                signal_map.entry(6).or_insert(supposed_6.to_string());
                break;
            }
        }
    }

    // Numbers available
    // 0 1 4 6 7 8
    let zero = signal_map[&0].deref();
    let six = signal_map[&6].deref();
    let eight = signal_map[&8].deref();
    for number_9 in signals.iter().map(|x| x.deref()) {
        if number_9.len() == 6 && number_9 != zero && number_9 != six {
            let abcdf = format!(
                "{}{}{}{}{}",
                letter_map[&"a"],
                letter_map[&"b"],
                letter_map[&"c"],
                letter_map[&"d"],
                letter_map[&"f"],
            );
            let letter_g = number_9
                .deref()
                .replace(&abcdf.chars().collect::<Vec<char>>()[..], "");

            let abcdfg = format!("{}{}", abcdf, letter_g,);
            let letter_e = eight.replace(&abcdfg.chars().collect::<Vec<char>>()[..], "");

            letter_map.entry("e").or_insert(letter_e);
            letter_map.entry("g").or_insert(letter_g);
            signal_map.entry(9).or_insert(number_9.to_string());

            break;
        }
    }

    // Numbers available
    // 0 1 4 6 7 8 9
    // To find number 2
    let letter_c = letter_map[&"c"].clone();
    let letter_e = letter_map[&"e"].clone();
    // To find number 5
    let letter_b = letter_map[&"b"].clone();
    let letter_f = letter_map[&"f"].clone();

    for supposed_2_3_or_5 in signals.iter().map(|x| x.deref()) {
        if supposed_2_3_or_5.len() == 5 {
            if supposed_2_3_or_5.contains(letter_c.as_str())
                && supposed_2_3_or_5.contains(letter_e.as_str())
            {
                signal_map.entry(2).or_insert(supposed_2_3_or_5.to_string());
            } else if supposed_2_3_or_5.contains(letter_b.as_str())
                && supposed_2_3_or_5.contains(letter_f.as_str())
            {
                signal_map.entry(5).or_insert(supposed_2_3_or_5.to_string());
            } else {
                signal_map.entry(3).or_insert(supposed_2_3_or_5.to_string());
            }
        }
    }

    let mut number_map: HashMap<String, String> = HashMap::new();
    for (k, v) in signal_map.into_iter() {
        number_map.entry(v).or_insert(k.to_string());
    }
    number_map
}

fn get_signal_number(outputs: Vec<String>, number_map: HashMap<String, String>) -> usize {
    let mut signal_number = vec![];
    for output in outputs.into_iter() {
        let correct_number = number_map[&output].clone();
        signal_number.push(correct_number);
    }
    signal_number
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn sort_string(value: &str) -> String {
    let mut chars: Vec<char> = value.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars.iter().collect::<String>()
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("26", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("61229", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real());
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
