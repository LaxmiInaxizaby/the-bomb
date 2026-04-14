use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

unsafe extern "C" fn game_smashattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 100.0, 362, 100, 0, 100, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 50.0, 362, 100, 0, 100, 17.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 25.0, 362, 100, 0, 100, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
	frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(
        agent.module_accessor, 
        *FIGHTER_STATUS_KIND_DEAD, 
        false
    );
    }
}

unsafe extern "C" fn game_normalattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 100.0, 362, 100, 0, 100, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 50.0, 362, 100, 0, 100, 17.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 25.0, 362, 100, 0, 100, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
	frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(
        agent.module_accessor, 
        *FIGHTER_STATUS_KIND_DEAD, 
        false
    );
    }
}

unsafe extern "C" fn game_aerialattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 100.0, 362, 100, 0, 100, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 50.0, 362, 100, 0, 100, 17.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 25.0, 362, 100, 0, 100, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
	frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(
        agent.module_accessor, 
        *FIGHTER_STATUS_KIND_DEAD, 
        false
    );
    }
}

unsafe extern "C" fn game_landingattack(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(
        agent.module_accessor, 
        *FIGHTER_STATUS_KIND_DEAD, 
        false
    );
    }
}

unsafe extern "C" fn effect_normalattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_smashattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_normalattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_purin_rnd_attack_smash_s"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

unsafe extern "C" fn sound_smashattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_purin_rnd_attack_smash_s"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

unsafe extern "C" fn sound_win(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_purin_001"));
	}
}

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 361, 60, 0, 60, 2.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 24.0, 361, 83, 0, 57, 5.5, 0.0, 2.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 18.0, 361, 83, 0, 8, 4.5, 0.0, 2.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 26.0, 270, 45, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 23.0, 270, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 270, 130, 0, 10, 6.0, 0.0, 4.0, 3.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 3, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.1);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 100.0, 362, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 100.0, 362, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 100.0, 362, 100, 0, 100, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 13, 2);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 2.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
	frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request(
        agent.module_accessor, 
        *FIGHTER_STATUS_KIND_DEAD, 
        false
    );
    }
}

pub fn install() {
    Agent::new("purin")
	.set_costume([24, 25, 26, 27, 28, 29, 30, 31].to_vec())
		.game_acmd("game_attack11", game_normalattack, Priority::Default)
		.game_acmd("game_attack12", game_normalattack, Priority::Default)
		.game_acmd("game_attacklw3", game_normalattack, Priority::Default)
		.game_acmd("game_attacks3lw", game_normalattack, Priority::Default)
		.game_acmd("game_attacks3", game_normalattack, Priority::Default)
		.game_acmd("game_attacks3hi", game_normalattack, Priority::Default)
		.game_acmd("game_attackhi3", game_normalattack, Priority::Default)
		.game_acmd("game_attacklw4", game_smashattack, Priority::Default)
		.game_acmd("game_attacks4", game_smashattack, Priority::Default)
		.game_acmd("game_attackhi4", game_smashattack, Priority::Default)
		.game_acmd("game_attackairn", game_aerialattack, Priority::Default)
		.game_acmd("game_attackairlw", game_aerialattack, Priority::Default)
		.game_acmd("game_attackairf", game_aerialattack, Priority::Default)
		.game_acmd("game_attackairb", game_aerialattack, Priority::Default)
		.game_acmd("game_attackairhi", game_aerialattack, Priority::Default)
		.game_acmd("game_landingairn", game_landingattack, Priority::Default)
		.game_acmd("game_landingairf", game_landingattack, Priority::Default)
		.game_acmd("game_landingairhi", game_landingattack, Priority::Default)
		.game_acmd("game_landingairlw", game_landingattack, Priority::Default)
		.game_acmd("game_landingairb", game_landingattack, Priority::Default)
		.game_acmd("game_specials", game_normalattack, Priority::Default)
		.game_acmd("game_specialairs", game_normalattack, Priority::Default)
		.game_acmd("game_specialhir", game_normalattack, Priority::Default)
		.game_acmd("game_specialairhir", game_normalattack, Priority::Default)
		.game_acmd("game_specialhil", game_normalattack, Priority::Default)
		.game_acmd("game_specialairhil", game_normalattack, Priority::Default)
		.game_acmd("game_speciallwl", game_normalattack, Priority::Default)
		.game_acmd("game_speciallwr", game_normalattack, Priority::Default)
		.game_acmd("game_specialairlwr", game_normalattack, Priority::Default)
		.game_acmd("game_specialairlwl", game_normalattack, Priority::Default)
		.game_acmd("game_specialairn", game_specialairn, Priority::Default)
		.game_acmd("game_specialn", game_specialn, Priority::Default)
		.game_acmd("game_attackdash", game_attackdash, Priority::Default)
		.game_acmd("game_throwlw", game_throwlw, Priority::Default)
		.game_acmd("game_throwf", game_throwf, Priority::Default)
		//
		.effect_acmd("effect_attack11", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attack12", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attacklw3", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attacks3lw", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attacks3", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attacks3hi", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attackhi3", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attacklw4", effect_smashattack, Priority::Default)
		.effect_acmd("effect_attacks4", effect_smashattack, Priority::Default)
		.effect_acmd("effect_attackhi4", effect_smashattack, Priority::Default)
		.effect_acmd("effect_attackairn", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attackairlw", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attackairf", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attackairb", effect_normalattack, Priority::Default)
		.effect_acmd("effect_attackairhi", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specials", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialairs", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialhir", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialairhir", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialhil", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialairhil", effect_normalattack, Priority::Default)
		.effect_acmd("effect_speciallwl", effect_normalattack, Priority::Default)
		.effect_acmd("effect_speciallwr", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialairlwr", effect_normalattack, Priority::Default)
		.effect_acmd("effect_specialairlwl", effect_normalattack, Priority::Default)
		//
		.sound_acmd("sound_attack11", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attack12", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attacklw3", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attacks3lw", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attacks3", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attacks3hi", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attackhi3", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attacklw4", sound_smashattack, Priority::Default)
		.sound_acmd("sound_attacks4", sound_smashattack, Priority::Default)
		.sound_acmd("sound_attackhi4", sound_smashattack, Priority::Default)
		.sound_acmd("sound_attackairn", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attackairlw", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attackairf", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attackairb", sound_normalattack, Priority::Default)
		.sound_acmd("sound_attackairhi", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specials", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialairs", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialhir", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialairhir", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialhil", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialairhil", sound_normalattack, Priority::Default)
		.sound_acmd("sound_speciallwl", sound_normalattack, Priority::Default)
		.sound_acmd("sound_speciallwr", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialairlwr", sound_normalattack, Priority::Default)
		.sound_acmd("sound_specialairlwl", sound_normalattack, Priority::Default)
		.sound_acmd("sound_win1", sound_win, Priority::Default)
		.sound_acmd("sound_win2", sound_win, Priority::Default)
		.sound_acmd("sound_win3", sound_win, Priority::Default)
        .install();
}