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

unsafe extern "C" fn ultima_weapon_switch(agent: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("bastar_sword_r"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("ultima_weapon_r"),
                true,
            );
        } else if MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_start")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_air_start")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_dash")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_dash_end")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_air_dash_end")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_hit")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_air_hit")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_move")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_attack")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_fall")
            || MotionModule::motion_kind(agent.module_accessor) == smash::hash40("final_end")
        {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("bastar_sword_r"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("ultima_weapon_r"),
                true,
            );
        } else {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("bastar_sword_r"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("ultima_weapon_r"),
                false,
            );
        }
    }
}

pub fn install() {
    Agent::new("cloud")
        .on_line(Main, ultima_weapon_switch)
        .install();
}
