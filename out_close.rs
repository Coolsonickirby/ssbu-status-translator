WorkModule::off_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_0 = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
if tmp_0 {
	KineticModule::change_kinetic(
		fighter.module_accessor,
		*FIGHTER_KINETIC_TYPE_GROUND_STOP
	);
	GroundModule::correct(
		fighter.module_accessor,
		*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
	);
	let tmp_1 = WorkModule::is_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_2 = tmp_1 & 1;
	let tmp_3 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_1 {
		if tmp_3 {
			MotionModule::change_motion_inherit_frame(
						fighter.module_accessor,
				0xebfffc603,
				-1.0,
				1.0,
				0,
				false,
				false
			);
		} else {
			let tmp_4 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_4 {
				MotionModule::change_motion_inherit_frame(
								fighter.module_accessor,
					0xec8f8f695,
					-1.0,
					1.0,
					0,
					false,
					false
				);
			} else {
				MotionModule::change_motion_inherit_frame(
								fighter.module_accessor,
					0xd20cd6527,
					-1.0,
					1.0,
					0,
					false,
					false
				);
			}
		}
	} else {
		if tmp_3 {
			MotionModule::change_motion(
						fighter.module_accessor,
				0xebfffc603,
				0,
				1.0,
				false,
				0,
				false,
				false
			);
		} else {
			let tmp_5 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_5 {
				MotionModule::change_motion(
								fighter.module_accessor,
					0xec8f8f695,
					0,
					1.0,
					false,
					0,
					false,
					false
				);
			} else {
				MotionModule::change_motion(
								fighter.module_accessor,
					0xd20cd6527,
					0,
					1.0,
					false,
					0,
					false,
					false
				);
			}
			WorkModule::on_flag(
                fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		WorkModule::on_flag(
				fighter.module_accessor,
			*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
		);
	}
} else {
	KineticModule::change_kinetic(
		fighter.module_accessor,
		*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
		fighter.module_accessor,
		*GROUND_CORRECT_KIND_AIR
	);
	let tmp_6 = WorkModule::is_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_7 = tmp_6 & 1;
	let tmp_8 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_6 {
		if tmp_8 {
			MotionModule::change_motion_inherit_frame(
						fighter.module_accessor,
				0x12fdad33cc,
				-1.0,
				1.0,
				0,
				false,
				false
			);
		} else {
			let tmp_9 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_9 {
				MotionModule::change_motion_inherit_frame(
								fighter.module_accessor,
					0x128aaa035a,
					-1.0,
					1.0,
					0,
					false,
					false
				);
			} else {
				MotionModule::change_motion_inherit_frame(
								fighter.module_accessor,
					0x11c0a0c60e,
					-1.0,
					1.0,
					0,
					false,
					false
				);
			}
		}
	} else {
		if tmp_8 {
			MotionModule::change_motion(
						fighter.module_accessor,
				0x12fdad33cc,
				0,
				1.0,
				false,
				0,
				false,
				false
			);
		} else {
			let tmp_10 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_10 {
				MotionModule::change_motion(
								fighter.module_accessor,
					0x128aaa035a,
					0,
					1.0,
					false,
					0,
					false,
					false
				);
			} else {
				MotionModule::change_motion(
								fighter.module_accessor,
					0x11c0a0c60e,
					0,
					1.0,
					false,
					0,
					false,
					false
				);
			}
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		WorkModule::on_flag(
            fighter.module_accessor,
			*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
		);
	}
}
fighter.sub_shift_status_main()
