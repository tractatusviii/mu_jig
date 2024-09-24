use mu_jig;
use pdbtbx::open;

fn main() {
    // let result = mu_jig::add(3, 4);
    // println!("3 + 4 = {}", result);
    let _ = open("~/dev/RIBETL_DATA/3J7Z/3J7Z.cif");
    ()
}