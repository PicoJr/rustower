use nom::{IResult};
use nom::sequence::tuple;
use nom::combinator::{map_res};
use nom::bytes::complete::{take_while1, take_while_m_n};

type N = usize;

#[derive(Debug, Eq, PartialEq)]
struct Header {
    units: N,
    towers: N,
    waves: N,
    budget: N,
}

fn number(input: &str) -> IResult<&str, &str> {
    take_while1(|c:char| c.is_digit(10))(input)
}

fn single_space(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, |c:char| c.is_ascii_whitespace())(input)
}

fn positive_number(input: &str) -> IResult<&str, N> {
    map_res(number,
        |out| N::from_str_radix(out, 10)
    )(input)
}

fn header(input: &str) -> IResult<&str, Header> {
    // tuple takes as argument a tuple of parsers and will return
    // a tuple of their results
    let (i, (units, _, towers, _, waves, _, budget)) =
        tuple((positive_number, single_space, positive_number, single_space, positive_number, single_space, positive_number))(input)?;

    Ok((i, Header {
        units,
        towers,
        waves,
        budget,
    }))
}

#[cfg(test)]
mod tests {

    use crate::parser::{number, positive_number, header, Header};
    use nom::error::{ErrorKind, Error};
    use nom::Err;

    #[test]
    fn test_number_1() {
        let r = number("0");
        assert_eq!(r, Ok(("", "0")))
    }

    #[test]
    fn test_number_2() {
        let r = number("42");
        assert_eq!(r, Ok(("", "42")))
    }

    #[test]
    fn test_number_ko() {
        let r = number("not a number");
        assert_eq!(r, Err(Err::Error(Error::new("not a number", ErrorKind::TakeWhile1))))
    }

    #[test]
    fn test_positive_number() {
        let r = positive_number("42");
        assert_eq!(r, Ok(("", 42)))
    }

    #[test]
    fn test_header() {
        let h = header("42 43 44 45");
        assert_eq!(h, Ok(("", Header{ units: 42, towers: 43, waves: 44, budget: 45 })))
    }

}