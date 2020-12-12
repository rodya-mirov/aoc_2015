use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/5.txt");

fn is_nice_a(s: &str) -> bool {
    let mut chars = s.chars();

    let is_vowel = |c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';

    let mut last_char = chars.next().unwrap();

    let mut num_vowels = is_vowel(last_char) as usize;
    let mut num_doubles = 0;

    for c in chars {
        if c == last_char {
            num_doubles += 1;
        }
        if is_vowel(c) {
            num_vowels += 1;
        }

        match (last_char, c) {
            ('a', 'b') => return false,
            ('c', 'd') => return false,
            ('p', 'q') => return false,
            ('x', 'y') => return false,
            _ => {}
        }

        last_char = c;
    }

    num_vowels >= 3 && num_doubles >= 1
}

fn run_5a_with_input(input: &str) -> usize {
    input.lines().filter(|s| is_nice_a(s)).count()
}

pub fn run_5a() -> usize {
    run_5a_with_input(INPUT)
}

fn is_nice_b(s: &str) -> bool {
    let mut chars = s.chars();

    let mut seen = HashSet::new();

    // If this panics just unwrap or return false
    let mut char_3 = chars.next().unwrap();
    let mut char_2 = chars.next().unwrap();
    seen.insert(char_3);
    seen.insert(char_2);

    // c -> starts of indices of pairs of c
    let mut doubles: HashMap<(char, char), Vec<usize>> = HashMap::new();
    doubles.entry((char_3, char_2)).or_default().push(0);

    let mut sep_double = false;

    for (i, c) in chars.enumerate() {
        doubles.entry((char_2, c)).or_default().push(i + 1);

        if c == char_3 {
            sep_double = true;
        }

        char_3 = char_2;
        char_2 = c;
        seen.insert(c);
    }

    if !sep_double {
        return false;
    }

    for (_, doubles) in doubles {
        if doubles.len() < 2 {
            continue;
        }

        let last = doubles.get(doubles.len() - 1).copied().unwrap();
        let first = doubles[0];

        if last > first + 1 {
            return true;
        }
    }

    false
}

fn run_5b_with_input(input: &str) -> usize {
    input.lines().filter(|s| is_nice_b(s)).count()
}

pub fn run_5b() -> usize {
    run_5b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_5a() {
        assert_eq!(is_nice_a("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_a("aaa"), true);
        assert_eq!(is_nice_a("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_a("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_a("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn sample_5b() {
        assert_eq!(is_nice_b("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_b("xxyxx"), true);
        assert_eq!(is_nice_b("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_b("ieodomkazucvgmuy"), false);
    }
}
