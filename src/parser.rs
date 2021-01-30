use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::map_res;
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

pub(crate) type N = usize;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Header {
    pub units: N,
    pub towers: N,
    pub waves: N,
    pub budget: N,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Body {
    pub hits: Vec<Vec<N>>,
    pub waves: Vec<Vec<N>>,
    pub bonus: Vec<N>,
    pub costs: Vec<N>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Input {
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Output {
    pub waves: Vec<Vec<N>>,
}

fn number(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_digit(10))(input)
}

fn single_space(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}

fn positive_number(input: &str) -> IResult<&str, N> {
    map_res(number, |out| N::from_str_radix(out, 10))(input)
}

fn header(input: &str) -> IResult<&str, Header> {
    // tuple takes as argument a tuple of parsers and will return
    // a tuple of their results
    let (i, (units, _, towers, _, waves, _, budget)) = tuple((
        positive_number,
        single_space,
        positive_number,
        single_space,
        positive_number,
        single_space,
        positive_number,
    ))(input)?;

    Ok((
        i,
        Header {
            units,
            towers,
            waves,
            budget,
        },
    ))
}

fn header_line(s: &str) -> IResult<&str, Header> {
    terminated(header, tag("\n"))(s)
}

fn number_list(s: &str) -> IResult<&str, Vec<N>> {
    separated_list1(single_space, positive_number)(s)
}

fn number_list_line(s: &str) -> IResult<&str, Vec<N>> {
    terminated(number_list, tag("\n"))(s)
}

fn parse_input_body<'a>(s: &'a str, header: &Header) -> IResult<&'a str, Body> {
    println!("parsing hits");
    let (out, hits) = many_m_n(header.units, header.units, number_list_line)(s)?;
    println!("parsing costs");
    let (out, costs) = number_list_line(out)?;
    println!("parsing bonus");
    let (out, bonus) = number_list_line(out)?;
    println!("parsing waves");
    let (out, waves) = separated_list1(tag("\n"), number_list)(out)?;
    println!("done parsing waves");
    Ok((
        out,
        Body {
            hits,
            waves,
            bonus,
            costs,
        },
    ))
}

pub(crate) fn parse_input(s: &str) -> IResult<&str, Input> {
    println!("parsing header");
    let (out, header) = header_line(s)?;
    println!("header: {:?}", header);
    println!("parsing input body");
    let (out, body) = parse_input_body(out, &header)?;
    println!("done parsing input body");
    Ok((out, Input { header, body }))
}

pub(crate) fn parse_output(s: &str) -> IResult<&str, Output> {
    let (out, waves) = separated_list1(tag("\n"), number_list)(s)?;
    Ok((out, Output { waves }))
}

#[cfg(test)]
mod tests {

    use crate::parser::{
        header, number, number_list, number_list_line, parse_input, parse_input_body, parse_output,
        positive_number, Body, Header, Input, Output,
    };
    use nom::bytes::complete::tag;
    use nom::error::{Error, ErrorKind};
    use nom::multi::separated_list1;
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
        assert_eq!(
            r,
            Err(Err::Error(Error::new(
                "not a number",
                ErrorKind::TakeWhile1
            )))
        )
    }

    #[test]
    fn test_positive_number() {
        let r = positive_number("42");
        assert_eq!(r, Ok(("", 42)))
    }

    #[test]
    fn test_header() {
        let h = header("42 43 44 45");
        assert_eq!(
            h,
            Ok((
                "",
                Header {
                    units: 42,
                    towers: 43,
                    waves: 44,
                    budget: 45
                }
            ))
        )
    }

    #[test]
    fn test_number_list() {
        let nbl = number_list("42 43 44");
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_number_list_line() {
        let nbl = number_list_line("42 43 44\n");
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_parse_input_body() {
        let header = Header {
            units: 2,
            towers: 2,
            waves: 3,
            budget: 4,
        };
        let text = "\
        0 1\n\
        1 0\n\
        2 2\n\
        1 2 3\n\
        1 1\n\
        2 2\n\
        3 3";
        let body = parse_input_body(text, &header);
        assert_eq!(
            body,
            Ok((
                "",
                Body {
                    hits: vec![vec![0, 1], vec![1, 0]],
                    waves: vec![vec![1, 1], vec![2, 2], vec![3, 3]],
                    bonus: vec![1, 2, 3],
                    costs: vec![2, 2]
                }
            ))
        )
    }

    #[test]
    fn test_parse_input_2() {
        let text = "\
        2 2 3 4\n\
        0 1\n\
        1 0\n\
        2 2\n\
        1 2 3\n\
        1 1\n\
        2 2\n\
        3 3";
        let expected_header = Header {
            units: 2,
            towers: 2,
            waves: 3,
            budget: 4,
        };
        let expected_body = Body {
            hits: vec![vec![0, 1], vec![1, 0]],
            waves: vec![vec![1, 1], vec![2, 2], vec![3, 3]],
            bonus: vec![1, 2, 3],
            costs: vec![2, 2],
        };
        let body = parse_input(text);
        assert_eq!(
            body,
            Ok((
                "",
                Input {
                    header: expected_header,
                    body: expected_body
                }
            ))
        )
    }

    #[test]
    fn test_parse_input_1() {
        let text = "\
        1 1 1 2\n\
        1\n\
        1\n\
        3\n\
        1";
        let expected_header = Header {
            units: 1,
            towers: 1,
            waves: 1,
            budget: 2,
        };
        let expected_body = Body {
            hits: vec![vec![1]],
            waves: vec![vec![1]],
            bonus: vec![3],
            costs: vec![1],
        };
        let body = parse_input(text);
        assert_eq!(
            body,
            Ok((
                "",
                Input {
                    header: expected_header,
                    body: expected_body
                }
            ))
        )
    }

    #[test]
    fn test_parse_output_with_newline() {
        let text = "1\n";
        let output = parse_output(&text);
        assert_eq!(
            output,
            Ok((
                "\n",
                Output {
                    waves: vec![vec![1]]
                }
            ))
        )
    }

    #[test]
    fn test_parse_output_without_newline() {
        let text = "1";
        let output = parse_output(&text);
        assert_eq!(
            output,
            Ok((
                "",
                Output {
                    waves: vec![vec![1]]
                }
            ))
        )
    }

    #[test]
    fn test_separated_without_newline() {
        let text = "1";
        let output = separated_list1(tag("\n"), number_list)(text);
        assert_eq!(output, Ok(("", vec![vec![1]])))
    }

    #[test]
    fn test_separated_with_newline() {
        let text = "1\n";
        let output = separated_list1(tag("\n"), number_list)(text);
        assert_eq!(output, Ok(("\n", vec![vec![1]])))
    }

    #[test]
    fn test_separated_without_newline_2() {
        let text = "1\n2";
        let output = separated_list1(tag("\n"), number_list)(text);
        assert_eq!(output, Ok(("", vec![vec![1], vec![2]])))
    }
}
