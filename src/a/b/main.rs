pub struct B {
    pub field1: i32,
    pub field2: i32,
}
// the import below is only there to demonstrate how importing from a module in
// another directory works. It is not used.
// The first "super" refers to module "b" that is described in the file "../b.rs". This
// "super" can be thought of as referring to directory "../b".
// The second "super" refers to module "a" that is described in the file "../../a.rs".
// This second "super" can be thought of as referring to directory "../../a"
#[allow(unused_imports)]
use super::super::c::main::C;
