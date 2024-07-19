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

mod Ultima_Weapon_Switch;
mod Aerials;
mod Grounded_Moves;
mod Special_Limit;
mod SpecialAir_Limit;


#[skyline::main(name = "cloud_limit_switch")]
pub fn main() {
    Ultima_Weapon_Switch::install();
	Aerials::install();
	Grounded_Moves::install();
	Special_Limit::install();
	SpecialAir_Limit::install();


}