use std::fmt::{write, Error};

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum EntryType {
    MOLECULE,
    ATOM,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum SpecialCharacter {
    QuestionMark,
    Dot,
}


impl std::fmt::Display for SpecialCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            SpecialCharacter::QuestionMark => write!(f, "?"),
            SpecialCharacter::Dot => write!(f, "."),
        }
    }
}

impl SpecialCharacter{
    fn parse_special_character(input: &str)->IResult<&str,SpecialCharacter>{
        alt((
            map(tag("?"), |_| SpecialCharacter::QuestionMark),
            map(tag("."), |_| SpecialCharacter::Dot),
        ))(input)
    }

}




impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.as_str())
    }
}

impl EntryType {
    const MOLECULE_STR: &'static str = "MOLECULE";
    const ATOM_STR: &'static str = "ATOM";

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

fn parse_id(input: &str) -> IResult<&str, u32> {
    preceded(space1, map_res(digit1, str::parse))(input)
}

fn parse_label_atom_id(input: &str) -> IResult<&str, String> {
    preceded(space1, map_res(alphanumeric1, str::parse))(input)
}

fn parse_type_symbol(input: &str) -> IResult<&str, String> {
    preceded( space1, map_res(alphanumeric1, |s: &str| Ok::<String, Error>(s.to_string())), )(input)
}
fn parse_label_alt_id(input: &str) -> IResult<&str,SpecialCharacter > {
    preceded(space1,SpecialCharacter::parse_special_character)(input)
}
fn parse_label_comp_id(input: &str) -> IResult<&str,String> {
    preceded(space1,map_res(alphanumeric1, str::parse))(input)
}
fn parse_label_asym_id(input: &str) -> IResult<&str,String> {
    preceded(space1,map_res(alphanumeric1, str::parse))(input)
}
fn parse_label_entity_id(input: &str) -> IResult<&str,String> {
    preceded(space1,map_res(digit1, str::parse))(input)
}
fn parse_label_seq_id(input: &str) -> IResult<&str,String> {
    preceded(space1,map_res(digit1, str::parse))(input)
}

fn main() {
    let input1 = "ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    let input2 = "ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    let input  = "ATOM   1     N H     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";
    // -----------1-------2---3--4-----5-6---7--8--9----10---11----12-------13-----14---15----17-18--19-20-21----22
    // -------------------------------------------------^

    let (remainder, entry_type)    = parse_entry_type(input).unwrap();
    let (remainder, id)            = parse_id(remainder).unwrap();
    let (remainder, type_symbol)   = parse_type_symbol(remainder).unwrap();
    let (remainder, label_atom_id) = parse_label_atom_id(remainder).unwrap();
    let (remainder, label_alt_id)       = parse_label_alt_id(remainder).unwrap();
    let (remainder, label_comp_id)      = parse_label_comp_id(remainder).unwrap();
    let (remainder, label_asym_id)      = parse_label_asym_id(remainder).unwrap();
    let (remainder, label_entity_id)    = parse_label_entity_id(remainder).unwrap();
    let (remainder, label_seq_id)       = parse_label_seq_id(remainder).unwrap();
    //TODO
    // let (remainder, pdbx_PDB_ins_code)  = parse_pdbx_PDB_ins_code(remainder).unwrap();
    // let (remainder, Cartn_x)            = parse_Cartn_x(remainder).unwrap();
    // let (remainder, Cartn_y)            = parse_Cartn_y(remainder).unwrap();
    // let (remainder, Cartn_z)            = parse_Cartn_z(remainder).unwrap();
    // let (remainder, occupancy)          = parse_occupancy(remainder).unwrap();
    // let (remainder, B_iso_or_equiv)     = parse_B_iso_or_equiv(remainder).unwrap();
    // let (remainder, pdbx_formal_charge) = parse_pdbx_formal_charge(remainder).unwrap();
    // let (remainder, auth_seq_id)        = parse_auth_seq_id(remainder).unwrap();
    // let (remainder, auth_comp_id)       = parse_auth_comp_id(remainder).unwrap();
    // let (remainder, auth_asym_id)       = parse_auth_asym_id(remainder).unwrap();
    // let (remainder, auth_atom_id)       = parse_auth_atom_id(remainder).unwrap();
    // let (remainder, pdbx_PDB_model_num) = parse_pdbx_PDB_model_num(remainder).unwrap();

    println!(

        "EntryTyp:{},ID: {}, Type Symbol: {}, label_atom_id: {}",
        entry_type, id, type_symbol, label_atom_id
    );
    println!(
        "Special char(label_alt_id):{}",
        label_alt_id
    );
}

// loop_
// 1_atom_site.group_PDB
// 2_atom_site.id
// 3_atom_site. type_symbol
// 4_atom_site. label_atom_id
// 5_atom_site. label_alt_id
// 6_atom_site. label_comp_id
// 7_atom_site. label_asym_id
// 8_atom_site. label_entity_id
// 9_atom_site. label_seq_id
// 10_atom_site.pdbx_PDB_ins_code
// 11_atom_site.Cartn_x
// 12_atom_site.Cartn_y
// 13_atom_site.Cartn_z
// 14_atom_site.occupancy
// 15_atom_site.B_iso_or_equiv
// 16_atom_site.pdbx_formal_charge
// 17_atom_site.auth_seq_id
// 18_atom_site.auth_comp_id
// 19_atom_site.auth_asym_id
// 20_atom_site.auth_atom_id
// 21_atom_site.pdbx_PDB_model_num
// ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1
// ATOM   2     C CA    . ALA A  1  2    ? 185.505 229.088 208.055 1.00 4.27   ? 1    ALA 0 CA    1
// ATOM   3     C C     . ALA A  1  2    ? 184.672 230.300 207.814 1.00 2.40   ? 1    ALA 0 C     1
// ATOM   4     O O     . ALA A  1  2    ? 185.159 231.267 207.269 1.00 3.02   ? 1    ALA 0 O     1
