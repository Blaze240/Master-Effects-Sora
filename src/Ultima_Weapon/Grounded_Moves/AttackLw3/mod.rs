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

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_turn_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.3,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_keyblade_flare_ultima"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_trail_keyblade1_ultima"),
            Hash40::new("tex_trail_keyblade2"),
            7,
            Hash40::new("haver"),
            0,
            2,
            0,
            Hash40::new("haver"),
            0,
            13.8,
            0,
            true,
            Hash40::new("null"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.4,
            0.2,
        );
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ultima"), false, true);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_attacklw3_ultima", effect_attacklw3, Priority::Low)
        .install();
}
