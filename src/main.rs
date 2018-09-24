extern crate rayon;

use rayon::prelude::*;

mod parser;

fn main() {
    for len in 1usize..14 {
        println!("Searching for max len {}", len);
        (0..parser::BASE.pow(len as u32)).into_par_iter()
            .for_each(|mut i| {
                let mut buf = [0u8; 14];
                for digit in 0..len {
                    buf[digit] = (i % parser::BASE) as u8;
                    i /= parser::BASE;
                }
                if parser::expr(&buf[0..len]) == Ok((13.0, &[])) {
                    println!("{}", parser::format(&buf[0..len]));
                }
            });
    }
}
