use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::usize;

const BF_IPV: char    = '+'; /* Increment pointer value by 1 */
const BF_DPV: char    = '-'; /* Decrement pointer value by 1 */
const BF_IP: char     = '>';
const BF_DP: char     = '<';
const BF_OUTP: char   = '.';
const BF_INP: char    = ',';
const BF_LSTART: char = '[';
const BF_LEND: char   = ']';

fn main() {
    if let Ok(input) = read_input() {
        run(&input);
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    if let Some(fname) = env::args().nth(1) {
        let mut f = try!(File::open(fname));
        try!(f.read_to_string(&mut input));
    }
    else {
        try!(io::stdin().read_line(&mut input));
    }
    Ok(input)
}

fn run(input: &str) {
    let input: Vec<char> = input.chars().filter(|c| is_bf_char(*c)).collect();
    let mut cells: Vec<Box<u32>> = vec![Box::new(0u32)];
    let mut ci: usize = 0;
    let mut ii: usize = 0;
    let mut loops: Vec<usize> = vec![];

    while ii < input.len() {
        match input[ii] {
            BF_IPV => *cells[ci] += 1,
            BF_DPV => *cells[ci] -= 1,
            BF_IP => {
                if ci == usize::MAX {
                    panic!("Out of memory; unable to allocate cell");
                }
                ci += 1;
                if ci == cells.len() {
                    cells.push(Box::new(0u32));
                }
            },
            BF_DP => {
                if ci == usize::MIN {
                    panic!("Cannot move pointer < {}", usize::MIN);
                }
                ci -= 1;
            },
            BF_LSTART => {
                if *cells[ci] == 0 {
                    let mut skip: usize = 0;
                    loop {
                        ii += 1;
                        if input[ii] == BF_LSTART {
                            skip += 1;
                        } else if input[ii] == BF_LEND{
                            if skip > 0 {
                                skip -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    loops.push(ii);
                }
            },
            BF_LEND => {
                if *cells[ci] != 0 {
                    if let Some(li) = loops.last() {
                        ii = *li;
                    } else {
                        panic!("Unmatched {} at index {}", BF_LEND, ii);
                    }
                } else {
                    loops.pop();
                }
            },
            BF_OUTP => {
                print!("{}", (*cells[ci] as u8) as char);
            },
            _ => panic!("Unknown token {:?}", input[ii]),
        }
        ii += 1;
    }
}

fn is_bf_char(c: char) -> bool {
    c == BF_OUTP || c == BF_LEND || c == BF_LSTART || c == BF_DP ||
    c == BF_INP  || c == BF_IP   || c == BF_DPV    || c == BF_IPV
}