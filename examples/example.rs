use zestors_codegen::{protocol, Msg};

fn main() {}

#[derive(Msg)]
#[msg(())]
struct TestMsg;

#[protocol]
enum MyProt {
    One(u32),
    Two(TestMsg),
}
