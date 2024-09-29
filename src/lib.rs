
use nom::{
    combinator::map_res,
};


const STRING: &str = "Hello, World!";

const BLOCK: &str = "loop_
_atom_site.group_PDB 
_atom_site.id 
_atom_site.type_symbol 
_atom_site.label_atom_id 
_atom_site.label_alt_id 
_atom_site.label_comp_id 
_atom_site.label_asym_id 
_atom_site.label_entity_id 
_atom_site.label_seq_id 
_atom_site.pdbx_PDB_ins_code 
_atom_site.Cartn_x 
_atom_site.Cartn_y 
_atom_site.Cartn_z 
_atom_site.occupancy 
_atom_site.B_iso_or_equiv 
_atom_site.pdbx_formal_charge 
_atom_site.auth_seq_id 
_atom_site.auth_comp_id 
_atom_site.auth_asym_id 
_atom_site.auth_atom_id 
_atom_site.pdbx_PDB_model_num 
ATOM   1     N N     . ALA A  1  2    ? 185.986 228.605 206.762 1.00 4.58   ? 1    ALA 0 N     1 
ATOM   2     C CA    . ALA A  1  2    ? 185.505 229.088 208.055 1.00 4.27   ? 1    ALA 0 CA    1 
ATOM   3     C C     . ALA A  1  2    ? 184.672 230.300 207.814 1.00 2.40   ? 1    ALA 0 C     1 
ATOM   4     O O     . ALA A  1  2    ? 185.159 231.267 207.269 1.00 3.02   ? 1    ALA 0 O     1 
ATOM   5     C CB    . ALA A  1  2    ? 186.651 229.422 208.997 1.00 4.30   ? 1    ALA 0 CB    1 
ATOM   6     N N     . VAL A  1  3    ? 183.426 230.259 208.246 1.00 1.99   ? 2    VAL 0 N     1";

pub fn count() -> i32 {
    for (index, line) in BLOCK.lines().enumerate() {
        println!("Line {}: {}", index + 1, line);
    }
    0




}