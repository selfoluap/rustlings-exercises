// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod fruits {
    pub const PEAR: &'static str = "Banana";
    pub const APPLE: &'static str = "Apple";
}

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use crate::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
