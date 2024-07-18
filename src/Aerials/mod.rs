use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};

mod AttackAirN;
mod AttackAirF;
mod AttackAirB;
mod AttackAirHi;
mod AttackAirLw;

pub fn install() {
    AttackAirN::install();
    AttackAirLw::install();
    AttackAirB::install();
    AttackAirHi::install();
    AttackAirF::install();
}
