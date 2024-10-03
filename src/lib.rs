use nom::{
    bytes::complete::tag, character::complete::{alphanumeric1, digit1, space1}, combinator::{map_res, opt}, multi::count, number::complete::float, sequence::{preceded, tuple}, IResult
};

#[derive(Debug)]
struct AtomSite {
    group_pdb         : String,
    id                : u32,
    type_symbol       : String,
    label_atom_id     : String,
    label_alt_id      : String,
    label_comp_id     : String,
    label_asym_id     : String,
    label_entity_id   : String,
    label_seq_id      : Option<u32>,
    pdbx_pdb_ins_code : String,
    cartn_x           : f32,
    cartn_y           : f32,
    cartn_z           : f32,
    occupancy         : f32,
    b_iso_or_equiv    : f32,
    pdbx_formal_charge: String,
    auth_seq_id       : u32,
    auth_comp_id      : String,
    auth_asym_id      : String,
    auth_atom_id      : String,
    pdbx_pdb_model_num: u32,
}

fn parse_group_pdb(input: &str) -> IResult<&str, String> {
    map_res(alphanumeric1, |s: &str| Ok::<_, ()>(s.to_string()))(input)
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_string(input: &str) -> IResult<&str, String> {
    map_res(alphanumeric1, |s: &str| Ok::<_, ()>(s.to_string()))(input)
}

fn parse_optional_u32(input: &str) -> IResult<&str, Option<u32>> {
    opt(map_res(digit1, |s: &str| s.parse::<u32>()))(input)
}

// fn parse_spaced_item<F, O>(mut parser: F) -> impl FnMut(&str) -> IResult<&str, O>
// where
//     F: FnMut(&str) -> IResult<&str, O>,
// {
//     move |input: &str| { preceded(space1, &mut parser)(input) }
// }


// fn parse_atom_site(input: &str) -> IResult<&str, AtomSite> {
//     let (input, group_pdb)                   = parse_group_pdb(input)?;
//     let (input, id)                          = pa    // match parse_atom_site(input) {
    //     Ok((_, atom_site)) => println!("Parsed AtomSite: {:#?}", atom_site),
    //     Err(e) => println!("Error parsing input: {:?}", e),
    // }rse_spaced_item(parse_id)(input)?;
//     let (input, type_symbol)                 = parse_spaced_item(parse_string)(input)?;
//     let (input, label_atom_id)               = parse_spaced_item(parse_string)(input)?;
//     let (input, label_alt_id)                = parse_spaced_item(parse_string)(input)?;
//     let (input, label_comp_id)               = parse_spaced_item(parse_string)(input)?;
//     let (input, label_asym_id)               = parse_spaced_item(parse_string)(input)?;
//     let (input, label_entity_id)             = parse_spaced_item(parse_string)(input)?;
//     let (input, label_seq_id)                = parse_spaced_item(parse_optional_u32)(input)?;
//     let (input, pdbx_pdb_ins_code)           = parse_spaced_item(parse_string)(input)?;
//     let (input, (cartn_x, cartn_y, cartn_z)) = parse_spaced_item(tuple(( float, parse_spaced_item(float), parse_spaced_item(float), )))(input)?;
//     let (input, occupancy)                   = parse_spaced_item(float)(input)?;
//     let (input, b_iso_or_equiv)              = parse_spaced_item(float)(input)?;
//     let (input, pdbx_formal_charge)          = parse_spaced_item(parse_string)(input)?;
//     let (input, auth_seq_id)                 = parse_spaced_item(parse_id)(input)?;
//     let (input, auth_comp_id)                = parse_spaced_item(parse_string)(input)?;
//     let (input, auth_asym_id)                = parse_spaced_item(parse_string)(input)?;
//     let (input, auth_atom_id)                = parse_spaced_item(parse_string)(input)?;
//     let (input, pdbx_pdb_model_num)          = parse_spaced_item(parse_id)(input)?;

//     Ok((input, AtomSite {
//         group_pdb,
//         id,
//         type_symbol,
//         label_atom_id,
//         label_alt_id,
//         label_comp_id,
//         label_asym_id,
//         label_entity_id,
//         label_seq_id,
//         pdbx_pdb_ins_code,
//         cartn_x,
//         cartn_y,
//         cartn_z,
//         occupancy,
//         b_iso_or_equiv,
//         pdbx_formal_charge,
//         auth_seq_id,
//         auth_comp_id,
//         auth_asym_id,
//         auth_atom_id,
//         pdbx_pdb_model_num,
//     }))
// }


fn parse_input(input: &str) -> IResult<&str, &str> {
    //  note that this is really creating a function, the parser for abc
    //  vvvvv 
    //         which is then called here, returning an IResult<&str, &str>
    //         vvvvv
    tag("ATOM")(input)
}

fn main() {

    let input = "ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 ";


    match parse_input(input){
        Err(e) => println!("Error parsing input: {:?}", e),
        Ok((_, _)) => println!("Parsed input"),
    };
    ()
    // match parse_atom_site(input) {
    //     Ok((_, atom_site)) => println!("Parsed AtomSite: {:#?}", atom_site),
    //     Err(e) => println!("Error parsing input: {:?}", e),
    // }
}

