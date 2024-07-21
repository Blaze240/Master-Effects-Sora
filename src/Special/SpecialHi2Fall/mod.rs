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
unsafe extern "C" fn effect_specialhi2fall_fusion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("cloud_climhazzard_slash2"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd("effect_specialhi2fall_fusion", effect_specialhi2fall_fusion, Priority::Low)
        .install();
}
