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

mod SpecialHiLB;
mod SpecialLw;
mod SpecialNLB;
mod SpecialS1LB;
mod SpecialS2LB;
mod SpecialS3LB;

pub fn install() {
    SpecialHiLB::install();
    SpecialLw::install();
    SpecialNLB::install();
    SpecialS1LB::install();
    SpecialS2LB::install();
    SpecialS3LB::install();
}
