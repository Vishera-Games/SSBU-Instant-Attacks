use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_11",
    animcmd = "game_attack11")]
pub fn instant_gbowser_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(23.0)
		}
		frame(24)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(25)
		if (is_excute) {
			ATTACK(0, 0, hash40("handl"), 6.0, 80, 100, 70, 0, 10.0, 6.0, -0.100000001, 0.600000024, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 6.0, 80, 100, 70, 0, 10.0, 8.0, -0.699999988, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("shoulderl"), 6.0, 80, 100, 70, 0, 10.0, 4.0, 0.100000001, 0.0, 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
		}
		frame(29)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(31)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_12",
    animcmd = "game_attack12")]
pub fn instant_gbowser_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(15)
		if (is_excute) {
			ATTACK(0, 0, hash40("handr"), 8.0, 45, 55, 0, 70, 10.0, 6.0, -0.100000001, 0.600000024, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("armr"), 8.0, 45, 55, 0, 70, 10.0, 8.0, -0.699999988, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("shoulderr"), 8.0, 45, 55, 0, 70, 10.0, 4.0, 0.100000001, 0.0, 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
		}
		frame(18)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
		frame(31)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn instant_gbowser_attack_s3s(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(28.0)
		}
		frame(29)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(30)
		if (is_excute) {
			ATTACK(0, 0, hash40("handl"), 18.0, 361, 35, 0, 80, 15.0, 6.5, -0.200000003, 0.699999988, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 18.0, 361, 35, 0, 80, 10.0, -2.0, -0.800000012, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(34)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_s3_hi",
    animcmd = "game_attacks3hi")]
pub fn instant_gbowser_attack_s3hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(28.0)
		}
		frame(29)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(30)
		if (is_excute) {
			ATTACK(0, 0, hash40("handl"), 18.0, 361, 35, 0, 80, 15.0, 6.5, -0.200000003, 0.699999988, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 18.0, 361, 35, 0, 80, 10.0, -2.0, -0.800000012, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(34)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_s3_lw",
    animcmd = "game_attacks3lw")]
pub fn instant_gbowser_attack_s3lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(28.0)
		}
		frame(29)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(30)
		if (is_excute) {
			ATTACK(0, 0, hash40("handl"), 18.0, 361, 35, 0, 80, 15.0, 6.5, -0.200000003, 0.699999988, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 18.0, 361, 35, 0, 80, 10.0, -2.0, -0.800000012, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(34)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn instant_gbowser_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(24.0)
		}
		frame(25)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(26)
		if (is_excute) {
			ATTACK(0, 0, hash40("handl"), 16.0, 80, 60, 0, 65, 15.0, 7.0, -0.200000003, 0.699999988, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 15.0, 80, 60, 0, 65, 12.0, 8.0, -0.800000012, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("shoulderl"), 12.0, 80, 60, 0, 65, 12.0, 0.0, 0.0, 0.0, 1.0, 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(3, 0, hash40("top"), 12.0, 80, 60, 0, 65, 10.0, 0.0, 5.0, 45.0, 0.0, 20.0, 45.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(30)
		if (is_excute) {
			CancelModule::enable_cancel()
		}
		frame(34)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn instant_gbowser_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(19.0)
		}
		frame(20)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
		frame(21)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(24)
		if (is_excute) {
			MotionModule::set_rate(5.0)
		}
		frame(47)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(48)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(1, 0, hash40("head"), 18.0, 45, 35, 0, 85, 8.0, 7.0, 3.0, 9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(49)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(1, 0, hash40("head"), 18.0, 45, 35, 0, 85, 8.0, 7.0, 3.0, 5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(51)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(0, 0, hash40("head"), 28.0, 45, 25, 0, 100, 16.0, 18.0, -6.0, 10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 15, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(52)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(0, 0, hash40("head"), 28.0, 45, 25, 0, 100, 21.0, 18.0, -8.0, 10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 15, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("head"), 18.0, 45, 35, 0, 85, 8.0, -5.0, 3.0, 5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(54)
		if (is_excute) {
			CancelModule::enable_cancel()
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn instant_gbowser_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(9.0)
		}
		frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
		frame(11)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(2.0)
		}
		frame(17)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(18)
		if (is_excute) {
			ATTACK(0, 0, hash40("bust"), 18.0, 90, 35, 0, 105, 20.0, -9.0, 5.69999981, -13.5, 6.0, -3.70000005, 9.0, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_elec"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		wait(2)
		if (is_excute) {
			ATTACK(0, 0, hash40("bust"), 18.0, 90, 35, 0, 105, 22.0, -3.0, 1.79999995, -4.5, 6.0, -3.70000005, 9.0, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_elec"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		wait(2)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
		frame(33)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(33)
		if (is_excute) {
			ATTACK(0, 0, hash40("rot"), 10.0, 70, 35, 0, 100, 10.0, 0.0, -28.0, -14.0, 0.0, -28.0, 22.0, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_G, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_elec"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(34)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn instant_gbowser_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(3.0)
		}
		frame(4)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
		frame(5)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(8)
		if (is_excute) {
			MotionModule::set_rate(5.0)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(15)
		if (is_excute) {
			ATTACK(0, 0, hash40("top"), 2.0, 120, 40, 0, 35, 13.0, 0.0, 10.0, -25.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.5, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("top"), 2.0, 120, 40, 0, 35, 13.0, 0.0, 10.0, 25.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.5, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("top"), 2.0, 160, 40, 0, 35, 11.0, 0.0, 24.0, -14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.5, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(3, 0, hash40("top"), 2.0, 160, 40, 0, 35, 11.0, 0.0, 24.0, 14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.5, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(4, 0, hash40("top"), 2.0, 367, 40, 0, 35, 15.0, 0.0, 10.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.5, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
		}
		wait(2)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(31)
		if (is_excute) {
			ATTACK(0, 0, hash40("top"), 6.0, 100, 20, 0, 110, 13.0, 0.0, 10.0, -20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_ice"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("top"), 6.0, 100, 20, 0, 110, 13.0, 0.0, 10.0, 20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_ice"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("top"), 6.0, 100, 20, 0, 110, 11.0, 0.0, 24.0, -14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_ice"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(3, 0, hash40("top"), 6.0, 100, 20, 0, 110, 11.0, 0.0, 24.0, 14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_ice"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(4, 0, hash40("top"), 6.0, 100, 20, 0, 110, 15.0, 0.0, 10.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 3.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_ice"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		wait(2)
		if (is_excute) {
			CancelModule::enable_cancel()
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn instant_gbowser_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(6.0)
		}
		frame(7)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(8)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(0, 0, hash40("handl"), 12.0, 361, 80, 0, 30, 15.0, 7.0, -0.200000003, 0.699999988, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("arml"), 12.0, 361, 80, 0, 30, 12.0, 8.0, -0.800000012, -0.200000003, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(2, 0, hash40("shoulderl"), 12.0, 361, 80, 0, 30, 12.0, 0.0, 0.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_purple"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(12)
		if (is_excute) {
			AttackModule::clear_all()
			CancelModule::enable_cancel()
		}
		frame(30)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn instant_gbowser_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(8.0)
		}
		frame(9)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(10)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			DamageModule::set_no_reaction_damage_power(12.0)
			ATTACK(0, 0, hash40("waist"), 15.0, 24, 30, 0, 90, 25.0, 3.5, -3.70000005, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_B, false, 10, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_elec"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_ELEC, ATTACK_REGION_NONE)
		}
		frame(13)
		if (is_excute) {
			CancelModule::enable_cancel()
		}
		wait(10)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(34)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn instant_gbowser_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(20.0)
		}
		frame(10)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(21)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(22)
		if (is_excute) {
			ATTACK(0, 0, hash40("head"), 13.0, 85, 50, 0, 70, 14.0, 0.0, 6.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("head"), 13.0, 85, 50, 0, 70, 14.0, 0.0, -7.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_FIRE, ATTACK_REGION_NONE)
		}
		frame(25)
		if (is_excute) {
			CancelModule::enable_cancel()
		}
		wait(5)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(40)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn instant_gbowser_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(21.0)
		}
		frame(22)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
        frame(23)
		if (is_excute) {
			ATTACK(0, 0, hash40("top"), 4.0, 85, 100, 110, 0, 11.0, 0.0, 10.0, 29.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_KICK, ATTACK_REGION_NONE)
		}
		wait(2)
		if (is_excute) {
			AttackModule::clear_all()
			WorkModule::on_flag(FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1)
			CancelModule::enable_cancel()
		}
		frame(26)
		if (is_excute) {
			MotionModule::set_rate(24.0)
		}
		frame(51)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
		frame(52)
		if (is_excute) {
			ATTACK(0, 0, hash40("top"), 16.0, 76, 60, 0, 70, 20.0, 0.0, 20.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_KICK, ATTACK_REGION_NONE)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
pub fn instant_gbowser_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(30.0)
		}
		frame(31)
		if (is_excute) {
			MotionModule::set_rate(1.0)
		}
		frame(32)
		if (is_excute) {
			CancelModule::enable_cancel()
			ATTACK(0, 0, hash40("top"), 18.0, 76, 30, 0, 110, 20.0, 0.0, 10.0, 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_normal"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_KICK, ATTACK_REGION_NONE)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPAG, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn instant_gbowser_special_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(1)
		if (is_excute) {
			KineticModule::unable_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
			KineticModule::clear_speed_all()
		}
		FT_MOTION_RATE(8)
		if (is_excute) {
			FighterAreaModuleImpl::enable_fix_jostle_area(20.0, 20.0)
		}
		frame(2)
		FT_MOTION_RATE(20)
		frame(3)
		FT_MOTION_RATE(1.20000005)
		if (is_excute) {
			KineticModule::enable_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
		}
		frame(6)
		if (is_excute) {
			WorkModule::on_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4)
			ATTACK(0, 0, hash40("top"), 2.0, 180, 100, 50, 0, 14.0, 0.0, 14.0, -8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.699999988, 1.5, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			ATTACK(1, 0, hash40("top"), 2.0, 180, 100, 50, 0, 14.0, 0.0, 14.0, 12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 0.699999988, 1.5, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 3, 0.0, 5, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
		}
		wait(1)
		if (is_excute) {
			WorkModule::on_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3)
		}
		if (is_excute) {
			HIT_NODE(hash40("hip"), HIT_STATUS_XLU)
			HIT_NODE(hash40("mouth1"), HIT_STATUS_XLU)
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
			HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
			HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
			HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("toer"), HIT_STATUS_XLU)
			HIT_NODE(hash40("toel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("tail3"), HIT_STATUS_XLU)
		}
		if (is_excute) {
			KineticModule::set_consider_ground_friction(false, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
		}
		wait(1)
		if (is_excute) {
			WorkModule::on_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG2)
		}
		if (is_excute) {
			HIT_NODE(hash40("hip"), HIT_STATUS_XLU)
			HIT_NODE(hash40("mouth1"), HIT_STATUS_XLU)
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
			HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
			HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
			HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("toer"), HIT_STATUS_XLU)
			HIT_NODE(hash40("toel"), HIT_STATUS_XLU)
			HIT_NODE(hash40("tail3"), HIT_STATUS_XLU)
		}
		for(4 Iterations){
			if (is_excute) {
				ATTACK(0, 0, hash40("top"), 10.0, 80, 65, 0, 80, 16.0, 0.0, 14.0, -10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.5, 1.5, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
				ATTACK(1, 0, hash40("top"), 10.0, 80, 65, 0, 80, 16.0, 0.0, 14.0, 14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, 1.5, 1.5, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_cutup"), ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_CUTUP, ATTACK_REGION_NONE)
			}
			wait(4)
		}
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(51)
		if (is_excute) {
			CancelModule::enable_cancel()
			WorkModule::off_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4)
			WorkModule::off_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3)
			HitModule::set_status_all(app::HitStatus(*HIT_STATUS_NORMAL), 0)
			KineticModule::set_consider_ground_friction(true, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			WorkModule::on_flag(FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1)
		}
		FT_MOTION_RATE(2)
    });
}

pub fn install() {
    acmd::add_hooks!(
        instant_gbowser_attack_11,
        instant_gbowser_attack_12,
        instant_gbowser_attack_s3s,
        instant_gbowser_attack_s3hi,
        instant_gbowser_attack_s3lw,
        instant_gbowser_attack_hi3,
        instant_gbowser_attack_s4,
        instant_gbowser_attack_hi4,
        instant_gbowser_attack_lw4,
        instant_gbowser_attack_air_f,
        instant_gbowser_attack_air_b,
        instant_gbowser_attack_air_hi,
        instant_gbowser_special_lw,
		instant_gbowser_special_air_lw,
		instant_gbowser_special_hi
    );
}