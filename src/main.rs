use std::fmt::Error;

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
        map(tag(EntryType::ATOM    .as_str()), |_| EntryType::ATOM    ),
    ))(input)
}


// ----------

fn parse_spaced_id(input: &str) -> IResult<&str, u32> {
    preceded(space1, map_res(digit1, str::parse))(input)
}
fn parse_label_atom_id(input: &str) -> IResult<&str, String> {
    preceded(space1, map_res(alphanumeric1, str::parse))(input)
}

fn parse_spaced_type_symbol(input: &str) -> IResult<&str, String> {
    preceded(space1, map_res(alphanumeric1, |s: &str| Ok::<String, Error>(s.to_string())))(input)
}

fn parse_spaced_cartn_x(input: &str) -> IResult<&str, f32> {
    preceded(space1, float)(input)
}

fn main() {
    let input1 = "ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    let input2 = "MOLECULE   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";

     let input = "ATOM   1     N H     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    
    let (remainder, _)             = parse_entry_type(input).unwrap();
    let (remainder, id)            = parse_spaced_id(remainder).unwrap();
    let (remainder, type_symbol)   = parse_spaced_type_symbol(remainder).unwrap();
    let (remainder, label_atom_id) = parse_label_atom_id(remainder).unwrap();
    // let (_, cartn_x)             = parse_spaced_cartn_x(&remainder[33..]).unwrap();
    
    println!("ID: {}, Type Symbol: {}, label_atom_id: {}", id, type_symbol, label_atom_id);
    // match parse_entry_type(input1) {
    //     Ok((remainder, entry_type)) => println!("Input1: {:?}, Remainder: {:?}", entry_type, remainder),
    //     Err(e) => println!("Error parsing input1: {:?}", e),
    // }

    // match parse_entry_type(input2) {
    //     Ok((remainder, entry_type)) => println!("Input2: {:?}, Remainder: {:?}", entry_type, remainder),
    //     Err(e) => println!("Error parsing input2: {:?}", e),
    // }
}





// loop_
// _atom_site.group_PDB 
// _atom_site.id 
// _atom_site.type_symbol 
// _atom_site.label_atom_id 
// _atom_site.label_alt_id 
// _atom_site.label_comp_id 
// _atom_site.label_asym_id 
// _atom_site.label_entity_id 
// _atom_site.label_seq_id 
// _atom_site.pdbx_PDB_ins_code 
// _atom_site.Cartn_x 
// _atom_site.Cartn_y 
// _atom_site.Cartn_z 
// _atom_site.occupancy 
// _atom_site.B_iso_or_equiv 
// _atom_site.pdbx_formal_charge 
// _atom_site.auth_seq_id 
// _atom_site.auth_comp_id 
// _atom_site.auth_asym_id 
// _atom_site.auth_atom_id 
// _atom_site.pdbx_PDB_model_num 
// ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 
// ATOM   2     C CA    . ALA A  1  2    ? 185.505 229.088 208.055 1.00 4.27   ? 1    ALA 0 CA    1 
// ATOM   3     C C     . ALA A  1  2    ? 184.672 230.300 207.814 1.00 2.40   ? 1    ALA 0 C     1 
// ATOM   4     O O     . ALA A  1  2    ? 185.159 231.267 207.269 1.00 3.02   ? 1    ALA 0 O     1 