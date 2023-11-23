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
			let tmp_5 = fighter.global_table[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_5 {
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
			let tmp_6 = fighter.global_table[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_6 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_7 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_7 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		KineticModule::change_kinetic(
				fighter.module_accessor,
			*FIGHTER_KINETIC_TYPE_AIR_STOP
		);
		GroundModule::correct(
				fighter.module_accessor,
			*GROUND_CORRECT_KIND_AIR
		);
		let tmp_8 = WorkModule::is_flag(
				fighter.module_accessor,
			*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
		);
		let tmp_9 = tmp_8 & 1;
		let tmp_10 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
		if tmp_8 {
			if tmp_10 {
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
				let tmp_11 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_11 {
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
				let tmp_12 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_12 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
				let tmp_13 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_13 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
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
			if tmp_10 {
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
				let tmp_14 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_14 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
				let tmp_15 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_15 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
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
			if tmp_10 {
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
				let tmp_16 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_16 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
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
			let tmp_17 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_17 {
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
			let tmp_18 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_18 {
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
			let tmp_19 = fighter.module_accessor[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_19 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_20 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_20 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			MotionModule::change_motion_inherit_frame(
						fighter.module_accessor,
				0xd20cd6527,
				-1.0,
				1.0,
				0,
				false,
				false
			);
		} else {
			if tmp_10 {
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
				let tmp_21 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
				if tmp_21 {
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
					WorkModule::on_flag(
										fighter.module_accessor,
						*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
					);
				}
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
			let tmp_22 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_22 {
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
			let tmp_23 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_23 {
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
			let tmp_24 = fighter.module_accessor[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_24 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_25 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_25 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		MotionModule::change_motion_inherit_frame(
				fighter.module_accessor,
			0x11c0a0c60e,
			-1.0,
			1.0,
			0,
			false,
			false
		);
	} else {
		if tmp_10 {
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
			let tmp_26 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_26 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_27 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_27 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		if tmp_10 {
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
			let tmp_28 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_28 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		let tmp_29 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_29 {
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
		let tmp_30 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_30 {
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
		let tmp_31 = fighter.module_accessor[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_31 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		let tmp_32 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_32 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
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
	WorkModule::on_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
} else {
	KineticModule::change_kinetic(
		fighter.module_accessor,
		*FIGHTER_KINETIC_TYPE_AIR_STOP
	);
	GroundModule::correct(
		fighter.module_accessor,
		*GROUND_CORRECT_KIND_AIR
	);
	let tmp_33 = WorkModule::is_flag(
		fighter.module_accessor,
		*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
	);
	let tmp_34 = tmp_33 & 1;
	let tmp_35 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2;
	if tmp_33 {
		if tmp_35 {
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
			let tmp_36 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_36 {
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
			let tmp_37 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_37 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_38 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_38 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		if tmp_35 {
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
			let tmp_39 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_39 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
			let tmp_40 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_40 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		if tmp_35 {
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
			let tmp_41 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_41 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		let tmp_42 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_42 {
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
		let tmp_43 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_43 {
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
		let tmp_44 = fighter.module_accessor[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_44 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		let tmp_45 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_45 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		MotionModule::change_motion_inherit_frame(
				fighter.module_accessor,
			0xd20cd6527,
			-1.0,
			1.0,
			0,
			false,
			false
		);
	} else {
		if tmp_35 {
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
			let tmp_46 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
			if tmp_46 {
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
				WorkModule::on_flag(
								fighter.module_accessor,
					*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
				);
			}
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
		let tmp_47 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_47 {
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
		let tmp_48 = fighter.module_accessor[0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_48 {
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
		let tmp_49 = fighter.module_accessor[0x9][0x9][0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_49 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
		let tmp_50 = fighter.module_accessor[0x9] == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3;
		if tmp_50 {
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
			WorkModule::on_flag(
						fighter.module_accessor,
				*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
			);
		}
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
WorkModule::on_flag(
fighter.module_accessor,
	*FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT
);
fighter.sub_shift_status_main()
