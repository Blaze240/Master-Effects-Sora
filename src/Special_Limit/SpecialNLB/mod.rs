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
unsafe extern "C" fn effect_specialn_lb_limit(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("cloud_hakogeki_scrape"),
            Hash40::new("top"),
            -1.6,
            0,
            -10,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_action_smoke_h"),
            Hash40::new("top"),
            5,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_specialn_lb_limit",
            effect_specialn_lb_limit,
            Priority::Low,
        )
        .install();
}
