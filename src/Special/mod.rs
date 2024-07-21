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

mod SpecialHi;
mod SpecialHi2Fall;
mod SpecialHi2Landing;
mod SpecialN;
mod SpecialS1;
mod SpecialS2;
mod SpecialS3;

pub fn install() {
    SpecialHi::install();
    SpecialHi2Fall::install();
    SpecialHi2Landing::install();
    SpecialN::install();
    SpecialS1::install();
    SpecialS2::install();
    SpecialS3::install();
}
