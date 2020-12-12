use std::collections::HashSet;

const INPUT: &str = include_str!("input/3.txt");

fn run_3a_with_input(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    seen.insert((x, y));

    for c in input.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            other => panic!("Unrecognized direction {}", other),
        }

        seen.insert((x, y));
    }

    seen.len()
}

pub fn run_3a() -> usize {
    run_3a_with_input(INPUT)
}

fn run_3b_with_input(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    let mut x1 = 0;
    let mut y1 = 0;

    let mut x2 = 0;
    let mut y2 = 0;

    let mut is_first = true;

    seen.insert((x1, y1));
    seen.insert((x2, y2));

    for c in input.chars() {
        let (x, y) = if is_first {
            (&mut x1, &mut y1)
        } else {
            (&mut x2, &mut y2)
        };

        match c {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y -= 1,
            'v' => *y += 1,
            other => panic!("Unrecognized direction {}", other),
        }

        seen.insert((*x, *y));
        is_first = !is_first;
    }

    seen.len()
}

pub fn run_3b() -> usize {
    run_3b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_3a() {
        assert_eq!(run_3a_with_input(">"), 2);
        assert_eq!(run_3a_with_input("^>v<"), 4);
        assert_eq!(run_3a_with_input("^v^v^v^v^v"), 2);
    }

    #[test]
    fn sample_3b() {
        assert_eq!(run_3b_with_input("^v"), 3);
        assert_eq!(run_3b_with_input("^>v<"), 3);
        assert_eq!(run_3b_with_input("^v^v^v^v^v"), 11);
    }
}
