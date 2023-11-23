WorkModule::off_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
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
	let tmp_1 = WorkModule::is_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_2 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_1 == 0 {
		if tmp_2 == 0 {
			let tmp_3 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_3 == 0 {
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
	if tmp_2 == 0 {
		let tmp_4 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_4 == 0 {
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
let tmp_5 = WorkModule::is_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_6 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
if tmp_5 == 0 {
	if tmp_6 == 0 {
		let tmp_7 = 48[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_7 == 0 {
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
if tmp_6 == 0 {
	let tmp_8 = 48[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
	if tmp_8 == 0 {
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
fighter.sub_shift_status_main()
None
