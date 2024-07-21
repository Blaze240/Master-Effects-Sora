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

unsafe extern "C" fn effect_specials3_lb_limit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke2_l_lb"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke2_r_lb"), true, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_cloud_sword1_blue"),
            Hash40::new("tex_cloud_sword2"),
            4,
            Hash40::new("haver"),
            0,
            1.5,
            -1.2,
            Hash40::new("haver"),
            0,
            20.5,
            -1.2,
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
            0.1,
        );
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke3_l_lb"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke4_l_lb"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke4_l_lb"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke3_r_lb"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                -100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke4_r_lb"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                -100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.3);
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_cloud_sword1_blue"),
            Hash40::new("tex_cloud_sword2"),
            6,
            Hash40::new("haver"),
            0,
            1.5,
            -1.2,
            Hash40::new("haver"),
            0,
            20.5,
            -1.2,
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
            0.1,
        );
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
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
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("cloud_kyogiri_vanish_lb"),
            Hash40::new("top"),
            0,
            10,
            17,
            0,
            0,
            0,
            1.1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(
            agent,
            Hash40::new("cloud_kyogiri_stroke3_l_lb"),
            false,
            false,
        );
        macros::EFFECT_OFF_KIND(
            agent,
            Hash40::new("cloud_kyogiri_stroke3_r_lb"),
            false,
            false,
        );
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}
pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_specials3_lb_limit",
            effect_specials3_lb_limit,
            Priority::Low,
        )
        .install();
}
