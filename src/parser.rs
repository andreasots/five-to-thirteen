pub const FIVE: u8 = 0;
pub const ADD: u8 = 1;
pub const SUB: u8 = 2;
pub const MUL: u8 = 3;
pub const DIV: u8 = 4;
pub const LPAREN: u8 = 5;
pub const RPAREN: u8 = 6;
pub const BASE: u64 = 7;

// expr ::= prod PLUS expr | prod MINUS expr | prod
pub fn expr(input: &[u8]) -> Result<(f64, &[u8]), ()> {
    let (left, input) = prod(input)?;
    match input.get(0) {
        Some(&ADD) => {
            let (right, input) = expr(&input[1..])?;
            Ok((left + right, input))
        },
        Some(&SUB) => {
            let (right, input) = expr(&input[1..])?;
            Ok((left - right, input))
        },
        _ => Ok((left, input))
    }
}

// prod ::= num MUL prod | num DIV prod | num
pub fn prod(input: &[u8]) -> Result<(f64, &[u8]), ()> {
    let (left, input) = num(input)?;
    match input.get(0) {
        Some(&MUL) => {
            let (right, input) = prod(&input[1..])?;
            Ok((left * right, input))
        },
        Some(&DIV) => {
            let (right, input) = prod(&input[1..])?;
            Ok((left / right, input))
        },
        _ => Ok((left, input)),
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

#[test]
fn minimal_conjecture() {
    assert_eq!(expr(&[LPAREN, FIVE, MUL, FIVE, MUL, FIVE, ADD, FIVE, RPAREN, DIV, LPAREN, FIVE, ADD, FIVE, RPAREN]), Ok((13.0f64, &[][..])));
}
