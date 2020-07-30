extern crate libc;

#[link(name="wrapper", kind="dylib")]
extern {
    fn in_orgo_subset(input: libc::c_int) -> bool;
}

fn main() {
  for i in 0..10 {
    println!("in_orgo_subset({}): {}", i, unsafe {
        in_orgo_subset(i)
    });
  }
}