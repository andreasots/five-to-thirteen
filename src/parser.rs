pub const FIVE: u8 = 0;
pub const ADD: u8 = 1;
pub const SUB: u8 = 2;
pub const MUL: u8 = 3;
pub const DIV: u8 = 4;
pub const LPAREN: u8 = 5;
pub const RPAREN: u8 = 6;
pub const BASE: u64 = 7;

macro_rules! try {
    ($term:ident($input:ident)) => {{
        let (res, input) = $term($input)?;
        $input = input;
        res
    }};
    ($term:ident(&$input:ident[$index:expr])) => {{
        let (res, input) = $term(&$input[$index])?;
        $input = input;
        res
    }};
}

// expr ::= prod ([ADD SUB] prod)*
pub fn expr(mut input: &[u8]) -> Result<(f64, &[u8]), ()> {
    let mut accumulator = try!(prod(input));
    loop {
        match input.get(0) {
            Some(&ADD) => accumulator += try!(prod(&input[1..])),
            Some(&SUB) => accumulator -= try!(prod(&input[1..])),
            _ => return Ok((accumulator, input)),
        }
    }
}

// prod ::= num ([MUL DIV] num)*
pub fn prod(mut input: &[u8]) -> Result<(f64, &[u8]), ()> {
    let mut accumulator = try!(num(input));
    loop {
        match input.get(0) {
            Some(&MUL) => accumulator *= try!(num(&input[1..])),
            Some(&DIV) => accumulator /= try!(num(&input[1..])),
            _ => return Ok((accumulator, input)),
        }
    }
}

// num ::= FIVE | LPAREN expr RPAREN
pub fn num(input: &[u8]) -> Result<(f64, &[u8]), ()> {
    match input.get(0) {
        Some(&FIVE) => Ok((5.0, &input[1..])),
        Some(&LPAREN) => {
            let (val, input) = expr(&input[1..])?;
            if input.get(0) != Some(&RPAREN) {
                Err(())
            } else {
                Ok((val, &input[1..]))
            }
        },
        _ => Err(()),
    }
}

pub fn format(input: &[u8]) -> String {
    let mut ret = String::new();

    for c in input {
        match *c {
            FIVE => ret.push_str("5"),
            ADD => ret.push_str(" + "),
            SUB => ret.push_str(" - "),
            MUL => ret.push_str(" * "),
            DIV => ret.push_str(" / "),
            LPAREN => ret.push_str("("),
            RPAREN => ret.push_str(")"),
            _ => unreachable!(),
        }
    }

    ret
}

#[test]
fn minimal_conjecture() {
    assert_eq!(expr(&[LPAREN, FIVE, MUL, FIVE, MUL, FIVE, ADD, FIVE, RPAREN, DIV, LPAREN, FIVE, ADD, FIVE, RPAREN]), Ok((13.0f64, &[][..])));
}

#[test]
fn all_len_13_candidates() {
    assert_eq!(expr(&[LPAREN, FIVE, ADD, FIVE, ADD, FIVE, RPAREN, DIV, FIVE, ADD, FIVE, ADD, FIVE]), Ok((13.0, &[][..])));
    assert_eq!(expr(&[FIVE, SUB, FIVE, DIV, FIVE, ADD, FIVE, DIV, FIVE, SUB, FIVE, ADD, FIVE]), Ok((5.0, &[][..])));
    assert_eq!(expr(&[FIVE, SUB, LPAREN, FIVE, ADD, FIVE, RPAREN, DIV, FIVE, SUB, FIVE, ADD, FIVE]), Ok((3.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, LPAREN, FIVE, ADD, FIVE, ADD, FIVE, RPAREN, DIV, FIVE, ADD, FIVE]), Ok((13.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, SUB, FIVE, DIV, FIVE, ADD, FIVE, DIV, FIVE, SUB, FIVE]), Ok((5.0, &[][..])));
    assert_eq!(expr(&[FIVE, SUB, FIVE, DIV, FIVE, SUB, FIVE, SUB, FIVE, DIV, FIVE, SUB, FIVE]), Ok((-7.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, SUB, LPAREN, FIVE, ADD, FIVE, RPAREN, DIV, FIVE, SUB, FIVE]), Ok((3.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, ADD, FIVE, SUB, FIVE, DIV, FIVE, ADD, FIVE, DIV, FIVE]), Ok((15.0, &[][..])));
    assert_eq!(expr(&[FIVE, SUB, FIVE, DIV, FIVE, SUB, FIVE, ADD, FIVE, SUB, FIVE, DIV, FIVE]), Ok((3.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, SUB, FIVE, DIV, FIVE, SUB, FIVE, SUB, FIVE, DIV, FIVE]), Ok((3.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, ADD, LPAREN, FIVE, ADD, FIVE, ADD, FIVE, RPAREN, DIV, FIVE]), Ok((13.0, &[][..])));
    assert_eq!(expr(&[FIVE, ADD, FIVE, ADD, FIVE, SUB, LPAREN, FIVE, ADD, FIVE, RPAREN, DIV, FIVE]), Ok((13.0, &[][..])));
}