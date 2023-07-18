use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;



#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn donkey_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LINK )]
fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SAMUS )]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}


#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_YOSHI )]
fn yoshi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn fox_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PIKACHU )]
fn pikachu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_NESS )]
fn ness_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }        
    }
}

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PURIN )]
fn purin_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn koopa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ICE_CLIMBER )]
fn ice_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SHEIK )]
fn sheik_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MARIOD )]
fn mariod_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MARTH )]
fn marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_YOUNGLINK )]
fn young_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ROY )]
fn roy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_CHROM )]
fn chrom_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_GAMEWATCH )]
fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn meta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PIT )]
fn pit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
fn pitb_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_IKE )]
fn ike_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PZENIGAME )]
fn zenigame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PFUSHIGISOU )]
fn fushigisou_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PLIZARDON )]
fn lizardon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
fn diddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
fn lucas_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
fn sonic_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DEDEDE )]
fn dedede_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PIKMIN )]
fn pikmin_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn lucario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ROBOT )]
fn robot_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_WOLF )]
fn wolf_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MURABITO )]
fn murabito_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ROCKMAN )]
fn rockman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}
/*
#[fighter_frame( agent = FIGHTER_KIND_WIIFIT )]
fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.0);
    }
}
*/

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn mii_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PACMAN )]
fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SHULK )]
fn shulk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KOOPAJR )]
fn koopajr_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DUCKHUNT )]
fn duckhunt_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
fn ken_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_CLOUD )]
fn cloud_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
fn simon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KROOL )]
fn krool_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PACKUN )]
fn packun_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BRAVE )]
fn brave_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PICKEL )]
fn pickel_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn demon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_TRAIL )]
fn trail_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);
        let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        let lua_state = fighter.lua_state_agent;

        let scale2 = 0.5 * scale;
        
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_DEAD {
            ModelModule::set_scale(module_accessor, scale2);
            GrabModule::set_scale_2nd(module_accessor, scale2);
            AttackModule::set_attack_scale(module_accessor, scale2, true);
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        littlemac_frame,
        mario_frame,
        donkey_frame,
        link_frame,
        samus_frame,
        samusd_frame,
        yoshi_frame,
        kirby_frame,
        fox_frame,
        pikachu_frame,
        luigi_frame,
        ness_frame,
        captain_frame,
        purin_frame,
        koopa_frame,
        ice_frame,
        sheik_frame,
        mariod_frame,
        pichu_frame,
        falco_frame,
        marth_frame,
        young_frame,
        ganon_frame,
        mewtwo_frame,
        roy_frame,
        chrom_frame,
        gamewatch_frame,
        meta_frame,
        pit_frame,
        pitb_frame,
        wario_frame,
        snake_frame,
        ike_frame,
        zenigame_frame,
        fushigisou_frame,
        lizardon_frame,
        diddy_frame,
        lucas_frame,
        sonic_frame,
        dedede_frame,
        pikmin_frame,
        lucario_frame,
        robot_frame,
        toonlink_frame,
        wolf_frame,
        murabito_frame,
        rockman_frame,
        //wiifit_frame,
        //littlemac_frame,
        gekkouga_frame,
        mii_frame,
        pacman_frame,
        shulk_frame,
        koopajr_frame,
        duckhunt_frame,
        ryu_frame,
        ken_frame,
        cloud_frame,
        ridley_frame,
        simon_frame,
        richter_frame,
        krool_frame,
        gaogaen_frame,
        packun_frame,
        jack_frame,
        brave_frame,
        buddy_frame,
        dolly_frame,
        pickel_frame,
        edge_frame,
        demon_frame,
        trail_frame
    );
    smashline::install_acmd_scripts!(
        
    );
}
