mod a;
use a::b::main::B;
use a::c::main::C;

fn main() {
    #[allow(non_snake_case)]
    let test_B = B {
        field1: 1,
        field2: 2,
    };
    #[allow(non_snake_case)]
    let test_C = C {
        field1: 3,
        field2: 4,
    };
    println!(
        "test_B.field1 = {}\ntest_B.field2 = {}\ntestC.field1 = {}\ntestC.field2 = {}",
        test_B.field1, test_B.field2, test_C.field1, test_C.field2
    );
}
