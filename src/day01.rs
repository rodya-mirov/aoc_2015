const INPUT: &str = include_str!("input/1.txt");

fn run_1a_with_input(input: &str) -> i32 {
    let mut total = 0;

    for c in input.chars() {
        match c {
            '(' => total += 1,
            ')' => total -= 1,
            o => panic!("Unknown char {}", o)
        }
    }

    total
}

pub fn run_1a() -> i32 {
    run_1a_with_input(INPUT)
}

fn run_1b_with_input(input: &str) -> usize {
    let mut total = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => total += 1,
            ')' => total -= 1,
            o => panic!("Unknown char {}", o)
        }

        if total == -1 {
            return i+1;
        }
    }

    panic!("Position not found");
}

pub fn run_1b() -> usize {
    run_1b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1a() {
        assert_eq!(run_1a_with_input("(())"), 0)
    }

    #[test]
    fn sample_1b() {
        assert_eq!(run_1b_with_input(")"), 1);
        assert_eq!(run_1b_with_input("()())"), 5);
    }
}