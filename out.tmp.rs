WorkModule::off_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_0 = fighter.global_table[0x16] == *SITUATION_KIND_GROUND;
if tmp_0 == 0 {
	let tmp_1 = KineticModule::change_kinetic(
		fighter.module_accessor,
		*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	let tmp_2 = GroundModule::correct(
		fighter.module_accessor,
		*GROUND_CORRECT_KIND_AIR
	);
	let tmp_3 = WorkModule::is_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_4 = tmp_3 & 1;
	let tmp_5 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_3 == 0 {
		if tmp_5 == 0 {
			let tmp_6 = fighter.global_table[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_6 == 0 {
				let tmp_7 = MotionModule::change_motion(
								fighter.module_accessor,
					0x11c0a0c60e,
					0,
					1.0,
					false,
					0,
					false,
					false
				);
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			} else {
			let tmp_8 = MotionModule::change_motion(
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
		let tmp_9 = MotionModule::change_motion(
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
	if tmp_5 == 0 {
		let tmp_10 = tmp_8[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_10 == 0 {
			let tmp_11 = MotionModule::change_motion_inherit_frame(
						fighter.module_accessor,
				0x11c0a0c60e,
				-1.0,
				1.0,
				0,
				false,
				false
			);
		} else {
		let tmp_12 = MotionModule::change_motion_inherit_frame(
				fighter.module_accessor,
			0x128aaa035a,
			-1.0,
			1.0,
			0,
			false,
			false
		);
	} else {
	let tmp_13 = MotionModule::change_motion_inherit_frame(
		fighter.module_accessor,
		0x12fdad33cc,
		-1.0,
		1.0,
		0,
		false,
		false
	);
} else {
let tmp_14 = KineticModule::change_kinetic(
fighter.module_accessor,
	*FIGHTER_KINETIC_TYPE_GROUND_STOP
);
let tmp_15 = GroundModule::correct(
fighter.module_accessor,
	*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
);
let tmp_16 = WorkModule::is_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
let tmp_17 = tmp_16 & 1;
let tmp_18 = tmp_8[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
if tmp_16 == 0 {
	if tmp_18 == 0 {
		let tmp_19 = tmp_8[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_19 == 0 {
			let tmp_20 = MotionModule::change_motion(
						fighter.module_accessor,
				0xd20cd6527,
				0,
				1.0,
				false,
				0,
				false,
				false
			);
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		} else {
		let tmp_21 = MotionModule::change_motion(
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
	let tmp_22 = MotionModule::change_motion(
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
if tmp_18 == 0 {
	let tmp_23 = tmp_21[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
	if tmp_23 == 0 {
		let tmp_24 = MotionModule::change_motion_inherit_frame(
				fighter.module_accessor,
			0xd20cd6527,
			-1.0,
			1.0,
			0,
			false,
			false
		);
	} else {
	let tmp_25 = MotionModule::change_motion_inherit_frame(
		fighter.module_accessor,
		0xec8f8f695,
		-1.0,
		1.0,
		0,
		false,
		false
	);
} else {
let tmp_26 = MotionModule::change_motion_inherit_frame(
fighter.module_accessor,
	0xebfffc603,
	-1.0,
	1.0,
	0,
	false,
	false
);
}
fighter.sub_shift_status_main()
