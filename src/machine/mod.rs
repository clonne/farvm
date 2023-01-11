
#[derive(Debug,Default)]
pub struct Object {
    stack:Vec<u8>,
}

pub fn make() -> Object {
    Object::default()
}

pub fn run(vm:&Object) {
}
