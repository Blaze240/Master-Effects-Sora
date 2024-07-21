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

mod Fusion_Sword_Switch;
mod Aerials;
mod Grounded_Moves;
mod SpecialAir;
mod Special;



#[skyline::main(name = "cloud_fusion_switch")]
pub fn main() {
    Fusion_Sword_Switch::install();
	Aerials::install();
	Grounded_Moves::install();
	Special::install();
	SpecialAir::install();
}