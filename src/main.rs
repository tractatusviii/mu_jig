use mu_jig;
use pdbtbx;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space1},
    combinator::{map, map_res, opt},
    multi::count,
    number::complete::float,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq,Clone,Copy)]
enum EntryType {
    MOLECULE,
    ATOM,
}


impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::MOLECULE => write!(f, "MOLECULE"),
            EntryType::ATOM => write!(f, "ATOM"),
        }
    }
}

impl EntryType {
    const MOLECULE_STR: &'static str = "MOLECULE";
    const ATOM_STR    : &'static str = "ATOM";

    fn as_str(&self) -> &'static str {
        match self {
            EntryType::MOLECULE => Self::MOLECULE_STR,
            EntryType::ATOM => Self::ATOM_STR,
        }
    }
}

fn parse_entry_type(input: &str) -> IResult<&str, EntryType> {
    alt((
        map(tag(EntryType::MOLECULE.as_str()), |_| EntryType::MOLECULE),
        map(tag(EntryType::ATOM.as_str()), |_| EntryType::ATOM),
    ))(input)
}

fn main() {
    let input1 = "ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    let input2 = "MOLECULE   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";

    match parse_entry_type(input1) {
        Ok((remainder, entry_type)) => println!("Input1: {:?}, Remainder: {:?}", entry_type, remainder),
        Err(e) => println!("Error parsing input1: {:?}", e),
    }

    match parse_entry_type(input2) {
        Ok((remainder, entry_type)) => println!("Input2: {:?}, Remainder: {:?}", entry_type, remainder),
        Err(e) => println!("Error parsing input2: {:?}", e),
    }
}