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

mod Attack13;
mod AttackHi3;
mod AttackDash;
mod AttackHi4;
mod AttackLw4;
mod AttackS3;
mod AttackS4;

pub fn install() {
    Attack13::install();
    AttackS3::install();
    AttackHi3::install();
    AttackS4::install();
    AttackHi4::install();
    AttackLw4::install();
    AttackDash::install();
}
