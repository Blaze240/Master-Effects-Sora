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
unsafe extern "C" fn effect_specialairn_lb_limit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("cloud_hakogeki_flash"),
            Hash40::new("haver"),
            1,
            10,
            0,
            0,
            0,
            0,
            1.2,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_specialairn_lb_limit",
            effect_specialairn_lb_limit,
            Priority::Low,
        )
        .install();
}
