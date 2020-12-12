const INPUT: &str = "yzbqklnj";

fn to_hash(key: &str, num: usize) -> [u8; 16] {
    let s = format!("{}{}", key, num);
    md5::compute(s).0
}

fn is_fit_a(d: [u8; 16]) -> bool {
    d[0] == 0 && d[1] == 0 && d[2] < 16
}

fn run_4a_with_input(input: &str) -> usize {
    let mut i = 0;
    loop {
        if is_fit_a(to_hash(input, i)) {
            return i;
        }
        i += 1;
    }
}

pub fn run_4a() -> usize {
    run_4a_with_input(INPUT)
}

fn is_fit_b(d: [u8; 16]) -> bool {
    d[0] == 0 && d[1] == 0 && d[2] == 0
}

fn run_4b_with_input(input: &str) -> usize {
    let mut i = 0;
    loop {
        if is_fit_b(to_hash(input, i)) {
            return i;
        }
        i += 1;
    }
}

pub fn run_4b() -> usize {
    run_4b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_hash() {
        let key = "abcdef";
        let num = 609043;

        let hash = to_hash(key, num);

        assert_eq!(&hash[0..3], &[0, 0, 1]);
    }

    #[test]
    fn sample_4a() {
        let key = "abcdef";

        assert_eq!(run_4a_with_input(key), 609043);
    }
}
