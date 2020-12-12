const INPUT: &str = include_str!("input/2.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Present {
    x: i64,
    y: i64,
    z: i64,
}

mod parse {
    use nom::{character::complete::char as exact_char, combinator::map, sequence::tuple, IResult};

    use crate::lib::{parse_i64, parse_lines};

    use super::Present;

    fn parse_present(input: &str) -> IResult<&str, Present> {
        map(
            tuple((
                parse_i64,
                exact_char('x'),
                parse_i64,
                exact_char('x'),
                parse_i64,
            )),
            |(x, _, y, _, z)| Present { x, y, z },
        )(input)
    }

    pub(super) fn parse(input: &str) -> Vec<Present> {
        let (_, out) = parse_lines(parse_present, input).unwrap();
        out
    }
}

fn surface_area(p: Present) -> i64 {
    let xy = p.x * p.y;
    let mut least = xy;
    let mut total = xy;

    let xz = p.x * p.z;
    least = least.min(xz);
    total += xz;

    let yz = p.y * p.z;
    least = least.min(yz);
    total += yz;

    total * 2 + least
}

fn run_2a_with_input(input: &str) -> i64 {
    let presents = parse::parse(input);

    presents.into_iter().map(surface_area).sum()
}

pub fn run_2a() -> i64 {
    run_2a_with_input(INPUT)
}

fn ribbon(p: Present) -> i64 {
    let vol = p.x * p.y * p.z;

    let perim = [p.x + p.y, p.x + p.z, p.y + p.z]
        .iter()
        .copied()
        .min()
        .unwrap()
        * 2;

    vol + perim
}

fn run_2b_with_input(input: &str) -> i64 {
    let presents = parse::parse(input);

    presents.into_iter().map(ribbon).sum()
}

pub fn run_2b() -> i64 {
    run_2b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_2a() {
        assert_eq!(surface_area(Present { x: 1, y: 1, z: 10 }), 43);
        assert_eq!(surface_area(Present { x: 2, y: 3, z: 4 }), 58);
    }

    #[test]
    fn sample_2b() {
        assert_eq!(ribbon(Present { x: 1, y: 1, z: 10 }), 14);
        assert_eq!(ribbon(Present { x: 2, y: 3, z: 4 }), 34);
    }
}
