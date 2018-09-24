mod parser;

fn main() {
    let mut buf = [0u8; 14];
    'outer: for len in 1usize..14 {
        println!("Searching for max len {}", len);
        for mut i in 0..parser::BASE.pow(len as u32) {
            for digit in 0..len {
                buf[digit] = (i % parser::BASE) as u8;
                i /= parser::BASE;
            }
            if parser::expr(&buf[0..len]) == Ok((13, &[])) {
                println!("{:?}", &buf[0..len]);
                break 'outer;
            }
        }
    }
}
