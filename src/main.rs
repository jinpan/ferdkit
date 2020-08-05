extern crate libc;

#[link(name="wrapper", kind="static")]
extern {
    fn in_orgo_subset(input: libc::c_int) -> bool;
}

fn main() {
  for i in 1..10 {
    println!("inOrganicSubset({}): {}", i, unsafe {
        in_orgo_subset(i)
    });
  }
}