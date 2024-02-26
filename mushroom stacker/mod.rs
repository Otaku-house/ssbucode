use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash_script::*;
use smash::app::*;
use std::convert::TryInto;

pub static mut SCALE: [f32; 8] = [1.0; 8];
pub static mut SCALEF: [f32; 8] = [1.0; 8];
pub static mut SCALEE: [f32; 8] = [1.0; 8];
pub static mut SCALEL: [f32; 8] = [1.0; 8];
pub static mut STF: [f32; 8] = [0.0; 8];
pub static mut STE: [f32; 8] = [0.0; 8];
pub static mut STL: [f32; 8] = [0.0; 8];
pub static mut FF: [i32; 8] = [0; 8];


#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);

        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

         
        let kinoko_giant = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("kinoko_giant_scale"));
        //let kinoko_more_giant = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("kinoko_more_giant_scale"));
        let kinoko_small = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("kinoko_small_scale"));
        let kinoko_more_small = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("kinoko_more_small_scale"));
        let thunder_giant = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("thunder_giant_scale"));
        //let thunder_more_giant = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("thunder_more_giant_scale"));
        let thunder_small = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("thunder_small_scale"));
        let thunder_more_small = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("thunder_more_small_scale"));

        
        if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
        {
            if MotionModule::frame(fighter.module_accessor) <= 1.0 {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = 1.0;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                
            }
        } 

        if FighterUtil::is_scaling(module_accessor) == false {
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KINOKO) == true {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_STATUS) == *FIGHTER_SCALING_STATUS_BIG {
                    FighterUtil::exit_big_small(module_accessor);
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * kinoko_more_small), *FIGHTER_SHIELD_KIND_GUARD);
                    SCALE[entry_id] = SCALE[entry_id] * kinoko_more_small;
                    FighterUtil::exit_big_small(module_accessor);
                    if SCALE[entry_id] > 1.0 || SCALE[entry_id] < 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KINOKO);
                }
            }
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_THUNDER) == true {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_STATUS) == *FIGHTER_SCALING_STATUS_BIG {
                    FighterUtil::exit_big_small(module_accessor);
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * thunder_more_small), *FIGHTER_SHIELD_KIND_GUARD);
                    SCALE[entry_id] = SCALE[entry_id] * thunder_more_small;
                    FighterUtil::exit_big_small(module_accessor);
                    if SCALE[entry_id] > 1.0 || SCALE[entry_id] < 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_THUNDER);
                }
            }
            if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_KIND) == *FIGHTER_SCALING_KIND_KINOKO {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_STATUS) == *FIGHTER_SCALING_STATUS_SMALL {
                    FighterUtil::exit_big_small(module_accessor);
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * kinoko_small), *FIGHTER_SHIELD_KIND_GUARD);
                    SCALE[entry_id] = SCALE[entry_id] * kinoko_small;
                    FighterUtil::exit_big_small(module_accessor);
                    if SCALE[entry_id] > 1.0 || SCALE[entry_id] < 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KINOKO);
                }
            } 
            if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_KIND) == *FIGHTER_SCALING_KIND_THUNDER {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCALING_STATUS) == *FIGHTER_SCALING_STATUS_SMALL {
                    FighterUtil::exit_big_small(module_accessor);
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * thunder_more_small), *FIGHTER_SHIELD_KIND_GUARD);
                    SCALE[entry_id] = SCALE[entry_id] * thunder_more_small;
                    FighterUtil::exit_big_small(module_accessor);
                    if SCALE[entry_id] > 1.0 || SCALE[entry_id] < 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_THUNDER);
                }
            } 
        }
            

        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            FF[entry_id] = 0;
        }

        if status_kind == *FIGHTER_FOX_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_YOSHI_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_FALCO_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_WARIO_STATUS_KIND_FINAL_SCENE
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_WOLF_STATUS_KIND_FINAL_JUMP
        || status_kind == *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_SHULK_STATUS_KIND_FINAL_SCENE_ENTRY
        || status_kind == *FIGHTER_SHULK_STATUS_KIND_FINAL_SCENE_ATTACK
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_SCENE04
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_SCENE05
        || status_kind == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY
        || status_kind == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK
        || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_SCENE04
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_SCENE05
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_FINAL_SCENE04
        || status_kind == *FIGHTER_SIMON_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_JACK_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_JACK_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_BRAVE_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_FINAL_SCENE04
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE04
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_EDGE_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_EFLAME_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_DEMON_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_FINAL_SCENE01
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_FINAL_SCENE02
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_FINAL_SCENE03
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_FINAL_SCENE04
        {
            FF[0] = 1;
            FF[1] = 1;
            FF[2] = 1;
            FF[3] = 1;
            FF[4] = 1;
            FF[5] = 1;
            FF[6] = 1;
            FF[7] = 1;
        }

        if FighterUtil::is_scaling(module_accessor) == false {
            if FF[entry_id] == 1 {
                if SCALE[entry_id] != 1.0 {
                    smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
                }
                FF[entry_id] = 0;
            }

            if status_kind == *FIGHTER_STATUS_KIND_FINAL
            || status_kind == *FIGHTER_STATUS_KIND_LINK_FINAL_ARROW_HIT
            || status_kind == *FIGHTER_LUIGI_STATUS_KIND_FINAL_VACUUM
            || status_kind == *FIGHTER_ZELDA_STATUS_KIND_FINAL_LOOP
            || status_kind == *FIGHTER_YOSHI_STATUS_KIND_FINAL_DASH
            || status_kind == *FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_DAMAGE
            || status_kind == *FIGHTER_STATUS_KIND_KIRBY_FINAL_CAPTURE
            || status_kind == *FIGHTER_STATUS_KIND_DEDEDE_FINAL_TARGET_DAMAGE
            || status_kind == *FIGHTER_PALUTENA_STATUS_KIND_FINAL_BEAM
            || status_kind == *FIGHTER_PALUTENA_STATUS_KIND_FINAL_BEAM_START
            || status_kind == *FIGHTER_STATUS_KIND_PALUTENA_FINAL_BLACKHOLE
            || status_kind == *FIGHTER_PIKMIN_STATUS_KIND_FINAL_FALL
            || status_kind == *FIGHTER_PIKMIN_STATUS_KIND_FINAL_FLY
            || status_kind == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK
            || status_kind == *FIGHTER_LUCARIO_STATUS_KIND_FINAL_ATTACK
            || status_kind == *FIGHTER_ROBOT_STATUS_KIND_FINAL_LOOP
            || status_kind == *FIGHTER_MURABITO_STATUS_KIND_FINAL_CHEER
            || status_kind == *FIGHTER_WIIFIT_STATUS_KIND_FINAL_HOLD
            || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_FINAL_DRAW
            || status_kind == *FIGHTER_STATUS_KIND_PACKUN_FINAL_CAPTURE
            {
                if SCALE[entry_id] != 1.0 {
                    smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
                    DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
                }
            }
        }

        if status_kind == *FIGHTER_FOX_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_YOSHI_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_FALCO_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_WARIO_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_WOLF_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_SHULK_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_KAMUI_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_SIMON_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_JACK_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_BRAVE_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_EDGE_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_EFLAME_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_DEMON_STATUS_KIND_FINAL_END
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_FINAL_END
        {
            FF[0] = 1;
            FF[1] = 1;
            FF[2] = 1;
            FF[3] = 1;
            FF[4] = 1;
            FF[5] = 1;
            FF[6] = 1;
            FF[7] = 1;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

    }
}

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);

        if sv_information::is_ready_go() == true {
            if STE[entry_id] == 1.0 {
                if SCALEE[entry_id] == SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = 1.0 / shield;
                    if SCALEF[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEF[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STE[entry_id] = 0.0;
                }
                if SCALEF[entry_id] != SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = SCALEE[entry_id];
                    if SCALEF[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEF[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STE[entry_id] = 0.0;
                }
            }
            if STE[entry_id] == 0.0 {
                STF[entry_id] = 1.0;
                SCALEF[entry_id] = 1.0 / shield;
                
            }
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEF[entry_id] = 1.0;
            SCALEE[entry_id] = 1.0;
            STF[entry_id] = 0.0;
            STE[entry_id] = 0.0;
        }

    }
}

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);

        if sv_information::is_ready_go() == true {
            if STF[entry_id] == 1.0 {
                if SCALEF[entry_id] == SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = 1.0 / shield;
                    if SCALEE[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STF[entry_id] = 0.0;
                }
                if SCALEE[entry_id] != SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = SCALEF[entry_id];
                    if SCALEE[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STF[entry_id] = 0.0;
                }
            }
            if STF[entry_id] == 0.0 {
                STE[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0 / shield;
            }  
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEE[entry_id] = 1.0;
            SCALEF[entry_id] = 1.0;
            STE[entry_id] = 0.0;
            STF[entry_id] = 0.0;
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PZENIGAME )]
fn pzenigame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);

        if sv_information::is_ready_go() == true {
            if STL[entry_id] == 1.0 {
                if SCALEL[entry_id] == SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = 1.0 / shield;
                    if SCALEE[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STL[entry_id] = 0.0;
                }
                if SCALEL[entry_id] != SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEL[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = SCALEL[entry_id];
                    if SCALEE[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEE[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STL[entry_id] = 0.0;
                }
            }
            if STL[entry_id] == 0.0 {
                STE[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0 / shield;
            }
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEF[entry_id] = 1.0;
            SCALEE[entry_id] = 1.0;
            SCALEL[entry_id] = 1.0;
            STF[entry_id] = 0.0;
            STE[entry_id] = 0.0;
            STL[entry_id] = 0.0;
        }
        
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PFUSHIGISOU )]
fn pfushigisou_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);

        if sv_information::is_ready_go() == true {
            if STE[entry_id] == 1.0 {
                if SCALEE[entry_id] == SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = 1.0 / shield;
                    if SCALEF[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEF[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STE[entry_id] = 0.0;
                }
                if SCALEE[entry_id] != SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = SCALEE[entry_id];
                    if SCALEF[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEF[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STE[entry_id] = 0.0;
                }
            }
            if STE[entry_id] == 0.0 {
                STF[entry_id] = 1.0;
                SCALEF[entry_id] = 1.0 / shield;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEE[entry_id] = 1.0;
            SCALEF[entry_id] = 1.0;
            SCALEL[entry_id] = 1.0;
            STE[entry_id] = 0.0;
            STF[entry_id] = 0.0;
            STL[entry_id] = 0.0;
        }

    }
}

#[fighter_frame( agent = FIGHTER_KIND_PLIZARDON )]
fn plizardon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);

        if sv_information::is_ready_go() == true {
            if STF[entry_id] == 1.0 {
                if SCALEL[entry_id] == SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEL[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEL[entry_id] = 1.0 / shield;
                    if SCALEL[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEL[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STF[entry_id] = 0.0;
                }
                if SCALEL[entry_id] != SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEL[entry_id] = SCALEF[entry_id];
                    if SCALEL[entry_id] != 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                    }
                    if SCALEL[entry_id] == 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                    }
                    STF[entry_id] = 0.0;
                }
            }
            if STF[entry_id] == 0.0 {
                STL[entry_id] = 1.0;
                SCALEL[entry_id] = 1.0 / shield;
            }
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEF[entry_id] = 1.0;
            SCALEE[entry_id] = 1.0;
            SCALEL[entry_id] = 1.0;
            STF[entry_id] = 0.0;
            STE[entry_id] = 0.0;
            STL[entry_id] = 0.0;
        }
        
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
        
    );
    smashline::install_agent_frames!(
        eflame_frame,
        elight_frame,
        pzenigame_frame,
        pfushigisou_frame,
        plizardon_frame
    );
}