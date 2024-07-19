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

mod SpecialAirHiLB;
mod SpecialAirLw;
mod SpecialAirNLB;
mod SpecialAirS1LB;
mod SpecialAirS2LB;
mod SpecialAirS3LB;

pub fn install() {
    SpecialAirHiLB::install();
    SpecialAirLw::install();
    SpecialAirNLB::install();
    SpecialAirS1LB::install();
    SpecialAirS2LB::install();
    SpecialAirS3LB::install();
}
