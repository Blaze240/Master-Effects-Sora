#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod Aerials;
mod Grounded_Moves;


#[skyline::main(name = "cloud_limit_effects")]
pub fn main() {
	Aerials::install();
	Grounded_Moves::install();

}