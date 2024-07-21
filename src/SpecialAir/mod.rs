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

mod SpecialAirHi;
mod SpecialAirN;
mod SpecialAirS1;
mod SpecialAirS2;
mod SpecialAirS3;

pub fn install() {
    SpecialAirHi::install();
    SpecialAirN::install();
    SpecialAirS1::install();
    SpecialAirS2::install();
    SpecialAirS3::install();
}
