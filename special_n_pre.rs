fighter.sub_status_pre_SpecialNCommon()

StatusModule::init_settings(
	fighter.module_accessor,
	SituationKind(*SITUATION_KIND_NONE),
	*FIGHTER_KINETIC_TYPE_UNIQ,
	*GROUND_CORRECT_KIND_KEEP,
	GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
	true,
	0,
	0,
	0,
	0
)

FighterStatusModuleImpl::set_fighter_status_data(
	fighter.module_accessor,
	false,
	*FIGHTER_TREADED_KIND_NO_REAC,
	false,
	false,
	false,
	(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
	*FIGHTER_STATUS_ATTR_START_TURN,
	*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N,
	0
)

0.into()