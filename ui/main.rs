mod flecs {
    pub struct With;
}

fn some_generic_function<T>() {}

fn main() {
    // This should trigger the lint warning
    let tester = some_generic_function::<flecs::With>();
}
