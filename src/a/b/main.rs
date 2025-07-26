pub struct B {
    pub field1: i32,
    pub field2: i32,
}
// the import below is only there to demonstrate how importing from a module in
// another directory works. It is not used.
#[allow(unused_imports)]
use super::super::c::main::C;
