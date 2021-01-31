use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::map_res;
use nom::error::{context, VerboseError};
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

pub(crate) type N = usize;
pub(crate) type Res<T, U> = IResult<T, U, VerboseError<T>>;

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

fn number(input: &str) -> Res<&str, &str> {
    context("number", take_while1(|c: char| c.is_digit(10)))(input)
}

fn single_space(input: &str) -> Res<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}

fn positive_number(input: &str) -> Res<&str, N> {
    map_res(number, |out| N::from_str_radix(out, 10))(input)
}

fn header(input: &str) -> Res<&str, Header> {
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

fn header_line(s: &str) -> Res<&str, Header> {
    context("header", terminated(header, tag("\n")))(s)
}

fn number_list(s: &str) -> Res<&str, Vec<N>> {
    separated_list1(single_space, positive_number)(s)
}

fn number_list_line(s: &str) -> Res<&str, Vec<N>> {
    terminated(number_list, tag("\n"))(s)
}

fn parse_input_body<'a>(s: &'a str, header: &Header) -> Res<&'a str, Body> {
    let (out, hits) = context(
        "hits",
        many_m_n(header.towers, header.towers, number_list_line),
    )(s)?;
    let (out, costs) = context("costs", number_list_line)(out)?;
    let (out, bonus) = context("bonus", number_list_line)(out)?;
    let (out, waves) = context("waves", separated_list1(tag("\n"), number_list))(out)?;
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

pub(crate) fn parse_input(s: &str) -> Res<&str, Input> {
    let (out, header) = header_line(s)?;
    let (out, body) = parse_input_body(out, &header)?;
    Ok((out, Input { header, body }))
}

pub(crate) fn parse_output(s: &str) -> Res<&str, Output> {
    let (out, waves) = context("output", separated_list1(tag("\n"), number_list))(s)?;
    Ok((out, Output { waves }))
}

#[cfg(test)]
mod tests {

    use crate::parser::{
        header, number, number_list, number_list_line, parse_input, parse_input_body, parse_output,
        positive_number, Body, Header, Input, Output,
    };

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
        assert!(r.is_err())
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
}
