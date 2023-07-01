use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

pub static mut SCALE: [f32; 8] = [1.0; 8];
static mut _RED: [f32; 8] = [4.0; 8];
static mut _BLUE: [f32; 8] = [4.0; 8];
static mut _GREEN: [f32; 8] = [0.0; 8];

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let scale2 = scale * 0.99641;

        if sv_information::is_ready_go() == true {
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCHED_REFLET {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    //smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG); 
                    ModelModule::set_scale(module_accessor, scale2);
                }
            }
        }
        if sv_information::is_ready_go() == false {
            SCALE[entry_id] = 1.0;
        }
        /*
        if scale < 1.0 {
            DamageModule::set_damage_mul(module_accessor, scale);
            AttackModule::set_power_up(module_accessor, 1.0 / scale);
            GrabModule::set_scale_2nd(module_accessor, scale);
            AttackModule::set_attack_scale(module_accessor, scale, true);
        }*/
        
    }
}

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        
        if sv_information::is_ready_go() == true {
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    SCALE[entry_id] = SCALE[entry_id] * 1.0028;
                    smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG); 
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if SCALE[entry_id] >= 1.0 {
                        smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG); 
                    }
                }
            }
        }
        if sv_information::is_ready_go() == false {
            SCALE[entry_id] = 1.0;
        }

        if SCALE[entry_id] > 1.0 {
            DamageModule::set_damage_mul(module_accessor, 0.9 / SCALE[entry_id]);
            AttackModule::set_power_up(module_accessor, SCALE[entry_id] * 1.3);
            ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
            if SCALE[entry_id] >= 1.23 {
                damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
                GrabModule::set_scale_2nd(module_accessor, 1.4);
            }
        }


        if SCALE[entry_id] >= 1.0
        && SCALE[entry_id] < 1.5
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_001")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03_chrom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win_lucina")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackbomb")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N11")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N12")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_mp_empty")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_throw")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_close")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_landing")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win1")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win2_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win3_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final12")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final14")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final15")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final17")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_sword")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_fire")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_elec")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_fire_02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win03")}, 1.0);   
        }
        if SCALE[entry_id] >= 1.5
        && SCALE[entry_id] < 50.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 0.9);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack05")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack06")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack07")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_jump")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_ottotto")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage_twinkle")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_swimup")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furafura")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furasleep")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_wakeup")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_heavyget")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_passive")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_cliffcatch")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_001")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_S01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03_chrom")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win_lucina")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom05")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom06")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom09")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_knockout")}, 0.9);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_start")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_stop")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_turn")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_squat")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_rise")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escape")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escapeair")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_s")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_m")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_l")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackbomb")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100end")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackdash")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_L01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N06")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N10")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N11")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N12")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_mp_empty")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_throw")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_close")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_landing")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_h01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appear01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win1")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win2_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win3_01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final03")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final04")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final09")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final10")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final12")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final14")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final15")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final17")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_sword")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_fire")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_elec")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02_win01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_fire_02_win01")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win02")}, 0.9);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win03")}, 0.9); 
        }

    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
    smashline::install_agent_frames!(
        reflet_frame
    );
}
