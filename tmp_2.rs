			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
	KineticModule::change_kinetic(
	fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
	fighter.module_accessor,
	*GROUND_CORRECT_KIND_AIR
	);
	WorkModule::is_flag(
	fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_1 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_0 == 0 {
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
		let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion_inherit_frame(
			fighter.module_accessor
			);
		}
	KineticModule::change_kinetic(
	fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
	fighter.module_accessor,
	*GROUND_CORRECT_KIND_AIR
	);
	WorkModule::is_flag(
	fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_1 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_0 == 0 {
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion_inherit_frame(
			fighter.module_accessor
			);
		}
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
let tmp_0 = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
if tmp_0 == 0 {
	KineticModule::change_kinetic(
	fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
	fighter.module_accessor,
	*GROUND_CORRECT_KIND_AIR
	);
	WorkModule::is_flag(
	fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_1 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_0 == 0 {
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion_inherit_frame(
			fighter.module_accessor
			);
		}
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
	MotionModule::change_motion_inherit_frame(
	fighter.module_accessor
	);
}
		let tmp_2 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
			WorkModule::on_flag(
			fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
			WorkModule::on_flag(
			fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
let tmp_0 = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
if tmp_0 == 0 {
	KineticModule::change_kinetic(
	fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
	fighter.module_accessor,
	*GROUND_CORRECT_KIND_AIR
	);
	WorkModule::is_flag(
	fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_1 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_0 == 0 {
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion_inherit_frame(
			fighter.module_accessor
			);
		}
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
	MotionModule::change_motion_inherit_frame(
	fighter.module_accessor
	);
}
KineticModule::change_kinetic(
fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_GROUND_STOP
);
GroundModule::correct(
fighter.module_accessor,
	*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
);
WorkModule::is_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_1 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
if tmp_2 == 0 {
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
			WorkModule::on_flag(
			fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	MotionModule::change_motion(
	fighter.module_accessor,
	false
	);
}
	let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
	if tmp_2 == 0 {
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
let tmp_0 = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
if tmp_0 == 0 {
	KineticModule::change_kinetic(
	fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
	fighter.module_accessor,
	*GROUND_CORRECT_KIND_AIR
	);
	WorkModule::is_flag(
	fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_1 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_0 == 0 {
		if tmp_1 == 0 {
			let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_2 == 0 {
				MotionModule::change_motion(
				fighter.module_accessor,
	false
				);
				WorkModule::on_flag(
				fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion_inherit_frame(
			fighter.module_accessor
			);
		}
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
	MotionModule::change_motion_inherit_frame(
	fighter.module_accessor
	);
}
KineticModule::change_kinetic(
fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_GROUND_STOP
);
GroundModule::correct(
fighter.module_accessor,
	*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
);
WorkModule::is_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_1 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
if tmp_2 == 0 {
	if tmp_1 == 0 {
		let tmp_2 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_2 == 0 {
			MotionModule::change_motion(
			fighter.module_accessor,
	false
			);
			WorkModule::on_flag(
			fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		MotionModule::change_motion(
		fighter.module_accessor,
	false
		);
	}
	MotionModule::change_motion(
	fighter.module_accessor,
	false
	);
}
if tmp_1 == 0 {
	let tmp_2 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
	if tmp_2 == 0 {
		MotionModule::change_motion_inherit_frame(
		fighter.module_accessor
		);
	}
	MotionModule::change_motion_inherit_frame(
	fighter.module_accessor
	);
}
