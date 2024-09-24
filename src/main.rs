use mu_jig;
use pdbtbx;


fn main() {
    use pdbtbx::*;
    let (mut pdb, _errors) = pdbtbx::open( "../pdbtbx/example-pdbs/1ubq.pdb").unwrap();

    pdb.remove_atoms_by(|atom| atom.element() == Some(&Element::H)); // Remove all H atoms

    let mut avg_b_factor = 0.0;
    for atom in pdb.atoms() { // Iterate over all atoms in the structure
        avg_b_factor += atom.b_factor();
    }
    avg_b_factor /= pdb.atom_count() as f64;

    println!("The average B factor of the protein is: {}", avg_b_factor);
    pdbtbx::save(&pdb, "dump/1ubq_no_hydrogens.pdb", pdbtbx::StrictnessLevel::Loose);
}