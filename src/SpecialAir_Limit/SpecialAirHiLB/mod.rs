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
unsafe extern "C" fn effect_specialairhi_lb_limit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, 1, 0.965, 0.376);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.23, 0.413, 2);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("cloud_climhazzard_sting_lb"), Hash40::new("haver"), 0, 3, 0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("cloud_climhazzard_slash_lb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_climhazzard_sting_lb"), false, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_specialairhi_lb",
            effect_specialairhi_lb_limit,
            Priority::Low,
        )
        .install();
}
