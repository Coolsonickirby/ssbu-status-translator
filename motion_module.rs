pub mod MotionAnimcmdModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43MotionAnimcmdModule__exec_motion_lines_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn exec_motion_lines(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52MotionAnimcmdModule__change_script_motion_lines_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbbfb"]
        pub fn change_script_motion_lines(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: bool,
            arg5: bool,
            arg6: f32,
            arg7: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60MotionAnimcmdModule__change_script_motion_partial_lines_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbfb"]
        pub fn change_script_motion_partial_lines(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: bool,
            arg5: f32,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44MotionAnimcmdModule__call_script_single_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ei"]
        pub fn call_script_single(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58MotionAnimcmdModule__change_script_motion_line_single_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ei"]
        pub fn change_script_motion_line_single(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54MotionAnimcmdModule__exec_motion_lines_initialize_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn exec_motion_lines_initialize(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46MotionAnimcmdModule__flush_current_motion_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn flush_current_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionAnimcmdModule__flush_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn flush(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35MotionAnimcmdModule__set_sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sleep(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40MotionAnimcmdModule__set_sleep_game_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sleep_game(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42MotionAnimcmdModule__set_sleep_effect_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sleep_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41MotionAnimcmdModule__set_sleep_sound_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sleep_sound(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionAnimcmdModule__is_sleep_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sleep(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50MotionAnimcmdModule__enable_skip_delay_update_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_skip_delay_update(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod LinkEventLassoHang {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventLassoHang__load_from_l2c_table_implEPNS_18LinkEventLassoHangERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventLassoHang,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventLassoHang__store_l2c_table_implEPKNS_18LinkEventLassoHangE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventLassoHang)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventLassoHang__store_l2c_table_implEPKNS_18LinkEventLassoHangERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventLassoHang,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod TurnModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25TurnModule__set_turn_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efbbb"]
        pub fn set_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: bool,
            arg5: bool,
            arg6: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25TurnModule__end_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_turn(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24TurnModule__is_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_turn(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32TurnModule__is_turn_after90_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_turn_after90(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26TurnModule__is_extern_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_extern(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38TurnModule__set_omit_intermediate_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_omit_intermediate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27TurnModule__ry_reverse_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn ry_reverse(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod AreaContactLog {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AreaContactLog__load_from_l2c_table_implEPNS_14AreaContactLogERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::AreaContactLog,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AreaContactLog__store_l2c_table_implEPKNS_14AreaContactLogE"]
        pub fn store_l2c_table(arg1: *const root::app::AreaContactLog) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AreaContactLog__store_l2c_table_implEPKNS_14AreaContactLogERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::AreaContactLog,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterRidleyLinkEventMotion {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterRidleyLinkEventMotion__load_from_l2c_table_implEPNS_28FighterRidleyLinkEventMotionERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterRidleyLinkEventMotion,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterRidleyLinkEventMotion__store_l2c_table_implEPKNS_28FighterRidleyLinkEventMotionE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterRidleyLinkEventMotion,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterRidleyLinkEventMotion__store_l2c_table_implEPKNS_28FighterRidleyLinkEventMotionERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterRidleyLinkEventMotion,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventCapturePulled {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkEventCapturePulled__load_from_l2c_table_implEPNS_22LinkEventCapturePulledERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCapturePulled,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventCapturePulled__store_l2c_table_implEPKNS_22LinkEventCapturePulledE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventCapturePulled,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventCapturePulled__store_l2c_table_implEPKNS_22LinkEventCapturePulledERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCapturePulled,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ControlModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_stick_angle_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__get_stick_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__is_stick_side_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_stick_side(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__get_sub_stick_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_sub_stick_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__is_sub_stickSide_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sub_stickSide(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__is_enable_flick_jump_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_flick_jump(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__start_clatter_implEPNS_26BattleObjectModuleAccessorEfffaibb"]
        pub fn start_clatter(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
            arg5: libc::c_schar,
            arg6: libc::c_int,
            arg7: bool,
            arg8: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ControlModule__set_dec_time_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_dec_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__set_dec_time_recovery_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_dec_time_recovery(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__set_clatter_time_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_clatter_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__add_clatter_time_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn add_clatter_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__get_clatter_time_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_clatter_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__set_clatter_stop_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_clatter_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__is_clatter_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_clatter_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ControlModule__start_clatter_motion_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn start_clatter_motion_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__end_clatter_motion_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_clatter_motion_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__end_clatter_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn end_clatter(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__is_input_clatter_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_input_clatter(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__set_clatter_shake_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_clatter_shake_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ControlModule__reset_button_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_button(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__reset_trigger_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_trigger(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__reset_trigger_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_trigger_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ControlModule__reset_main_stick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_main_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__set_main_stick_x_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_main_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ControlModule__reset_main_stick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_main_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__set_main_stick_y_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_main_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__reset_main_stick_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_main_stick(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__reset_sub_stick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_sub_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__reset_sub_stick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_sub_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__reset_sub_stick_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_sub_stick(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__reset_flick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__reset_flick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__reset_flick_sub_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_sub_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__reset_flick_sub_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_sub_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        /// Returns the current x-axis position of the main control stick in a -1 to 1 range.
        /// (I.E. -1 = all the way left, 1 = all the way right, 0 = neutral)
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // if you are holding left during a taunt, take 2 points of damage
        /// if ControlModule::get_stick_x(module_accessor) < -0.5 && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_APPEAL {
        ///     DamageModule::add_damage(module_accessor, 2.0, 0);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_stick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__get_stick_prev_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_prev_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        /// Returns the current y-axis position of the main control stick in a -1 to 1 range.
        /// (I.E. -1 = all the way down, 1 = all the way up, 0 = neutral)
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // if you are holding down during a taunt, take 2 points of damage
        /// if ControlModule::get_stick_y(module_accessor) < -0.5 && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_APPEAL {
        ///     DamageModule::add_damage(module_accessor, 2.0, 0);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_stick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__get_stick_prev_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_prev_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_flick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_flick_x_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_x_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_flick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_flick_y_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_y_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_flick_no_reset_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_no_reset_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_flick_no_reset_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_no_reset_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__get_flick_after_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_after_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__get_flick_after_x_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_after_x_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__get_flick_after_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_after_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_flick_sub_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_sub_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__get_flick_sub_x_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_sub_x_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_flick_sub_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_sub_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__get_flick_sub_y_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_sub_y_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_sub_stick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_sub_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_sub_stick_prev_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_sub_stick_prev_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_sub_stick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_sub_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_sub_stick_prev_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_sub_stick_prev_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_trigger_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_trigger(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__get_trigger_count_implEPNS_26BattleObjectModuleAccessorEh"]
        pub fn get_trigger_count(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_trigger_count_prev_implEPNS_26BattleObjectModuleAccessorEh"]
        pub fn get_trigger_count_prev(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ControlModule__get_button_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_button(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_button_prev_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_button_prev(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__get_release_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_release(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        /// Returns whether or not the current module_accessor has pressed the specified CONTROL_PAD_BUTTON on the same frame this function was run
        ///
        /// # Arguments
        ///
        /// * `moudule_accessor` - a pointer to BattleObjectModuleAccessor
        ///
        /// * `control_pad_button` - a CONTROL_PAD_BUTTON lua const
        ///
        /// # Example
        ///
        /// ``` if the player has just pressed the jump button, add 5 percent.
        /// if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        ///     DamageModule::add_damage(boma, 5.0, 0);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__check_button_trigger_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_trigger(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__check_button_release_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_release(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        /// Returns whether or not the current module_accessor is holding the specified CONTROL_PAD_BUTTON on the same frame this function was run
        ///
        /// # Arguments
        ///
        /// * `moudule_accessor` - a pointer to BattleObjectModuleAccessor
        ///
        /// * `control_pad_button` - a CONTROL_PAD_BUTTON lua const
        ///
        /// # Example
        ///
        /// ``` if the player is holding the guard (shield) button, add 5 percent.
        /// if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ///     DamageModule::add_damage(boma, 5.0, 0);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__check_button_on_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_on(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        /// Returns whether or not the current module_accessor is NOT holding the specified CONTROL_PAD_BUTTON on the same frame this function was run
        ///
        /// # Arguments
        ///
        /// * `moudule_accessor` - a pointer to BattleObjectModuleAccessor
        ///
        /// * `control_pad_button` - a CONTROL_PAD_BUTTON lua const
        ///
        /// # Example
        ///
        /// ``` if the player is not holding the guard (shield) button, add 1 percent.
        /// if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ///     DamageModule::add_damage(boma, 1.0, 0);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__check_button_off_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_off(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__check_button_on_trriger_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_on_trriger(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__check_button_on_release_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn check_button_on_release(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ControlModule__set_off_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_off(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__set_stick_reverse_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_stick_reverse(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__is_stick_reversed_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_stick_reversed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__get_clatter_threshold_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_clatter_threshold(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__set_rumble_hit_data_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eibji"]
        pub fn set_rumble_hit_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
            arg5: libc::c_uint,
            arg6: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__clear_rumble_hit_data_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_rumble_hit_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ControlModule__request_rumble_hit_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn request_rumble_hit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ControlModule__set_rumble_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eibj"]
        pub fn set_rumble(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
            arg5: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ControlModule__set_rumble_all_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eij"]
        pub fn set_rumble_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__stop_rumble_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej"]
        pub fn stop_rumble_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ControlModule__stop_rumble_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn stop_rumble_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ControlModule__stop_rumble_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn stop_rumble(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__stop_rumble_all_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej"]
        pub fn stop_rumble_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__set_reverse_x_frame_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_reverse_x_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ControlModule__get_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_lr(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__get_flick_bonus_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_bonus(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ControlModule__get_flick_bonus_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_flick_bonus_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__reset_flick_bonus_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_bonus(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__reset_flick_bonus_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_flick_bonus_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_stick_x_no_clamp_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_x_no_clamp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_stick_y_no_clamp_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_stick_y_no_clamp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ControlModule__get_pad_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_pad_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        /// returns the current "command flag" based on the player's current controller inputs. Command flags come in 4 different categories.
        ///
        /// The first category is for general inputs that are done very often (aerials, attacks, walking, dashing, jumping, etc).
        /// The second category is for less common inputs. Stuff like taunts, rolls, throws (and strangely, shielding).
        /// The third category is for things like item tossing.
        /// The fourth category is reserved exclusively for command inputs for ryu/ken/terry.
        ///
        /// When the function is called, it will poll inputs from *only* the specified category.
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'category" - the desired category to poll inputs from, indexed from 0-3.... (0 meaning category 1, 1 meaning category 2, etc.)
        ///
        /// # Example
        ///
        /// ```
        /// // set a boolean to true if the player inputs a forward aerial.
        /// // because "cat"s can be represented as bitmasks, a useful way to compare them is with bitwise operations, as seen below
        /// let mut random_bool = false;
        /// let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        /// if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F) != 0 {
        ///     random_bool = true;
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind40ControlModule__get_command_flag_cat_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_command_flag_cat(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__clear_command_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn clear_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__clear_command_one_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn clear_command_one(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__get_command_life_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_command_life(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ControlModule__get_command_life_count_max_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_command_life_count_max(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__set_command_life_extend_implEPNS_26BattleObjectModuleAccessorEh"]
        pub fn set_command_life_extend(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__is_clear_command_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_clear_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ControlModule__exec_command_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn exec_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ControlModule__set_back_command_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_back_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ControlModule__reset_turn_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_turn_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_attack_hi3_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_hi3_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_attack_lw3_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_lw3_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__get_attack_air_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_air_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__reset_attack_air_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_attack_air_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__set_attack_air_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_attack_air_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_attack_air_stick_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_air_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_attack_air_stick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_air_stick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ControlModule__get_attack_air_stick_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_air_stick_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_down_stand_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_down_stand_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ControlModule__reset_down_stand_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_down_stand_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__set_down_stand_fb_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_down_stand_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ControlModule__item_light_throw_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn item_light_throw_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ControlModule__item_light_throw_fb4_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn item_light_throw_fb4_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ControlModule__item_light_throw_air_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn item_light_throw_air_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ControlModule__item_light_throw_air_fb4_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn item_light_throw_air_fb4_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ControlModule__item_heavy_throw_fb_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn item_heavy_throw_fb_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ControlModule__special_s_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn special_s_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__is_jump_mini_button_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_jump_mini_button(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ControlModule__set_add_jump_mini_button_life_implEPNS_26BattleObjectModuleAccessorEa"]
        pub fn set_add_jump_mini_button_life(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_schar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ControlModule__set_log_active_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_log_active(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ControlModule__is_remake_command_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_remake_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__get_reserve_turn_lr_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_reserve_turn_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__delete_reserve_turn_lr_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn delete_reserve_turn_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ControlModule__set_special_command_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_special_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ControlModule__reset_special_command_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn reset_special_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ControlModule__reverse_special_command_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reverse_special_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ControlModule__get_special_command_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_special_command_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ControlModule__reverse_x_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reverse_x_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51ControlModule__set_special_command_life_extend_implEPNS_26BattleObjectModuleAccessorEa"]
        pub fn set_special_command_life_extend(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_schar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57ControlModule__set_special_command_life_count_extend_implEPNS_26BattleObjectModuleAccessorEh"]
        pub fn set_special_command_life_count_extend(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
        );
    }
}
pub mod DamageModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24DamageModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30DamageModule__init_damage_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn init_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35DamageModule__set_force_damage_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fEiibbbb"]
        pub fn set_force_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector3f,
            arg4: libc::c_int,
            arg5: libc::c_int,
            arg6: bool,
            arg7: bool,
            arg8: bool,
            arg9: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29DamageModule__add_damage_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn add_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25DamageModule__damage_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27DamageModule__reaction_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28DamageModule__power_max_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn power_max(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36DamageModule__check_no_reaction_implEPNS_26BattleObjectModuleAccessorERKNS_10DamageInfoE"]
        pub fn check_no_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::app::DamageInfo,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46DamageModule__set_no_reaction_mode_status_implEPNS_26BattleObjectModuleAccessorENS_20DamageNoReactionModeEffi"]
        pub fn set_no_reaction_mode_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::DamageNoReactionMode,
            arg3: f32,
            arg4: f32,
            arg5: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48DamageModule__reset_no_reaction_mode_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_no_reaction_mode_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47DamageModule__set_no_reaction_damage_power_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_no_reaction_damage_power(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44DamageModule__set_no_reaction_no_effect_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_reaction_no_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46DamageModule__is_no_reaction_mode_perfect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_no_reaction_mode_perfect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29DamageModule__damage_log_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn damage_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36DamageModule__set_attacker_info_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_attacker_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33DamageModule__is_capture_cut_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn is_capture_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44DamageModule__set_ignore_capture_cut_no_implEPNS_26BattleObjectModuleAccessorEa"]
        pub fn set_ignore_capture_cut_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_schar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33DamageModule__set_damage_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_damage_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37DamageModule__set_damage_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_damage_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39DamageModule__set_force_damage_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_force_damage_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35DamageModule__set_reaction_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39DamageModule__set_reaction_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39DamageModule__set_reaction_mul_4th_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33DamageModule__set_weak_param_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef"]
        pub fn set_weak_param(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34DamageModule__set_damage_lock_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_damage_lock(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38DamageModule__set_damage_lock_2nd_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_damage_lock_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33DamageModule__is_damage_lock_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_damage_lock(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23DamageModule__heal_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn heal(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47DamageModule__overwrite_log_reaction_frame_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn overwrite_log_reaction_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40DamageModule__start_damage_info_log_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn start_damage_info_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38DamageModule__end_damage_info_log_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_damage_info_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52DamageModule__set_force_damage_from_last_damage_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_force_damage_from_last_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30DamageModule__is_paralyze_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_paralyze(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35DamageModule__set_critical_hit_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_critical_hit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34DamageModule__is_critical_hit_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_critical_hit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
}
pub mod FighterWorkModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterWorkModuleImpl__calc_param_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn calc_param(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterWorkModuleImpl__calc_escape_air_slide_param_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn calc_escape_air_slide_param(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
}
pub mod BattleObjectSlow {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27BattleObjectSlow__rate_implEPNS_16BattleObjectSlowE"]
        pub fn rate(arg1: *mut root::app::BattleObjectSlow) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34BattleObjectSlow__is_estimate_implEPNS_16BattleObjectSlowE"]
        pub fn is_estimate(arg1: *mut root::app::BattleObjectSlow) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32BattleObjectSlow__is_adjust_implEPNS_16BattleObjectSlowE"]
        pub fn is_adjust(arg1: *mut root::app::BattleObjectSlow) -> bool;
    }
}
pub mod GimmickEventPipe {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GimmickEventPipe__load_from_l2c_table_implEPNS_16GimmickEventPipeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventPipe,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GimmickEventPipe__store_l2c_table_implEPKNS_16GimmickEventPipeE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventPipe) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GimmickEventPipe__store_l2c_table_implEPKNS_16GimmickEventPipeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventPipe,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LuaModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27LuaModule__enable_line_implEPNS_26BattleObjectModuleAccessorENS_15LuaScriptLineIDE"]
        pub fn enable_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptLineID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28LuaModule__disable_line_implEPNS_26BattleObjectModuleAccessorENS_15LuaScriptLineIDE"]
        pub fn disable_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptLineID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29LuaModule__is_valid_line_implEPNS_26BattleObjectModuleAccessorENS_15LuaScriptLineIDE"]
        pub fn is_valid_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptLineID,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32LuaModule__get_execute_line_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_execute_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41LuaModule__reserve_status_data_array_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn reserve_status_data_array(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27LuaModule__enable_func_implEPNS_26BattleObjectModuleAccessorENS_21LuaScriptStatusFuncIDE"]
        pub fn enable_func(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptStatusFuncID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28LuaModule__disable_func_implEPNS_26BattleObjectModuleAccessorENS_21LuaScriptStatusFuncIDE"]
        pub fn disable_func(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptStatusFuncID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29LuaModule__is_valid_func_implEPNS_26BattleObjectModuleAccessorENS_21LuaScriptStatusFuncIDE"]
        pub fn is_valid_func(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LuaScriptStatusFuncID,
        ) -> bool;
    }
}
pub mod LinkEventCaptureDriver {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkEventCaptureDriver__load_from_l2c_table_implEPNS_22LinkEventCaptureDriverERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCaptureDriver,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventCaptureDriver__store_l2c_table_implEPKNS_22LinkEventCaptureDriverE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventCaptureDriver,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventCaptureDriver__store_l2c_table_implEPKNS_22LinkEventCaptureDriverERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCaptureDriver,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod EffectModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22EffectModule__req_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_fjibi"]
        pub fn req(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg6: libc::c_uint,
            arg7: libc::c_int,
            arg8: bool,
            arg9: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25EffectModule__req_2d_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_fj"]
        pub fn req_2d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg6: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31EffectModule__req_on_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RKNS3_8Vector3fES7_fS7_S7_bjii"]
        pub fn req_on_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: root::phx::Hash40,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg7: *const root::phx::Vector3f,
            arg8: *const root::phx::Vector3f,
            arg9: bool,
            arg10: libc::c_uint,
            arg11: libc::c_int,
            arg12: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36EffectModule__req_attach_ground_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS_25GroundCollisionLineHandleERKNS3_8Vector3fESA_fSA_ji"]
        pub fn req_attach_ground(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: *const root::app::GroundCollisionLineHandle,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg7: *const root::phx::Vector3f,
            arg8: libc::c_uint,
            arg9: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29EffectModule__req_follow_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RKNS3_8Vector3fES7_fbjiiiibb"]
        pub fn req_follow(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: root::phx::Hash40,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg7: bool,
            arg8: libc::c_uint,
            arg9: libc::c_int,
            arg10: libc::c_int,
            arg11: libc::c_int,
            arg12: libc::c_int,
            arg13: bool,
            arg14: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__req_continual_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_fji"]
        pub fn req_continual(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: root::phx::Hash40,
            size: f32,
            arg5: libc::c_uint,
            arg6: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__remove_continual_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn remove_continual(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39EffectModule__remove_all_continual_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn remove_all_continual(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__req_time_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EiRKNS3_8Vector3fES7_fjbb"]
        pub fn req_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: libc::c_int,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg7: libc::c_uint,
            arg8: bool,
            arg9: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__req_time_follow_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_iRKNS3_8Vector3fES7_fbj"]
        pub fn req_time_follow(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: libc::c_int,
            pos: *const root::phx::Vector3f,
            rot: *const root::phx::Vector3f,
            size: f32,
            arg8: bool,
            arg9: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30EffectModule__remove_time_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn remove_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__remove_all_time_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn remove_all_time(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__req_emit_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej"]
        pub fn req_emit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25EffectModule__remove_implEPNS_26BattleObjectModuleAccessorEjj"]
        pub fn remove(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29EffectModule__remove_all_implEPNS_26BattleObjectModuleAccessorEjj"]
        pub fn remove_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23EffectModule__kill_implEPNS_26BattleObjectModuleAccessorEjbb"]
        pub fn kill(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28EffectModule__kill_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb"]
        pub fn kill_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__kill_joint_id_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb"]
        pub fn kill_joint_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            effHash: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__kill_all_implEPNS_26BattleObjectModuleAccessorEjbb"]
        pub fn kill_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29EffectModule__detach_all_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn detach_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30EffectModule__detach_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn detach_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25EffectModule__detach_implEPNS_26BattleObjectModuleAccessorEji"]
        pub fn detach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__end_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn end_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__req_after_image_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_S4_S4_jRKNS3_8Vector3fES7_jS4_S4_S7_S7_fhhfiii"]
        pub fn req_after_image(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: root::phx::Hash40,
            arg5: root::phx::Hash40,
            arg6: libc::c_uint,
            arg7: *const root::phx::Vector3f,
            arg8: *const root::phx::Vector3f,
            arg9: libc::c_uint,
            arg10: root::phx::Hash40,
            arg11: root::phx::Hash40,
            arg12: *const root::phx::Vector3f,
            arg13: *const root::phx::Vector3f,
            arg14: f32,
            arg15: libc::c_uchar,
            arg16: libc::c_uchar,
            arg17: f32,
            arg18: libc::c_int,
            arg19: libc::c_int,
            arg20: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44EffectModule__req_after_image_no_parent_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_jRKNS3_8Vector3fES7_jS4_S4_S7_S7_fhhfiii"]
        pub fn req_after_image_no_parent(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: libc::c_uint,
            arg5: *const root::phx::Vector3f,
            arg6: *const root::phx::Vector3f,
            arg7: libc::c_uint,
            arg8: root::phx::Hash40,
            arg9: root::phx::Hash40,
            arg10: *const root::phx::Vector3f,
            arg11: *const root::phx::Vector3f,
            arg12: f32,
            arg13: libc::c_uchar,
            arg14: libc::c_uchar,
            arg15: f32,
            arg16: libc::c_int,
            arg17: libc::c_int,
            arg18: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40EffectModule__clear_all_after_image_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn clear_all_after_image(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37EffectModule__remove_after_image_implEPNS_26BattleObjectModuleAccessorEjj"]
        pub fn remove_after_image(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41EffectModule__remove_all_after_image_implEPNS_26BattleObjectModuleAccessorEjj"]
        pub fn remove_all_after_image(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__get_local_matrix_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_local_matrix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26EffectModule__set_pos_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE"]
        pub fn set_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26EffectModule__set_rot_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE"]
        pub fn set_rot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28EffectModule__set_scale_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector3fE"]
        pub fn set_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__is_exist_effect_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_exist_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30EffectModule__set_visible_implEPNS_26BattleObjectModuleAccessorEjb"]
        pub fn set_visible(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__set_visible_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn set_visible_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28EffectModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__is_whole_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33EffectModule__set_whole_attr_implEPNS_26BattleObjectModuleAccessorEbj"]
        pub fn set_whole_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30EffectModule__fill_screen_implEPNS_26BattleObjectModuleAccessorEiiRKN3phx8Vector4fENS_21EffectScreenBlendTypeENS_17EffectScreenLayerEh"]
        pub fn fill_screen(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: *const root::phx::Vector4f,
            arg5: root::app::EffectScreenBlendType,
            arg6: root::app::EffectScreenLayer,
            arg7: libc::c_uchar,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__fill_screen_2_implEPNS_26BattleObjectModuleAccessorEiiRKN3phx8Vector4fES6_ffNS_17EffectScreenLayerEh"]
        pub fn fill_screen_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: *const root::phx::Vector4f,
            arg5: *const root::phx::Vector4f,
            arg6: f32,
            arg7: f32,
            arg8: root::app::EffectScreenLayer,
            arg9: libc::c_uchar,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__color_collection_implEPNS_26BattleObjectModuleAccessorEiiRKN3phx8Vector4fES6_fffffNS_21EffectScreenBlendTypeEh"]
        pub fn color_collection(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: *const root::phx::Vector4f,
            arg5: *const root::phx::Vector4f,
            arg6: f32,
            arg7: f32,
            arg8: f32,
            arg9: f32,
            arg10: f32,
            arg11: root::app::EffectScreenBlendType,
            arg12: libc::c_uchar,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31EffectModule__clear_screen_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn clear_screen(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31EffectModule__reset_screen_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_screen(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40EffectModule__get_dead_effect_rot_z_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEfb"]
        pub fn get_dead_effect_rot_z(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: f32,
            arg4: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40EffectModule__get_dead_effect_scale_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEfb"]
        pub fn get_dead_effect_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: f32,
            arg4: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39EffectModule__is_dead_effect_slant_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEf"]
        pub fn is_dead_effect_slant(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29EffectModule__req_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef"]
        pub fn req_common(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__remove_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn remove_common(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31EffectModule__reset_common_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_common(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__is_end_common_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_end_common(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__is_exist_common_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_exist_common(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29EffectModule__req_screen_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebbb"]
        pub fn req_screen(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36EffectModule__req_screen_system_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb"]
        pub fn req_screen_system(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__remove_screen_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn remove_screen(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38EffectModule__set_sync_visibility_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sync_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37EffectModule__is_sync_visibility_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sync_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33EffectModule__set_sync_scale_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sync_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__is_sync_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sync_scale(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44EffectModule__get_variation_effect_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn get_variation_effect_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26EffectModule__set_rgb_implEPNS_26BattleObjectModuleAccessorEjfff"]
        pub fn set_rgb(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39EffectModule__set_rgb_partial_last_implEPNS_26BattleObjectModuleAccessorEfff"]
        pub fn set_rgb_partial_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28EffectModule__set_alpha_implEPNS_26BattleObjectModuleAccessorEjf"]
        pub fn set_alpha(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28EffectModule__set_frame_implEPNS_26BattleObjectModuleAccessorEjf"]
        pub fn set_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__set_billboard_implEPNS_26BattleObjectModuleAccessorEjb"]
        pub fn set_billboard(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27EffectModule__set_rate_implEPNS_26BattleObjectModuleAccessorEjf"]
        pub fn set_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32EffectModule__set_rate_last_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_rate_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33EffectModule__set_scale_last_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_scale_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33EffectModule__set_alpha_last_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_alpha_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42EffectModule__set_disable_system_slow_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_disable_system_slow(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36EffectModule__set_slow_rate_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_slow_rate_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44EffectModule__set_slow_rate_no_stop_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_slow_rate_no_stop_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30EffectModule__set_add_vel_implEPNS_26BattleObjectModuleAccessorEjRN3phx8Vector3fE"]
        pub fn set_add_vel(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *mut root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41EffectModule__set_material_anim_last_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_material_anim_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49EffectModule__set_disable_render_offset_last_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_disable_render_offset_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__get_last_handle_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_last_handle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42EffectModule__is_enable_ground_effect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_ground_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37EffectModule__kill_status_effect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn kill_status_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__unsync_vis_whole_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn unsync_vis_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34EffectModule__sync_visibility_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sync_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37EffectModule__set_offset_to_next_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_offset_to_next(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__make_offset_hash_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn make_offset_hash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47EffectModule__preset_lifetime_rate_partial_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn preset_lifetime_rate_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41EffectModule__preset_curtail_emitter_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn preset_curtail_emitter(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35EffectModule__preset_limit_num_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn preset_limit_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44EffectModule__enable_sync_init_pos_last_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_sync_init_pos_last(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39EffectModule__set_custom_uv_offset_implEPNS_26BattleObjectModuleAccessorEjRKN3phx8Vector2fEi"]
        pub fn set_custom_uv_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector2f,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42EffectModule__remove_post_effect_line_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn remove_post_effect_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50EffectModule__request_post_effect_line_circle_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_NS3_8Vector2fENS3_8Vector3fEbff"]
        pub fn request_post_effect_line_circle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: root::phx::Vector2f,
            arg5: root::phx::Vector3f,
            arg6: bool,
            arg7: f32,
            arg8: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53EffectModule__set_post_effect_line_circle_target_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS3_8Vector2fENS3_8Vector3fEb"]
        pub fn set_post_effect_line_circle_target(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Vector2f,
            arg4: root::phx::Vector3f,
            arg5: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55EffectModule__request_post_effect_line_parallel_2d_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS3_8Vector2fES5_S5_S5_bff"]
        pub fn request_post_effect_line_parallel_2d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Vector2f,
            arg4: root::phx::Vector2f,
            arg5: root::phx::Vector2f,
            arg6: root::phx::Vector2f,
            arg7: bool,
            arg8: f32,
            arg9: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33EffectModule__enable_stencil_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn enable_stencil(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45EffectModule__force_update_common_effect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn force_update_common_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminSyncPos {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind67FighterPikminLinkEventWeaponPikminSyncPos__load_from_l2c_table_implEPNS_41FighterPikminLinkEventWeaponPikminSyncPosERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminSyncPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind63FighterPikminLinkEventWeaponPikminSyncPos__store_l2c_table_implEPKNS_41FighterPikminLinkEventWeaponPikminSyncPosE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSyncPos,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind63FighterPikminLinkEventWeaponPikminSyncPos__store_l2c_table_implEPKNS_41FighterPikminLinkEventWeaponPikminSyncPosERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSyncPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod CancelModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CancelModule__is_enable_cancel_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_cancel(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CancelModule__enable_cancel_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_cancel(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminChangeMotion {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind72FighterPikminLinkEventWeaponPikminChangeMotion__load_from_l2c_table_implEPNS_46FighterPikminLinkEventWeaponPikminChangeMotionERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminChangeMotion,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterPikminLinkEventWeaponPikminChangeMotion__store_l2c_table_implEPKNS_46FighterPikminLinkEventWeaponPikminChangeMotionE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminChangeMotion,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterPikminLinkEventWeaponPikminChangeMotion__store_l2c_table_implEPKNS_46FighterPikminLinkEventWeaponPikminChangeMotionERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminChangeMotion,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventDrumCheckNeedLock {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55GimmickEventDrumCheckNeedLock__load_from_l2c_table_implEPNS_29GimmickEventDrumCheckNeedLockERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumCheckNeedLock,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventDrumCheckNeedLock__store_l2c_table_implEPKNS_29GimmickEventDrumCheckNeedLockE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumCheckNeedLock,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventDrumCheckNeedLock__store_l2c_table_implEPKNS_29GimmickEventDrumCheckNeedLockERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumCheckNeedLock,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GroundModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__update_force_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_force(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__update_shape_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn update_shape(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__get_rhombus_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn get_rhombus(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> *const root::phx::Vector4f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__modify_rhombus_implEPNS_26BattleObjectModuleAccessorEfff"]
        pub fn modify_rhombus(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__set_init_circle_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fEf"]
        pub fn set_init_circle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__get_circle_radius_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_circle_radius(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__set_offset_x_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_offset_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__set_offset_y_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_offset_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        /// returns the x offset for the current battle object's ECB's base. The base of the ECB rhombus is between the character's feet
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__get_offset_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_offset_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        /// returns the y offset for the current battle object's ECB's base. The base of the ECB rhombus is between the character's feet
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__get_offset_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_offset_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        /// Sets the offset for the current battle object's ECB (Environmental collision box)
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * `offset_vector` - Pointer to a Vector2f which specifies the x and y values to set the new ECB offset to.
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__set_rhombus_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn set_rhombus_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26GroundModule__correct_implEPNS_26BattleObjectModuleAccessorENS_17GroundCorrectKindE"]
        pub fn correct(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundCorrectKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__set_correct_implEPNS_26BattleObjectModuleAccessorENS_17GroundCorrectKindE"]
        pub fn set_correct(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundCorrectKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__get_correct_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_correct(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__set_collidable_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_collidable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__set_passable_check_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_passable_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__is_passable_check_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_passable_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__set_cloud_through_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_cloud_through(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__set_ignore_line_type1_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_line_type1(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__set_ignore_boss_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_boss(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GroundModule__is_ignore_fighter_other_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_ignore_fighter_other(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__set_ignore_fighter_other_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_fighter_other(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GroundModule__is_ignore_fighter_other_wall_l_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_ignore_fighter_other_wall_l(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GroundModule__is_ignore_fighter_other_wall_r_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_ignore_fighter_other_wall_r(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__set_correct_ignore_slope_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_correct_ignore_slope(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__set_cliff_check_implEPNS_26BattleObjectModuleAccessorENS_20GroundCliffCheckKindE"]
        pub fn set_cliff_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundCliffCheckKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__cliff_check_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn cliff_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__select_cliff_hangdata_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn select_cliff_hangdata(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__is_status_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_status_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__correct_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn correct_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__entry_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn entry_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__reentry_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reentry_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__can_entry_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn can_entry_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39GroundModule__can_entry_cliff_same_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector2fE"]
        pub fn can_entry_cliff_same(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GroundModule__can_entry_cliff_hang_data_implEPNS_26BattleObjectModuleAccessorEjNS_20GroundCliffCheckKindE"]
        pub fn can_entry_cliff_hang_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: root::app::GroundCliffCheckKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__leave_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn leave_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__hang_cliff_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hang_cliff_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        /// returns the coordinates of the nearest cliff as a Vector3f
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // store the position of the current nearest cliff if the player is in the ledge hang status
        /// let mut pos: smash::phx::Vector3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        /// if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        ///     pos = GroundModule::hang_cliff_pos_3f(module_accessor);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__hang_cliff_pos_3f_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hang_cliff_pos_3f(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__hang_can_entry_cliff_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hang_can_entry_cliff_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__hang_cliff_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hang_cliff_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__hang_can_entry_cliff_dir_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hang_can_entry_cliff_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__get_cliff_movement_speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_cliff_movement_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GroundModule__is_cliff_move_exception_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_cliff_move_exception(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__clear_cliff_point_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_cliff_point(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__set_test_coll_stop_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_test_coll_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GroundModule__set_test_coll_stop_status_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_test_coll_stop_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48GroundModule__set_coll_stop_slidable_length_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_coll_stop_slidable_length(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GroundModule__set_ignore_slide_up_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_slide_up(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GroundModule__is_ottotto_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn is_ottotto(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__is_ottotto_lr_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn is_ottotto_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__is_myground_nearcliff_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn is_myground_nearcliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__is_near_cliff_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn is_near_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__is_miss_foot_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_miss_foot(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__set_shape_flag_implEPNS_26BattleObjectModuleAccessorEtb"]
        pub fn set_shape_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_ushort,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__set_shape_safe_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn set_shape_safe_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__get_shape_safe_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_shape_safe_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__set_status_ground_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_status_ground(module_accessor: *mut root::app::BattleObjectModuleAccessor);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GroundModule__set_init_shape_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_init_shape_kind(module_accessor: *mut root::app::BattleObjectModuleAccessor);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__get_shape_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_shape_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24GroundModule__set_z_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_z(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24GroundModule__get_z_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_z(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__attach_ground_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn attach_ground(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__set_attach_ground_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_attach_ground(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__is_attachable_implEPNS_26BattleObjectModuleAccessorENS_15GroundTouchFlagE"]
        pub fn is_attachable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundTouchFlag,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25GroundModule__attach_implEPNS_26BattleObjectModuleAccessorENS_15GroundTouchFlagE"]
        pub fn attach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundTouchFlag,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28GroundModule__is_attach_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_attach(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GroundModule__get_line_movement_speed_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_line_movement_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25GroundModule__detach_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn detach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23GroundModule__test_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn test(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35GroundModule__get_touch_normal_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__get_touch_normal_x_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__get_touch_normal_y_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52GroundModule__get_touch_normal_consider_gravity_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_consider_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54GroundModule__get_touch_normal_x_consider_gravity_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_x_consider_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54GroundModule__get_touch_normal_y_consider_gravity_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_y_consider_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57GroundModule__get_touch_normal_for_touch_attack_data_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_normal_for_touch_attack_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__ground_touch_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn ground_touch_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39GroundModule__ground_touch_flag_ex_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn ground_touch_flag_ex(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__get_touch_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_touch_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__get_touch_moment_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_touch_moment_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GroundModule__get_touch_material_type_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_material_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__get_touch_pos_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_touch_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27GroundModule__is_touch_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_touch(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__get_touch_line_raw_implEPNS_26BattleObjectModuleAccessorENS_13GroundTouchIDE"]
        pub fn get_touch_line_raw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::GroundTouchID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__get_touch_wall_cliff_pos_implEPNS_26BattleObjectModuleAccessorEjRN3phx8Vector2fE"]
        pub fn get_touch_wall_cliff_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *mut root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GroundModule__is_floor_touch_line_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_floor_touch_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__is_wall_touch_line_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_wall_touch_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__is_passable_ground_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_passable_ground(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__is_floor_vanishing_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_floor_vanishing(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GroundModule__get_cliff_id_uint32_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_cliff_id_uint32(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GroundModule__get_up_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_up_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__get_down_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_down_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GroundModule__get_left_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_left_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__get_right_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_right_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        ///Returns the X position of the base of your ECB relative the the origin of the current stage.
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        #[link_name = "\u{1}_ZN3app8lua_bind33GroundModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GroundModule__center_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28GroundModule__get_width_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_width(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GroundModule__get_down_movement_speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_down_movement_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41GroundModule__check_down_correct_pos_implEPNS_26BattleObjectModuleAccessorEN3phx8Vector2fE"]
        pub fn check_down_correct_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46GroundModule__get_latest_down_correct_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_latest_down_correct_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GroundModule__get_down_correct_edge_pos_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector2fERKS4_"]
        pub fn get_down_correct_edge_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__get_down_friction_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_down_friction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__get_distance_to_floor_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEfb"]
        pub fn get_distance_to_floor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: f32,
            arg4: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__is_still_on_floor_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn is_still_on_floor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28GroundModule__ray_check_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_b"]
        pub fn ray_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36GroundModule__ray_check_hit_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_b"]
        pub fn ray_check_hit_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39GroundModule__ray_check_hit_normal_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_b"]
        pub fn ray_check_hit_normal(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__ray_check_hit_pos_normal_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_S7_b"]
        pub fn ray_check_hit_pos_normal(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: *mut root::phx::Vector2f,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54GroundModule__ray_check_hit_pos_normal_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_S7_b"]
        pub fn ray_check_hit_pos_normal_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: *mut root::phx::Vector2f,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__ray_check_hit_pos_target_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_jb"]
        pub fn ray_check_hit_pos_target(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: libc::c_uint,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__ray_check_get_line_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_b"]
        pub fn ray_check_get_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48GroundModule__ray_check_get_line_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_b"]
        pub fn ray_check_get_line_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GroundModule__ray_check_get_line_hit_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_b"]
        pub fn ray_check_get_line_hit_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56GroundModule__ray_check_get_line_hit_pos_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_b"]
        pub fn ray_check_get_line_hit_pos_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48GroundModule__ray_check_get_line_target_any_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_jb"]
        pub fn ray_check_get_line_target_any(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: libc::c_uint,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59GroundModule__ray_check_get_line_target_any_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_jb"]
        pub fn ray_check_get_line_target_any_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: libc::c_uint,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48GroundModule__ray_check_get_line_ignore_any_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_j"]
        pub fn ray_check_get_line_ignore_any(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59GroundModule__ray_check_get_line_ignore_any_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_j"]
        pub fn ray_check_get_line_ignore_any_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56GroundModule__ray_check_get_line_hit_pos_ignore_any_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_j"]
        pub fn ray_check_get_line_hit_pos_ignore_any(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind67GroundModule__ray_check_get_line_hit_pos_ignore_any_no_culling_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_RS4_j"]
        pub fn ray_check_get_line_hit_pos_ignore_any_no_culling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *mut root::phx::Vector2f,
            arg5: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__line_segment_check_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_S6_RS4_b"]
        pub fn line_segment_check(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *const root::phx::Vector2f,
            arg5: *mut root::phx::Vector2f,
            arg6: bool,
        ) -> *const *const u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GroundModule__test_ground_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn test_ground(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GroundModule__set_ignore_friction_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_friction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__get_correct_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_correct_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__get_correct_pos_local_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_correct_pos_local(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35GroundModule__set_update_shape_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_update_shape(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__is_attach_cliff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_attach_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GroundModule__pass_floor_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pass_floor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35GroundModule__clear_pass_floor_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_pass_floor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GroundModule__set_auto_detach_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_auto_detach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GroundModule__set_no_cliff_stop_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_no_cliff_stop_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__set_gr_collision_mode_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_gr_collision_mode(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60GroundModule__set_shape_data_rhombus_modify_node_offset_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fE"]
        pub fn set_shape_data_rhombus_modify_node_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41GroundModule__set_keep_distant_cliff_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_keep_distant_cliff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GroundModule__set_reverse_direction_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_reverse_direction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GroundModule__set_rhombus_modify_air_lasso_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_rhombus_modify_air_lasso(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GroundModule__set_rhombus_modify_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_rhombus_modify(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
}
pub mod AbsorberModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AbsorberModule__clean_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clean(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AbsorberModule__is_shield_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn is_shield(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AbsorberModule__set_size_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AbsorberModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_12ShieldStatusEi"]
        pub fn set_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ShieldStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AbsorberModule__set_status_all_implEPNS_26BattleObjectModuleAccessorENS_12ShieldStatusEi"]
        pub fn set_status_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AbsorberModule__is_turn_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AbsorberModule__is_front_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AbsorberModule__is_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AbsorberModule__get_hop_angle_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_hop_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AbsorberModule__is_no_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AbsorberModule__set_turn_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AbsorberModule__set_front_implEPNS_26BattleObjectModuleAccessorENS_11ShieldFrontEi"]
        pub fn set_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldFront,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AbsorberModule__set_hop_implEPNS_26BattleObjectModuleAccessorEbfi"]
        pub fn set_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AbsorberModule__set_attack_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AbsorberModule__get_attack_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AbsorberModule__set_speed_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AbsorberModule__get_speed_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AbsorberModule__set_life_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AbsorberModule__get_life_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AbsorberModule__set_attack_limit_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AbsorberModule__get_attack_limit_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AbsorberModule__set_hit_stop_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_hit_stop_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AbsorberModule__is_no_m_ball_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_m_ball(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AbsorberModule__get_part_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_part_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AbsorberModule__get_team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AbsorberModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AbsorberModule__set_shield_type_implEPNS_26BattleObjectModuleAccessorENS_10ShieldTypeEii"]
        pub fn set_shield_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldType,
            arg3: libc::c_int,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AbsorberModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AbsorberModule__clear_all_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_all_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AbsorberModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AbsorberModule__get_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AbsorberModule__get_group_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_group_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AbsorberModule__get_pos_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_pos_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AbsorberModule__set_target_property_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_property(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AbsorberModule__set_target_category_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_category(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AbsorberModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
}
pub mod LinkEventCaptureItem {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46LinkEventCaptureItem__load_from_l2c_table_implEPNS_20LinkEventCaptureItemERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCaptureItem,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkEventCaptureItem__store_l2c_table_implEPKNS_20LinkEventCaptureItemE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventCaptureItem,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkEventCaptureItem__store_l2c_table_implEPKNS_20LinkEventCaptureItemERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCaptureItem,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventCaptureFishingrodDamage {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58LinkEventCaptureFishingrodDamage__load_from_l2c_table_implEPNS_32LinkEventCaptureFishingrodDamageERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCaptureFishingrodDamage,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54LinkEventCaptureFishingrodDamage__store_l2c_table_implEPKNS_32LinkEventCaptureFishingrodDamageE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventCaptureFishingrodDamage,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54LinkEventCaptureFishingrodDamage__store_l2c_table_implEPKNS_32LinkEventCaptureFishingrodDamageERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCaptureFishingrodDamage,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod AttackModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__clear_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn clear(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AttackModule__sleep_partialy_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn sleep_partialy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__set_attack_implEPNS_26BattleObjectModuleAccessorEiiRKNS_10AttackDataE"]
        pub fn set_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: *const root::app::AttackData,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__set_attack_2_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_attack_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__set_absolute_implEPNS_26BattleObjectModuleAccessorEiiRKNS_18AttackAbsoluteDataE"]
        pub fn set_absolute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: *const root::app::AttackAbsoluteData,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__hit_absolute_implEPNS_26BattleObjectModuleAccessorEijRKN3phx8Vector3fEii"]
        pub fn hit_absolute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
            arg4: *const root::phx::Vector3f,
            arg5: libc::c_int,
            arg6: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__hit_absolute_joint_implEPNS_26BattleObjectModuleAccessorEijN3phx6Hash40Eii"]
        pub fn hit_absolute_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
            arg4: root::phx::Hash40,
            arg5: libc::c_int,
            arg6: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__set_absolute_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_absolute_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__set_whole_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__is_attack_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn is_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_invalid_invincible_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_invalid_invincible(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackModule__is_invalid_invincible_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn is_invalid_invincible(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__set_invalid_xlu_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_invalid_xlu(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AttackModule__is_invalid_xlu_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn is_invalid_xlu(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43AttackModule__set_ignore_ground_shield_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_ignore_ground_shield(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_ignore_capture_cut_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_ignore_capture_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_schar,
            arg3: bool
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__is_hit_abs_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_hit_abs(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__set_power_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_power(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__add_power_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn add_power(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__set_vector_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_vector(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23AttackModule__size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AttackModule__set_size_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__set_target_category_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_target_category(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__off_target_kind_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn off_target_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__set_offset_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__set_offset2_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_offset2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__get_offset2_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn get_offset2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AttackModule__set_node_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40E"]
        pub fn set_node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_serial_hit_frame_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_serial_hit_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__get_power_implEPNS_26BattleObjectModuleAccessorEibfb"]
        pub fn get_power(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: f32,
            arg5: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__reaction_effect_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn reaction_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__set_reaction_effect_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_reaction_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__reaction_fix_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn reaction_fix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_reaction_fix_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_reaction_fix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__reaction_add_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn reaction_add(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AttackModule__reset_safe_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_safe_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_reaction_add_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_reaction_add(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__reaction_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__set_pos_x_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_pos_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__pos_x_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn pos_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AttackModule__pos_x_2_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_x_2(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__pos_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_y(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__center_pos_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AttackModule__speed_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn speed_x(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24AttackModule__speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn speed(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__set_speed_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn set_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__attack_data_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn attack_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> *mut root::app::AttackData;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_power_mul_status_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__power_mul_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_power_add_status_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_add_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__power_add_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_add_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_power_speed_status_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_speed_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__power_speed_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_speed_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackModule__set_power_speed_limit_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_speed_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__power_speed_limit_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_speed_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__set_power_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__power_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_power_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__power_mul_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_power_mul_3rd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul_3rd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__power_mul_3rd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul_3rd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_power_mul_4th_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__power_mul_4th_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_power_mul_5th_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_mul_5th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__power_mul_5th_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn power_mul_5th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45AttackModule__set_customize_attack_ratio_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_customize_attack_ratio(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__customize_attack_ratio_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn customize_attack_ratio(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_reaction_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_reaction_mul_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__reaction_mul_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reaction_mul_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_reaction_mul_3rd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul_3rd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__reaction_mul_3rd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reaction_mul_3rd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_reaction_mul_4th_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_reaction_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__reaction_mul_4th_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reaction_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_damage_reaction_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_damage_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__get_damage_reaction_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_damage_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_shield_stiff_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_shield_stiff_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__shield_stiff_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn shield_stiff_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_ignore_just_shield_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_just_shield(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25AttackModule__get_lr_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn get_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AttackModule__part_size_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn part_size(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__ref_log_group_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn ref_log_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__un_ref_log_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn un_ref_log_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__get_inflict_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_inflict(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        /// Returns whether or not you are *currently* hitting an opponent (in hitlag).
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'collision_kind_mask' - a COLLISION_KIND_MASK_ const
        ///
        /// # Example
        ///
        /// ```
        /// // transition to instantly to WAIT when hitting someone and holding shield
        /// if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ///     StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__is_infliction_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_infliction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__get_inflict_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_inflict_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        /// Returns whether or not you have hit something during the current status
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'collision_kind_mask' - a COLLISION_KIND_MASK_ const
        ///
        /// # Example
        ///
        /// ```
        /// // transition to instantly to WAIT when you've hit someone in the current status and are holding shield
        /// if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ///     StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__is_infliction_status_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_infliction_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_indirect_info_implEPNS_26BattleObjectModuleAccessorEjibb"]
        pub fn set_indirect_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_int,
            arg4: bool,
            arg5: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__indirect_object_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn indirect_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__indirect_team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn indirect_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__set_restrict_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_restrict(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__set_no_attacker_log_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_no_attacker_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AttackModule__set_catch_only_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_catch_only(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_catch_only_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_catch_only_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_attack_keep_rumble_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_attack_keep_rumble(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__get_power_up_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_power_up(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AttackModule__set_power_up_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_up(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__get_power_up_attr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_power_up_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_power_up_attr_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_up_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__get_attacker_attribute_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attacker_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::app::AttackerAttribute;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_attacker_attribute_implEPNS_26BattleObjectModuleAccessorENS_17AttackerAttributeE"]
        pub fn set_attacker_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::AttackerAttribute,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__get_power_apply_defense_implEPNS_26BattleObjectModuleAccessorEffRKS1_"]
        pub fn get_power_apply_defense(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: *const root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__attack_part_speed_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn attack_part_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__attack_reference_pos_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fE"]
        pub fn attack_reference_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__attack_direction_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn attack_direction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48AttackModule__set_attack_reference_joint_id_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS_19AttackDirectionAxisES5_S5_"]
        pub fn set_attack_reference_joint_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::app::AttackDirectionAxis,
            arg4: root::app::AttackDirectionAxis,
            arg5: root::app::AttackDirectionAxis,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__set_overlap_hit_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_overlap_hit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__disable_tip_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn disable_tip(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AttackModule__relocate_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn relocate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AttackModule__set_lerp_implEPNS_26BattleObjectModuleAccessorEiii"]
        pub fn set_lerp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AttackModule__clear_lerp_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_lerp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33AttackModule__set_lerp_ratio_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_lerp_ratio(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47AttackModule__set_attack_power_mul_pattern_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_attack_power_mul_pattern(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47AttackModule__get_attack_power_mul_pattern_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_power_mul_pattern(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45AttackModule__set_attack_power_mul_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_attack_power_mul_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45AttackModule__get_attack_power_mul_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_power_mul_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_lr_check_default_implEPNS_26BattleObjectModuleAccessorEh"]
        pub fn set_lr_check_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__lr_check_default_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn lr_check_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> libc::c_uchar;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_lr_check_front_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_lr_check_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_lr_check_back_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_lr_check_back(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackModule__set_lr_check_front_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_lr_check_front_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52AttackModule__set_disable_power_add_status_zero_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_disable_power_add_status_zero(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__reset_status_attack_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_status_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__damage_shake_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn damage_shake_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_damage_shake_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_damage_shake_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__set_latest_no_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_latest_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__set_latest_absolute_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_latest_absolute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_reflect_attack_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_reflect_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__is_power_up_reaction_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_power_up_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackModule__set_power_up_reaction_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_power_up_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_attack_scale_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_attack_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_attack_scale_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_attack_scale_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__set_base_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn set_base_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__is_attack_occur_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_attack_occur(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_constraint_pos_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_constraint_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__is_constraint_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_constraint_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48AttackModule__set_no_dead_damage_fly_effect_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_dead_damage_fly_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47AttackModule__is_no_dead_damage_fly_effect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_no_dead_damage_fly_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46AttackModule__set_damage_effect_mul_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_damage_effect_mul_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__damage_effect_mul_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn damage_effect_mul_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_attack_level_implEPNS_26BattleObjectModuleAccessorEih"]
        pub fn set_attack_level(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__set_ink_value_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_ink_value(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_special_paint_implEPNS_26BattleObjectModuleAccessorEiNS_16SpecialPaintKindE"]
        pub fn set_special_paint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::SpecialPaintKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_paralyze_frame_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_paralyze_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_poison_param_implEPNS_26BattleObjectModuleAccessorEiiifb"]
        pub fn set_poison_param(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
            arg5: f32,
            arg6: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_optional_hit_effect_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40E"]
        pub fn set_optional_hit_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_optional_hit_sound_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40E"]
        pub fn set_optional_hit_sound(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44AttackModule__set_no_comp_damage_motion_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_no_comp_damage_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AttackModule__set_no_finish_camera_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_no_finish_camera(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_no_finish_camera_ex_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_no_finish_camera_ex(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_final_finish_cut_in_implEPNS_26BattleObjectModuleAccessorEibbfb"]
        pub fn set_final_finish_cut_in(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
            arg5: f32,
            arg6: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__set_no_dead_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_no_dead_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_no_damage_orbit_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_no_damage_orbit_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48AttackModule__set_captured_same_time_attack_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_captured_same_time_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59AttackModule__set_captured_same_time_attack_damage_mul_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_captured_same_time_attack_damage_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47AttackModule__set_attack_composition_speed_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_attack_composition_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_attack_camera_quake_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_attack_camera_quake(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49AttackModule__set_attack_camera_quake_forced_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_attack_camera_quake_forced(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48AttackModule__set_attack_no_weight_reaction_implEPNS_26BattleObjectModuleAccessorEihb"]
        pub fn set_attack_no_weight_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uchar,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52AttackModule__set_attack_no_weight_reaction_all_implEPNS_26BattleObjectModuleAccessorEhb"]
        pub fn set_attack_no_weight_reaction_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48AttackModule__set_attack_no_reaction_search_implEPNS_26BattleObjectModuleAccessorEihb"]
        pub fn set_attack_no_reaction_search(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uchar,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44AttackModule__clear_inflict_kind_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_inflict_kind_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_force_reaction_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_force_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AttackModule__set_accept_no_lr_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_accept_no_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__set_nearest_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_nearest(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_vec_target_pos_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ERKNS3_8Vector2fEjb"]
        pub fn set_vec_target_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: *const root::phx::Vector2f,
            arg5: libc::c_uint,
            arg6: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__enable_safe_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_safe_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__is_critical_attack_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_critical_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49AttackModule__get_critical_attack_damage_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_critical_attack_damage_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51AttackModule__get_critical_attack_reaction_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_critical_attack_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_is_critical_attack_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_is_critical_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49AttackModule__set_critical_attack_damage_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_critical_attack_damage_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51AttackModule__set_critical_attack_reaction_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_critical_attack_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58AttackModule__enable_attack_r_fix_damage_speed_up_all_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn enable_attack_r_fix_damage_speed_up_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_add_reaction_frame_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_add_reaction_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49AttackModule__set_add_reaction_frame_revised_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_add_reaction_frame_revised(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AttackModule__set_set_off_implEPNS_26BattleObjectModuleAccessorEiNS_16AttackSetOffKindE"]
        pub fn set_set_off(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::AttackSetOffKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51AttackModule__set_no_squat_damage_reaction_mul_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_no_squat_damage_reaction_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46AttackModule__set_no_damage_fly_smoke_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_no_damage_fly_smoke_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41AttackModule__set_no_uniq_effect_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_no_uniq_effect_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AttackModule__set_no_world_scale_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_world_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_attack_height_implEPNS_26BattleObjectModuleAccessorEiNS_12AttackHeightEb"]
        pub fn set_attack_height(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::AttackHeight,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackModule__set_attack_height_all_implEPNS_26BattleObjectModuleAccessorENS_12AttackHeightEb"]
        pub fn set_attack_height_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::AttackHeight,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AttackModule__set_no_hop_opponent_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn set_no_hop_opponent(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42AttackModule__set_no_hop_opponent_all_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_no_hop_opponent_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackModule__set_ice_frame_mul_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_ice_frame_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AttackModule__init_attack_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn init_attack_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackModule__set_down_only_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_down_only(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47AttackModule__resume_catch_absolute_damage_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn resume_catch_absolute_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor
        );
    }
}
pub mod FighterCutInManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39FighterCutInManager__request_start_implEPNS_19FighterCutInManagerERNS_26BattleObjectModuleAccessorEPNS_15CutInTransactorENS_9CutInTypeEPKNS_9CutInDataENS_13CutInPriorityE"]
        pub fn request_start(
            arg1: *mut root::app::FighterCutInManager,
            arg2: *mut root::app::BattleObjectModuleAccessor,
            arg3: *mut root::app::CutInTransactor,
            arg4: root::app::CutInType,
            arg5: *const root::app::CutInData,
            arg6: root::app::CutInPriority,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterCutInManager__request_end_implEPNS_19FighterCutInManagerE"]
        pub fn request_end(arg1: *mut root::app::FighterCutInManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterCutInManager__is_owner_implEPNS_19FighterCutInManagerERKNS_26BattleObjectModuleAccessorE"]
        pub fn is_owner(
            arg1: *mut root::app::FighterCutInManager,
            arg2: *const root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterCutInManager__is_play_implEPNS_19FighterCutInManagerE"]
        pub fn is_play(arg1: *mut root::app::FighterCutInManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40FighterCutInManager__is_play_status_implEPNS_19FighterCutInManagerE"]
        pub fn is_play_status(arg1: *mut root::app::FighterCutInManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterCutInManager__add_task_implEPNS_19FighterCutInManagerEj"]
        pub fn add_task(
            arg1: *mut root::app::FighterCutInManager,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterCutInManager__set_throw_finish_zoom_rate_implEPNS_19FighterCutInManagerEf"]
        pub fn set_throw_finish_zoom_rate(
            arg1: *mut root::app::FighterCutInManager,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterCutInManager__set_throw_finish_offset_implEPNS_19FighterCutInManagerEN3phx8Vector3fE"]
        pub fn set_throw_finish_offset(
            arg1: *mut root::app::FighterCutInManager,
            arg2: root::phx::Vector3f,
        );
    }
}
pub mod GimmickEventWarp {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42GimmickEventWarp__load_from_l2c_table_implEPNS_16GimmickEventWarpERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventWarp,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GimmickEventWarp__store_l2c_table_implEPKNS_16GimmickEventWarpE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventWarp) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GimmickEventWarp__store_l2c_table_implEPKNS_16GimmickEventWarpERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventWarp,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventDrumCheckNeedHide {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55GimmickEventDrumCheckNeedHide__load_from_l2c_table_implEPNS_29GimmickEventDrumCheckNeedHideERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumCheckNeedHide,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventDrumCheckNeedHide__store_l2c_table_implEPKNS_29GimmickEventDrumCheckNeedHideE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumCheckNeedHide,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventDrumCheckNeedHide__store_l2c_table_implEPKNS_29GimmickEventDrumCheckNeedHideERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumCheckNeedHide,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod Circle {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32Circle__load_from_l2c_table_implEPNS_6CircleERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::Circle,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28Circle__store_l2c_table_implEPKNS_6CircleE"]
        pub fn store_l2c_table(arg1: *const root::app::Circle) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28Circle__store_l2c_table_implEPKNS_6CircleERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::Circle,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterInklingLinkEventPaint {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterInklingLinkEventPaint__load_from_l2c_table_implEPNS_28FighterInklingLinkEventPaintERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterInklingLinkEventPaint,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterInklingLinkEventPaint__store_l2c_table_implEPKNS_28FighterInklingLinkEventPaintE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterInklingLinkEventPaint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterInklingLinkEventPaint__store_l2c_table_implEPKNS_28FighterInklingLinkEventPaintERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterInklingLinkEventPaint,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    /// extern "C" {
    ///     #[link_name = "\u{1}_ZN3app28FighterInklingLinkEventPaint13new_l2c_tableEv"]
    ///     pub fn new_l2c_table()-> u64; 
    ///     //This function isn't actually part of lua_bind::FighterInklingLinkEventPaint, but for now I'm just trying to test something and it won't build given its own module. 
    /// }
}
pub mod FighterControlModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterControlModuleImpl__reserve_on_attack_button_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reserve_on_attack_button(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterControlModuleImpl__get_attack_s3_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_s3_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterControlModuleImpl__get_attack_s4_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attack_s4_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterControlModuleImpl__get_special_s_turn_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_special_s_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53FighterControlModuleImpl__update_attack_air_kind_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn update_attack_air_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterControlModuleImpl__get_param_dash_s4_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_param_dash_s4_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59FighterControlModuleImpl__get_param_attack_hi4_flick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_param_attack_hi4_flick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59FighterControlModuleImpl__get_param_attack_lw4_flick_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_param_attack_lw4_flick_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58FighterControlModuleImpl__special_command_236236_step_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn special_command_236236_step(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58FighterControlModuleImpl__special_command_214214_step_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn special_command_214214_step(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterControlModuleImpl__special_command_21416_step_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn special_command_21416_step(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterControlModuleImpl__special_command_23634_step_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn special_command_23634_step(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59FighterControlModuleImpl__check_hit_stop_delay_command_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector2fE"]
        pub fn check_hit_stop_delay_command(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterControlModuleImpl__is_enable_hit_stop_delay_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_hit_stop_delay(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60FighterControlModuleImpl__is_enable_hit_stop_delay_life_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_hit_stop_delay_life(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterControlModuleImpl__get_stick_button_trigger_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_stick_button_trigger(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterControlModuleImpl__get_stick_button_repeat_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_stick_button_repeat(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
}
pub mod FighterRyuLinkEventFinalDeadDamage {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60FighterRyuLinkEventFinalDeadDamage__load_from_l2c_table_implEPNS_34FighterRyuLinkEventFinalDeadDamageERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterRyuLinkEventFinalDeadDamage,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56FighterRyuLinkEventFinalDeadDamage__store_l2c_table_implEPKNS_34FighterRyuLinkEventFinalDeadDamageE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterRyuLinkEventFinalDeadDamage,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56FighterRyuLinkEventFinalDeadDamage__store_l2c_table_implEPKNS_34FighterRyuLinkEventFinalDeadDamageERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterRyuLinkEventFinalDeadDamage,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterMotionModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterMotionModuleImpl__add_body_type_hash_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn add_body_type_hash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        /// Returns the earliest actionable frame of the specified motion
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'motion_kind' - a hash of a motion_kind
        ///
        /// # Example
        ///
        /// ```
        /// // allow characters to act of of back aerial 2 frames sooner than normal
        /// if MotionModule::get_cancel_frame(module_accessor, Hash40::new("attack_air_b")) - MotionModule::frame(module_accessor) <= 2 {
        ///     CancelModule::enable_cancel(module_accessor);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterMotionModuleImpl__get_cancel_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn get_cancel_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterMotionModuleImpl__is_valid_cancel_frame_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn is_valid_cancel_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterMotionModuleImpl__get_hit_normal_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn get_hit_normal_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47FighterMotionModuleImpl__get_hit_xlu_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn get_hit_xlu_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind61FighterMotionModuleImpl__motion_kind_kirby_copy_original_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn motion_kind_kirby_copy_original(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterMotionModuleImpl__change_motion_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb"]
        pub fn change_motion_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: bool,
            arg6: f32,
            arg7: bool,
            arg8: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterMotionModuleImpl__change_motion_inherit_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efffbb"]
        pub fn change_motion_inherit_frame_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
            arg6: bool,
            arg7: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind78FighterMotionModuleImpl__change_motion_inherit_frame_keep_rate_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff"]
        pub fn change_motion_inherit_frame_keep_rate_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind74FighterMotionModuleImpl__change_motion_force_inherit_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff"]
        pub fn change_motion_force_inherit_frame_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60FighterMotionModuleImpl__end_frame_from_hash_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn end_frame_from_hash_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind64FighterMotionModuleImpl__set_frame_sync_anim_cmd_kirby_copy_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_frame_sync_anim_cmd_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterMotionModuleImpl__get_cancel_frame_kirby_copy_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn get_cancel_frame_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59FighterMotionModuleImpl__add_motion_partial_kirby_copy_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Effbbfbbb"]
        pub fn add_motion_partial_kirby_copy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: f32,
            arg5: f32,
            arg6: bool,
            arg7: bool,
            arg8: f32,
            arg9: bool,
            arg10: bool,
            arg11: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45FighterMotionModuleImpl__set_blend_waist_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_blend_waist(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind61FighterMotionModuleImpl__start_damage_stop_interpolation_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn start_damage_stop_interpolation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind65FighterMotionModuleImpl__set_pause_motion_interpolation_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_pause_motion_interpolation_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterMotionModuleImpl__set_update_finger_and_face_joint_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_update_finger_and_face_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}
pub mod FighterStopModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterStopModuleImpl__is_damage_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_damage_stop(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterStopModuleImpl__get_damage_stop_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_damage_stop_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
}
pub mod WeaponShizueFishingrodLinkEventReel {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind61WeaponShizueFishingrodLinkEventReel__load_from_l2c_table_implEPNS_35WeaponShizueFishingrodLinkEventReelERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponShizueFishingrodLinkEventReel,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57WeaponShizueFishingrodLinkEventReel__store_l2c_table_implEPKNS_35WeaponShizueFishingrodLinkEventReelE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventReel,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57WeaponShizueFishingrodLinkEventReel__store_l2c_table_implEPKNS_35WeaponShizueFishingrodLinkEventReelERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventReel,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventBarrel {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventBarrel__load_from_l2c_table_implEPNS_18GimmickEventBarrelERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventBarrel,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventBarrel__store_l2c_table_implEPKNS_18GimmickEventBarrelE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventBarrel)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventBarrel__store_l2c_table_implEPKNS_18GimmickEventBarrelERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventBarrel,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ArticleModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ArticleModule__generate_article_implEPNS_26BattleObjectModuleAccessorEibi"]
        pub fn generate_article(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ArticleModule__generate_article_enable_implEPNS_26BattleObjectModuleAccessorEibi"]
        pub fn generate_article_enable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ArticleModule__generate_article_have_item_implEPNS_26BattleObjectModuleAccessorEiiN3phx6Hash40E"]
        pub fn generate_article_have_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24ArticleModule__have_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetEjb"]
        pub fn have(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::app::ArticleOperationTarget,
            arg5: libc::c_uint,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ArticleModule__shoot_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetEb"]
        pub fn shoot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ArticleModule__shoot_exist_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetEb"]
        pub fn shoot_exist(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ArticleModule__set_pos_implEPNS_26BattleObjectModuleAccessorEiN3phx8Vector3fE"]
        pub fn set_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ArticleModule__motion_kind_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE"]
        pub fn motion_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ArticleModule__change_motion_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ebf"]
        pub fn change_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: bool,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ArticleModule__change_motion_from_no_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Eibf"]
        pub fn change_motion_from_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: libc::c_int,
            arg5: bool,
            arg6: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ArticleModule__add_motion_2nd_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Effbf"]
        pub fn add_motion_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: f32,
            arg5: f32,
            arg6: bool,
            arg7: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ArticleModule__motion_kind_2nd_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE"]
        pub fn motion_kind_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ArticleModule__set_weight_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ArticleModule__add_motion_partial_implEPNS_26BattleObjectModuleAccessorEiiN3phx6Hash40Effbbfbbb"]
        pub fn add_motion_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: root::phx::Hash40,
            arg5: f32,
            arg6: f32,
            arg7: bool,
            arg8: bool,
            arg9: f32,
            arg10: bool,
            arg11: bool,
            arg12: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ArticleModule__set_frame_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ArticleModule__set_frame_2nd_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_frame_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ArticleModule__set_rate_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ArticleModule__change_status_implEPNS_26BattleObjectModuleAccessorEiiNS_22ArticleOperationTargetE"]
        pub fn change_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: root::app::ArticleOperationTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ArticleModule__change_status_exist_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn change_status_exist(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ArticleModule__set_visibility_whole_implEPNS_26BattleObjectModuleAccessorEibNS_22ArticleOperationTargetE"]
        pub fn set_visibility_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: root::app::ArticleOperationTarget,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ArticleModule__set_visibility_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ES4_NS_22ArticleOperationTargetE"]
        pub fn set_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::phx::Hash40,
            arg5: root::app::ArticleOperationTarget,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ArticleModule__set_default_visibility_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetE"]
        pub fn set_default_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::app::ArticleOperationTarget,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ArticleModule__set_situation_kind_implEPNS_26BattleObjectModuleAccessorEih"]
        pub fn set_situation_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ArticleModule__set_flag_implEPNS_26BattleObjectModuleAccessorEibi"]
        pub fn set_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ArticleModule__is_flag_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn is_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ArticleModule__is_flag_from_no_implEPNS_26BattleObjectModuleAccessorEiii"]
        pub fn is_flag_from_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ArticleModule__set_int_implEPNS_26BattleObjectModuleAccessorEiii"]
        pub fn set_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ArticleModule__get_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ArticleModule__set_float_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ArticleModule__get_float_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ArticleModule__set_item_action_implEPNS_26BattleObjectModuleAccessorEiif"]
        pub fn set_item_action(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ArticleModule__remove_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE"]
        pub fn remove(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ArticleModule__remove_exist_implEPNS_26BattleObjectModuleAccessorEiNS_22ArticleOperationTargetE"]
        pub fn remove_exist(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ArticleOperationTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ArticleModule__remove_exist_object_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn remove_exist_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ArticleModule__is_exist_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_exist(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ArticleModule__get_num_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ArticleModule__is_generatable_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_generatable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ArticleModule__get_active_num_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_active_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ArticleModule__get_article_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_article(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> *mut root::app::Article;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ArticleModule__get_joint_pos_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetE"]
        pub fn get_joint_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::app::ArticleOperationTarget,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ArticleModule__get_joint_rotate_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ENS_22ArticleOperationTargetE"]
        pub fn get_joint_rotate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::app::ArticleOperationTarget,
        ) -> u64;
    }
}
pub mod LinkEventThrow {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventThrow__load_from_l2c_table_implEPNS_14LinkEventThrowERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventThrow,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkEventThrow__store_l2c_table_implEPKNS_14LinkEventThrowE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventThrow) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkEventThrow__store_l2c_table_implEPKNS_14LinkEventThrowERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventThrow,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod CatchModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CatchModule__set_catch_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_catch(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26CatchModule__is_catch_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_catch(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CatchModule__catch_cut_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn catch_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34CatchModule__update_pos_cling_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_pos_cling(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CatchModule__cling_cut_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn cling_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30CatchModule__check_damage_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn check_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CatchModule__set_send_cut_event_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_send_cut_event(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CatchModule__capture_object_id_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn capture_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CatchModule__capture_pos_x_diff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn capture_pos_x_diff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
}
pub mod DamageLog {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35DamageLog__load_from_l2c_table_implEPNS_9DamageLogERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::DamageLog,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31DamageLog__store_l2c_table_implEPKNS_9DamageLogE"]
        pub fn store_l2c_table(arg1: *const root::app::DamageLog) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31DamageLog__store_l2c_table_implEPKNS_9DamageLogERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::DamageLog,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ShieldModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24ShieldModule__clean_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clean(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ShieldModule__is_shield_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn is_shield(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShieldModule__set_size_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ShieldModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_12ShieldStatusEi"]
        pub fn set_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ShieldStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ShieldModule__set_status_all_implEPNS_26BattleObjectModuleAccessorENS_12ShieldStatusEi"]
        pub fn set_status_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ShieldModule__is_turn_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShieldModule__is_front_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ShieldModule__is_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShieldModule__get_hop_angle_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_hop_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ShieldModule__is_no_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShieldModule__set_turn_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ShieldModule__set_front_implEPNS_26BattleObjectModuleAccessorENS_11ShieldFrontEi"]
        pub fn set_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldFront,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ShieldModule__set_hop_implEPNS_26BattleObjectModuleAccessorEbfi"]
        pub fn set_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ShieldModule__set_attack_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ShieldModule__get_attack_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShieldModule__set_speed_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShieldModule__get_speed_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ShieldModule__set_life_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ShieldModule__get_life_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ShieldModule__set_attack_limit_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ShieldModule__get_attack_limit_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ShieldModule__set_hit_stop_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_hit_stop_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ShieldModule__is_no_m_ball_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_m_ball(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShieldModule__get_part_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_part_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ShieldModule__get_team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ShieldModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ShieldModule__set_shield_type_implEPNS_26BattleObjectModuleAccessorENS_10ShieldTypeEii"]
        pub fn set_shield_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldType,
            arg3: libc::c_int,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ShieldModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ShieldModule__clear_all_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_all_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ShieldModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ShieldModule__get_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShieldModule__get_group_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_group_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ShieldModule__get_pos_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_pos_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ShieldModule__set_target_property_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_property(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ShieldModule__set_target_category_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_category(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24ShieldModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
}
pub mod FighterRyuLinkEventFinalMoveTarget {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60FighterRyuLinkEventFinalMoveTarget__load_from_l2c_table_implEPNS_34FighterRyuLinkEventFinalMoveTargetERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterRyuLinkEventFinalMoveTarget,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56FighterRyuLinkEventFinalMoveTarget__store_l2c_table_implEPKNS_34FighterRyuLinkEventFinalMoveTargetE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterRyuLinkEventFinalMoveTarget,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56FighterRyuLinkEventFinalMoveTarget__store_l2c_table_implEPKNS_34FighterRyuLinkEventFinalMoveTargetERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterRyuLinkEventFinalMoveTarget,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod DamageInfo {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36DamageInfo__load_from_l2c_table_implEPNS_10DamageInfoERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::DamageInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32DamageInfo__store_l2c_table_implEPKNS_10DamageInfoE"]
        pub fn store_l2c_table(arg1: *const root::app::DamageInfo) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32DamageInfo__store_l2c_table_implEPKNS_10DamageInfoERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::DamageInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod PostureModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__init_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEbb"]
        pub fn init_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23PostureModule__pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> *const root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26PostureModule__pos_2d_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_2d(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> root::phx::Vector2f;
    }
    extern "C" {
        /// Returns stage X position of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // is on right side of the stage
        /// if PostureModule::pos_x(module_accessor) > 0.0 {
        ///     // ...
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__pos_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_x(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        /// Returns stage Y position of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // is above Final Destination stage level
        /// if PostureModule::pos_y(module_accessor) > 0.0 {
        ///     // ...
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__pos_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_y(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        /// Returns stage Z position of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__pos_z_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_z(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27PostureModule__set_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__set_pos_2d_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn set_pos_2d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27PostureModule__add_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn add_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__add_pos_2d_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn add_pos_2d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__prev_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn prev_pos(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> *const root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PostureModule__prev_pos_2d_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn prev_pos_2d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::phx::Vector2f;
    }
    extern "C" {
        /// Returns the current direction the current battle object is facing. -1 = left, 1 = right
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        #[link_name = "\u{1}_ZN3app8lua_bind22PostureModule__lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn lr(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        /// Sets the direction faced for the current battle object.
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'facing_dir' - The direction to set the character to... -1 = left, 1 = right
        #[link_name = "\u{1}_ZN3app8lua_bind26PostureModule__set_lr_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_lr(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        /// Reverses the direction that the current battle object is facing
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__reverse_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reverse_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__init_rot_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn init_rot(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__init_rot_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn init_rot_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23PostureModule__rot_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn rot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__rot_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn rot_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__rot_y_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn rot_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__rot_z_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn rot_z(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27PostureModule__set_rot_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn set_rot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33PostureModule__init_rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn init_rot_y_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50PostureModule__is_rot_y_lr_different_inner_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_rot_y_lr_different_inner_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rot_y_lr(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        /// Updates the current battle object's orientation. Usually used in conjunction with PostureModule::set_lr or PostureModule::reverse_lr
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        #[link_name = "\u{1}_ZN3app8lua_bind35PostureModule__update_rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_rot_y_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36PostureModule__reverse_rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reverse_rot_y_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__base_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn base_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25PostureModule__scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn scale(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29PostureModule__set_scale_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30PostureModule__init_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn init_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PostureModule__owner_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn owner_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PostureModule__set_owner_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_owner_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PostureModule__set_link_scale_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_link_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32PostureModule__set_stick_lr_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_stick_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45PostureModule__set_sync_constraint_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_sync_constraint_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
}
pub mod LinkEventTouchItem {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkEventTouchItem__load_from_l2c_table_implEPNS_18LinkEventTouchItemERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventTouchItem,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventTouchItem__store_l2c_table_implEPKNS_18LinkEventTouchItemE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventTouchItem)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventTouchItem__store_l2c_table_implEPKNS_18LinkEventTouchItemERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventTouchItem,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventCatch {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GimmickEventCatch__load_from_l2c_table_implEPNS_17GimmickEventCatchERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventCatch,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39GimmickEventCatch__store_l2c_table_implEPKNS_17GimmickEventCatchE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventCatch) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39GimmickEventCatch__store_l2c_table_implEPKNS_17GimmickEventCatchERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventCatch,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod MotionModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__is_changing_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_changing(
            module_accessor: *mut root::app::BattleObjectModuleAccessor
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__change_motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbfbb"]
        pub fn change_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: bool,
            arg6: f32,
            arg7: bool,
            arg8: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46MotionModule__change_motion_inherit_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efffbb"]
        pub fn change_motion_inherit_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
            arg6: bool,
            arg7: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56MotionModule__change_motion_inherit_frame_keep_rate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff"]
        pub fn change_motion_inherit_frame_keep_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52MotionModule__change_motion_force_inherit_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Efff"]
        pub fn change_motion_force_inherit_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__change_motion_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn change_motion_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33MotionModule__add_motion_2nd_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbf"]
        pub fn add_motion_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
            arg4: f32,
            arg5: bool,
            arg6: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__set_weight_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionModule__set_weight_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_weight_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27MotionModule__is_blend_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_blend(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        /// returns the hash of the current motion (animation)
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // make up air universally cancelable
        /// let curr_motion_kind = MotionModule::motion_kind(module_accessor);
        /// if curr_motion_kind == smash::hash40("attack_air_b") {
        ///     CancelModule::enable_cancel(module_accessor);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__motion_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn motion_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23MotionModule__rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rate(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27MotionModule__set_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_rate(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27MotionModule__rate_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rate_2nd(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionModule__set_rate_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_rate_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        /// Returns the current frame of the battle object's current animation as a float.
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // if your are doing an aerial and your current frame is 5 or fewer frames away from the end of the anim, transition to FALL
        /// if MotionModule::end_frame(module_accessor) - MotionModule::frame(module_accessor) <= 5 {
        ///     StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind24MotionModule__frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn frame(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__prev_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn prev_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28MotionModule__set_frame_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42MotionModule__set_frame_sync_anim_cmd_implEPNS_26BattleObjectModuleAccessorEfbbb"]
        pub fn set_frame_sync_anim_cmd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
            arg4: bool,
            arg5: bool,
        );
    }
    extern "C" {
        /// Returns the total amount of frames of the battle object's current animation as a float.
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // if your are doing an aerial and your current frame is 5 or fewer frames away from the end of the anim, transition to FALL
        /// if MotionModule::end_frame(module_accessor) - MotionModule::frame(module_accessor) <= 5 {
        ///     StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind28MotionModule__end_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_frame(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38MotionModule__end_frame_from_hash_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn end_frame_from_hash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__set_frame_2nd_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_frame_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28MotionModule__frame_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn frame_2nd(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28MotionModule__trans_tra_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fEbb"]
        pub fn trans_tra(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector3f,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__trans_tra_2nd_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fEbb"]
        pub fn trans_tra_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector3f,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionModule__joint_local_tra_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EbRNS3_8Vector3fE"]
        pub fn joint_local_tra(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38MotionModule__trans_tra_end_frame_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn trans_tra_end_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25MotionModule__is_end_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_end(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28MotionModule__is_looped_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_looped(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionModule__is_loop_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_loop_flag(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42MotionModule__update_trans_move_speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_trans_move_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35MotionModule__trans_move_speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn trans_move_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> nnsdk::root::nn::util::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39MotionModule__trans_move_speed_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn trans_move_speed_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> nnsdk::root::nn::util::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48MotionModule__set_trans_move_speed_no_scale_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_trans_move_speed_no_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35MotionModule__is_anim_resource_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_anim_resource(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__resource_id_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn resource_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36MotionModule__animcmd_name_hash_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn animcmd_name_hash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25MotionModule__weight_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn weight(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__prev_weight_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn prev_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43MotionModule__set_weight_change_motion_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_weight_change_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__calc_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_RNS3_8Vector3fE"]
        pub fn calc_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66MotionModule__update_dynamic_skeleton_without_complete_matrix_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_dynamic_skeleton_without_complete_matrix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39MotionModule__joint_local_rotation_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn joint_local_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__set_trans_joint_id_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_trans_joint_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__end_frame_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_frame_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__is_end_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_end_2nd(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionModule__motion_kind_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn motion_kind_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionModule__is_flag_link_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_flag_link(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57MotionModule__is_flag_start_1_frame_from_motion_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_flag_start_1_frame_from_motion_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40MotionModule__is_flag_start_1_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_flag_start_1_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44MotionModule__is_flag_start_1_frame_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_flag_start_1_frame_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionModule__joint_rotate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn joint_rotate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__add_motion_partial_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Effbbfbbb"]
        pub fn add_motion_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: f32,
            arg5: f32,
            arg6: bool,
            arg7: bool,
            arg8: f32,
            arg9: bool,
            arg10: bool,
            arg11: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40MotionModule__remove_motion_partial_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn remove_motion_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45MotionModule__remove_motion_partial_comp_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn remove_motion_partial_comp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38MotionModule__motion_kind_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn motion_kind_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33MotionModule__is_end_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_end_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36MotionModule__end_frame_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn end_frame_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35MotionModule__set_rate_partial_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_rate_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31MotionModule__rate_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn rate_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__frame_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn frame_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__prev_frame_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn prev_frame_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36MotionModule__set_frame_partial_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_frame_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50MotionModule__set_frame_partial_sync_anim_cmd_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_frame_partial_sync_anim_cmd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58MotionModule__set_frame_partial_sync_anim_cmd_revised_implEPNS_26BattleObjectModuleAccessorEifb"]
        pub fn set_frame_partial_sync_anim_cmd_revised(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39MotionModule__set_part_animcmd_fix_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_part_animcmd_fix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48MotionModule__is_flag_start_1_frame_partial_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_flag_start_1_frame_partial(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26MotionModule__is_flip_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_flip(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27MotionModule__set_flip_implEPNS_26BattleObjectModuleAccessorEbbb"]
        pub fn set_flip(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45MotionModule__set_link_flip_target_joint_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_link_flip_target_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__set_frame_material_implEPNS_26BattleObjectModuleAccessorEfNS_17MaterialAnimeKindE"]
        pub fn set_frame_material(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: root::app::MaterialAnimeKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36MotionModule__set_rate_material_implEPNS_26BattleObjectModuleAccessorEfNS_17MaterialAnimeKindE"]
        pub fn set_rate_material(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: root::app::MaterialAnimeKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41MotionModule__set_loop_flag_material_implEPNS_26BattleObjectModuleAccessorENS_21MaterialAnimeLoopFlagENS_17MaterialAnimeKindE"]
        pub fn set_loop_flag_material(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::MaterialAnimeLoopFlag,
            arg3: root::app::MaterialAnimeKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37MotionModule__get_frame_material_implEPNS_26BattleObjectModuleAccessorENS_17MaterialAnimeKindE"]
        pub fn get_frame_material(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::MaterialAnimeKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41MotionModule__set_helper_calculation_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_helper_calculation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__set_skip_rate_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_skip_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__whole_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn whole_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33MotionModule__set_whole_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_whole_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__update_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32MotionModule__set_recalc_ik_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_recalc_ik(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__set_no_comp_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_comp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29MotionModule__is_no_comp_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_no_comp(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionModule__set_kind_offset_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_kind_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50MotionModule__enable_shift_material_animation_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn enable_shift_material_animation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36MotionModule__trans_joint_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn trans_joint_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41MotionModule__set_stop_interpolation_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_stop_interpolation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35MotionModule__is_flip_resource_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_flip_resource(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30MotionModule__set_reverse_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_reverse(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48MotionModule__set_force_progress_2nd_weight_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_force_progress_2nd_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34MotionModule__clear_joint_srt_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn clear_joint_srt(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43MotionModule__start_flip_interpolation_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn start_flip_interpolation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50MotionModule__enable_remove_2nd_change_motion_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn enable_remove_2nd_change_motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
}
pub mod WorkModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__get_float_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__set_float_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__rnd_float_implEPNS_26BattleObjectModuleAccessorEffi"]
        pub fn rnd_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__add_float_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn add_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__sub_float_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn sub_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__mul_float_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn mul_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__div_float_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn div_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__get_int_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__set_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__get_int64_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__set_int64_implEPNS_26BattleObjectModuleAccessorEli"]
        pub fn set_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__rnd_int_implEPNS_26BattleObjectModuleAccessorEiii"]
        pub fn rnd_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__inc_int_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn inc_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__dec_int_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn dec_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__add_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn add_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__sub_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn sub_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__mul_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn mul_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__div_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn div_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31WorkModule__count_down_int_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn count_down_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__is_flag_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24WorkModule__on_flag_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn on_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25WorkModule__off_flag_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn off_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        /// sets the specified flag on/off depending on the specified bool
        /// (I.E. false = off, true = on)
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'on_or_off' - bool to represent the new state of the flag
        ///
        /// * 'work_id_flag' - a WORK_ID_FLAG_ const
        ///
        /// # Example
        ///
        /// ```
        /// // if you are doing an aerial and input a fast fall, set the fast fall flag on (enables fast falls at all points in an aerial)
        ///
        ///             //if your are doing an aerial                                                                                  and you input a fast-fall
        /// if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
        ///     WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind25WorkModule__set_flag_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30WorkModule__turn_off_flag_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn turn_off_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45WorkModule__enable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45WorkModule__unable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44WorkModule__clear_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_transition_term_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48WorkModule__is_enable_transition_term_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_enable_transition_term_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48WorkModule__enable_transition_term_group_ex_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_group_ex(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52WorkModule__enable_transition_term_group_ex_all_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_group_ex_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48WorkModule__unable_transition_term_group_ex_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_group_ex(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52WorkModule__unable_transition_term_group_ex_all_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_group_ex_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42WorkModule__is_enable_transition_term_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_enable_transition_term(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39WorkModule__enable_transition_term_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39WorkModule__unable_transition_term_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38WorkModule__clear_transition_term_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_transition_term(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49WorkModule__is_enable_transition_term_forbid_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_enable_transition_term_forbid(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46WorkModule__enable_transition_term_forbid_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_forbid(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53WorkModule__enable_transition_term_forbid_indivi_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_forbid_indivi(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46WorkModule__unable_transition_term_forbid_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_forbid(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53WorkModule__unable_transition_term_forbid_indivi_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_forbid_indivi(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52WorkModule__enable_transition_term_forbid_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_transition_term_forbid_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52WorkModule__unable_transition_term_forbid_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_transition_term_forbid_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45WorkModule__clear_transition_term_forbid_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_transition_term_forbid(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26WorkModule__clear_all_implEPNS_26BattleObjectModuleAccessorENS_8WorkKindE"]
        pub fn clear_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::WorkKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30WorkModule__get_param_int_implEPNS_26BattleObjectModuleAccessorEmm"]
        pub fn get_param_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_ulong,
            arg3: libc::c_ulong,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32WorkModule__get_param_int64_implEPNS_26BattleObjectModuleAccessorEmm"]
        pub fn get_param_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_ulong,
            arg3: libc::c_ulong,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32WorkModule__get_param_float_implEPNS_26BattleObjectModuleAccessorEmm"]
        pub fn get_param_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_ulong,
            arg3: libc::c_ulong,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33WorkModule__set_customize_no_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_customize_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
}
pub mod Weapon {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27Weapon__get_founder_id_implEPNS_6WeaponE"]
        pub fn get_founder_id(arg1: *mut root::app::Weapon) -> u64;
    }
}
pub mod AttackAbsoluteData {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44AttackAbsoluteData__load_from_l2c_table_implEPNS_18AttackAbsoluteDataERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::AttackAbsoluteData,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackAbsoluteData__store_l2c_table_implEPKNS_18AttackAbsoluteDataE"]
        pub fn store_l2c_table(arg1: *const root::app::AttackAbsoluteData)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40AttackAbsoluteData__store_l2c_table_implEPKNS_18AttackAbsoluteDataERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::AttackAbsoluteData,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod SlopeModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40SlopeModule__update_model_top_angle_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_model_top_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SlopeModule__floor_diff_l_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn floor_diff_l(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SlopeModule__floor_diff_r_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn floor_diff_r(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
}
pub mod StopModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32StopModule__cancel_hit_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn cancel_hit_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31StopModule__set_other_stop_implEPNS_26BattleObjectModuleAccessorEiNS_13StopOtherKindE"]
        pub fn set_other_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::StopOtherKind,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31StopModule__get_other_stop_implEPNS_26BattleObjectModuleAccessorENS_13StopOtherKindE"]
        pub fn get_other_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::StopOtherKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34StopModule__cancel_other_stop_implEPNS_26BattleObjectModuleAccessorENS_13StopOtherKindE"]
        pub fn cancel_other_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::StopOtherKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30StopModule__set_item_stop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_item_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30StopModule__get_item_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_item_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33StopModule__cancel_item_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn cancel_item_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30StopModule__set_link_stop_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn set_link_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24StopModule__is_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_stop(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32StopModule__is_special_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_special_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23StopModule__is_hit_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_hit(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26StopModule__is_damage_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_damage(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24StopModule__is_item_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_item(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25StopModule__is_other_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_other(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40StopModule__get_hit_stop_real_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_hit_stop_real_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35StopModule__set_hit_stop_frame_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_hit_stop_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39StopModule__set_hit_stop_frame_fix_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_hit_stop_frame_fix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33StopModule__set_special_stop_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_special_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25StopModule__end_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_stop(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
}
pub mod OnCalcParamEvent {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42OnCalcParamEvent__load_from_l2c_table_implEPNS_16OnCalcParamEventERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::OnCalcParamEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38OnCalcParamEvent__store_l2c_table_implEPKNS_16OnCalcParamEventE"]
        pub fn store_l2c_table(arg1: *const root::app::OnCalcParamEvent) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38OnCalcParamEvent__store_l2c_table_implEPKNS_16OnCalcParamEventERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::OnCalcParamEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ShakeModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21ShakeModule__req_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EibRKNS3_8Vector2fEffbb"]
        pub fn req(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
            arg5: *const root::phx::Vector2f,
            arg6: f32,
            arg7: f32,
            arg8: bool,
            arg9: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShakeModule__req_time_scale_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EibRKNS3_8Vector2fEffbbif"]
        pub fn req_time_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
            arg5: *const root::phx::Vector2f,
            arg6: f32,
            arg7: f32,
            arg8: bool,
            arg9: bool,
            arg10: libc::c_int,
            arg11: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24ShakeModule__extend_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn extend(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22ShakeModule__stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn stop(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShakeModule__stop_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn stop_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ShakeModule__is_shake_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_shake(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24ShakeModule__offset_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fE"]
        pub fn offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ShakeModule__enable_offset_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector3fE"]
        pub fn enable_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShakeModule__disable_offset_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn disable_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ShakeModule__set_scale_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef"]
        pub fn set_scale_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ShakeModule__set_axis_xy_kind_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn set_axis_xy_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ShakeModule__set_ignore_slow_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_slow(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}
pub mod GimmickEventSlashEscape {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GimmickEventSlashEscape__load_from_l2c_table_implEPNS_23GimmickEventSlashEscapeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventSlashEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GimmickEventSlashEscape__store_l2c_table_implEPKNS_23GimmickEventSlashEscapeE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventSlashEscape,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GimmickEventSlashEscape__store_l2c_table_implEPKNS_23GimmickEventSlashEscapeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventSlashEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ShadowModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShadowModule__set_draw_implEPNS_26BattleObjectModuleAccessorENS_14ShadowDrawFlagEb"]
        pub fn set_draw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShadowDrawFlag,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ShadowModule__set_draw_status_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_draw_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ShadowModule__set_size_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_size(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ShadowModule__set_offset_y_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_offset_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
}
pub mod SoundModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SoundModule__set_position_sub_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_position_sub(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25SoundModule__play_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40EbbbbNS_11SoundModule8enSETypeE"]
        pub fn play_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
            arg5: bool,
            arg6: bool,
            arg7: root::app::enSEType,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SoundModule__play_se_no3d_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb"]
        pub fn play_se_no3d(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SoundModule__play_se_pos_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fEbb"]
        pub fn play_se_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: bool,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__play_status_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebbb"]
        pub fn play_status_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__stop_status_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn stop_status_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__play_sequence_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ebb"]
        pub fn play_sequence(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38SoundModule__set_play_hit_se_flag_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_play_hit_se_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38SoundModule__get_play_hit_se_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_play_hit_se_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__set_no_hit_se_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_hit_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__get_no_hit_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_no_hit_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SoundModule__play_hit_se_implEPNS_26BattleObjectModuleAccessorEfRKNS_10AttackDataEi"]
        pub fn play_hit_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: *const root::app::AttackData,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__play_damage_se_implEPNS_26BattleObjectModuleAccessorEiiN3phx6Hash40ES4_"]
        pub fn play_damage_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: root::phx::Hash40,
            arg5: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27SoundModule__play_step_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn play_step(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37SoundModule__play_step_flippable_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_"]
        pub fn play_step_flippable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33SoundModule__play_landing_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn play_landing_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SoundModule__play_down_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn play_down_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__play_fly_voice_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_"]
        pub fn play_fly_voice(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SoundModule__set_se_vol_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_se_vol(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SoundModule__get_se_vol_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_se_vol(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__set_se_vol_db_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_se_vol_db(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__get_se_vol_db_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_se_vol_db(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35SoundModule__set_auto_se_pitch_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_auto_se_pitch(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35SoundModule__get_auto_se_pitch_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_auto_se_pitch(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35SoundModule__set_se_pitch_cent_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef"]
        pub fn set_se_pitch_cent(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36SoundModule__set_se_pitch_ratio_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ef"]
        pub fn set_se_pitch_ratio(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37SoundModule__set_se_pitch_status_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_se_pitch_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44SoundModule__set_se_pitch_status_handle_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_se_pitch_status_handle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48SoundModule__set_continue_se_at_game_finish_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_continue_se_at_game_finish(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25SoundModule__stop_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej"]
        pub fn stop_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SoundModule__stop_se_all_implEPNS_26BattleObjectModuleAccessorEjbb"]
        pub fn stop_se_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SoundModule__stop_loop_se_all_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn stop_loop_se_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__stop_se_handle_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn stop_se_handle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__stop_all_sound_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn stop_all_sound(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SoundModule__pause_se_all_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn pause_se_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SoundModule__set_se_speed_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_se_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__set_landing_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_landing_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__get_landing_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_landing_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44SoundModule__play_landing_se_registered_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn play_landing_se_registered(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__set_takeout_se_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_takeout_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39SoundModule__set_takeout_se_status_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_takeout_se_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SoundModule__reset_takeout_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_takeout_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33SoundModule__play_takeout_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn play_takeout_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SoundModule__get_takeout_se_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_takeout_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44SoundModule__get_takeout_se_status_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_takeout_se_status_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SoundModule__set_play_inhivit_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ej"]
        pub fn set_play_inhivit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SoundModule__is_playing_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_playing(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35SoundModule__is_playing_status_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn is_playing_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SoundModule__is_playing_voice_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_playing_voice(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40SoundModule__get_common_sound_label_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_common_sound_label(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33SoundModule__play_status_bgm_implEPNS_26BattleObjectModuleAccessorENS_15enStatusBGMTypeE"]
        pub fn play_status_bgm(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::enStatusBGMType,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46SoundModule__set_gamespeed_se_calibration_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_gamespeed_se_calibration(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31SoundModule__set_remain_se_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_remain_se(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}

pub mod Article {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34Article__get_battle_object_id_implEPNS_7ArticleE"]
        pub fn get_battle_object_id(arg1: *mut root::app::Article) -> u64;
    }
}
pub mod GimmickEventDrumPos {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GimmickEventDrumPos__load_from_l2c_table_implEPNS_19GimmickEventDrumPosERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41GimmickEventDrumPos__store_l2c_table_implEPKNS_19GimmickEventDrumPosE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumPos,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41GimmickEventDrumPos__store_l2c_table_implEPKNS_19GimmickEventDrumPosERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventYoshiTamagoDamageEffect {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58LinkEventYoshiTamagoDamageEffect__load_from_l2c_table_implEPNS_32LinkEventYoshiTamagoDamageEffectERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventYoshiTamagoDamageEffect,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54LinkEventYoshiTamagoDamageEffect__store_l2c_table_implEPKNS_32LinkEventYoshiTamagoDamageEffectE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventYoshiTamagoDamageEffect,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54LinkEventYoshiTamagoDamageEffect__store_l2c_table_implEPKNS_32LinkEventYoshiTamagoDamageEffectERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventYoshiTamagoDamageEffect,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminSetFloat {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterPikminLinkEventWeaponPikminSetFloat__load_from_l2c_table_implEPNS_42FighterPikminLinkEventWeaponPikminSetFloatERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminSetFloat,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind64FighterPikminLinkEventWeaponPikminSetFloat__store_l2c_table_implEPKNS_42FighterPikminLinkEventWeaponPikminSetFloatE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetFloat,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind64FighterPikminLinkEventWeaponPikminSetFloat__store_l2c_table_implEPKNS_42FighterPikminLinkEventWeaponPikminSetFloatERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetFloat,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ItemInkPaintModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemInkPaintModuleImpl__apply_link_ink_paint_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn apply_link_ink_paint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterPitBFinalModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterPitBFinalModule__init_implEPNS_22FighterPitBFinalModuleE"]
        pub fn init(arg1: *mut root::app::FighterPitBFinalModule) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40FighterPitBFinalModule__get_hit_num_implEPNS_22FighterPitBFinalModuleEh"]
        pub fn get_hit_num(
            arg1: *mut root::app::FighterPitBFinalModule,
            arg2: libc::c_uchar,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterPitBFinalModule__get_hit_object_id_implEPNS_22FighterPitBFinalModuleEhi"]
        pub fn get_hit_object_id(
            arg1: *mut root::app::FighterPitBFinalModule,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterPitBFinalModule__get_hit_group_implEPNS_22FighterPitBFinalModuleEhi"]
        pub fn get_hit_group(
            arg1: *mut root::app::FighterPitBFinalModule,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39FighterPitBFinalModule__get_hit_no_implEPNS_22FighterPitBFinalModuleEhi"]
        pub fn get_hit_no(
            arg1: *mut root::app::FighterPitBFinalModule,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        ) -> u64;
    }
}
pub mod BattleObjectWorld {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49BattleObjectWorld__gravity_speed_coefficient_implEPNS_17BattleObjectWorldE"]
        pub fn gravity_speed_coefficient(
            arg1: *mut root::app::BattleObjectWorld,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31BattleObjectWorld__scale_z_implEPNS_17BattleObjectWorldE"]
        pub fn scale_z(arg1: *mut root::app::BattleObjectWorld) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35BattleObjectWorld__gravity_pos_implEPNS_17BattleObjectWorldE"]
        pub fn gravity_pos(arg1: *mut root::app::BattleObjectWorld) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41BattleObjectWorld__is_gravity_normal_implEPNS_17BattleObjectWorldE"]
        pub fn is_gravity_normal(arg1: *mut root::app::BattleObjectWorld) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42BattleObjectWorld__is_disable_reverse_implEPNS_17BattleObjectWorldE"]
        pub fn is_disable_reverse(arg1: *mut root::app::BattleObjectWorld) -> bool;
    }
}
pub mod JostleModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29JostleModule__set_status_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_status(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28JostleModule__set_layer_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_layer(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26JostleModule__set_fix_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_fix(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37JostleModule__set_ignore_speed_x_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_speed_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24JostleModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27JostleModule__is_sleep_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sleep(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33JostleModule__jostle_speed_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn jostle_speed_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41JostleModule__speed_attenuation_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn speed_attenuation_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32JostleModule__target_weight_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn target_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24JostleModule__pos_z_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_z(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28JostleModule__set_refer_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_refer(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35JostleModule__set_push_speed_x_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_push_speed_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37JostleModule__set_push_speed_x_2_implEPNS_26BattleObjectModuleAccessorEPKfb"]
        pub fn set_push_speed_x_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31JostleModule__set_priority_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27JostleModule__priority_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn priority(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32JostleModule__set_force_gap_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_force_gap(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38JostleModule__set_ignore_owner_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_ignore_owner_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34JostleModule__ignore_owner_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn ignore_owner_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29JostleModule__reset_area_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28JostleModule__overlap_x_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn overlap_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33JostleModule__set_ignore_pri_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_ignore_pri(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48JostleModule__set_push_speed_x_overlap_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_push_speed_x_overlap_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28JostleModule__area_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn area_kind(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25JostleModule__weight_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn weight(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29JostleModule__set_weight_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_weight(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48JostleModule__set_influence_opponent_weight_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_influence_opponent_weight(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43JostleModule__set_propagate_push_speed_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_propagate_push_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39JostleModule__set_overlap_rate_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_overlap_rate_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27JostleModule__set_team_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
}
pub mod InkPaintModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24InkPaintModule__ink_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn ink(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28InkPaintModule__ink_max_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn ink_max(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28InkPaintModule__add_ink_implEPNS_26BattleObjectModuleAccessorEfj"]
        pub fn add_ink(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28InkPaintModule__set_ink_implEPNS_26BattleObjectModuleAccessorEfj"]
        pub fn set_ink(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30InkPaintModule__reset_ink_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_ink(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34InkPaintModule__get_ink_color_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_ink_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38InkPaintModule__set_special_paint_implEPNS_26BattleObjectModuleAccessorENS_16SpecialPaintKindE"]
        pub fn set_special_paint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::SpecialPaintKind,
        );
    }
}
pub mod CaptureModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CaptureModule__set_nodes_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_f"]
        pub fn set_nodes(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CaptureModule__is_thrown_finish_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_thrown_finish(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40CaptureModule__set_ignore_object_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_ignore_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30CaptureModule__is_capture_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_capture(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CaptureModule__catch_object_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn catch_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CaptureModule__capture_implEPNS_26BattleObjectModuleAccessorEjibi"]
        pub fn capture(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_int,
            arg4: bool,
            arg5: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CaptureModule__capture_2_implEPNS_26BattleObjectModuleAccessorEjiibi"]
        pub fn capture_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_int,
            arg4: libc::c_int,
            arg5: bool,
            arg6: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37CaptureModule__is_catch_hit_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_catch_hit_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CaptureModule__node_offset_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn node_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CaptureModule__catch_pos_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn catch_pos_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CaptureModule__capture_cut_implEPNS_26BattleObjectModuleAccessorEbbb"]
        pub fn capture_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CaptureModule__check_damage_implEPNS_26BattleObjectModuleAccessorEiff"]
        pub fn check_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39CaptureModule__check_damage_thrown_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn check_damage_thrown(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26CaptureModule__thrown_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn thrown(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CaptureModule__is_thrown_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_thrown(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37CaptureModule__update_pos_thrown_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_pos_thrown(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CaptureModule__update_lr_thrown_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_lr_thrown(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30CaptureModule__thrown_cut_implEPNS_26BattleObjectModuleAccessorEbb"]
        pub fn thrown_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38CaptureModule__set_send_cut_event_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_send_cut_event(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34CaptureModule__set_update_pos_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_update_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CaptureModule__is_reaction_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_reaction(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33CaptureModule__motion_offset_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn motion_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CaptureModule__motion_offset_lw_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn motion_offset_lw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26CaptureModule__motion_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn motion(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CaptureModule__motion_lw_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn motion_lw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38CaptureModule__motion_offset_lw_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn motion_offset_lw_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39CaptureModule__set_ignore_catching_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_ignore_catching(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38CaptureModule__is_ignore_catching_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_ignore_catching(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CaptureModule__is_motion_hi_lw_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_motion_hi_lw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38CaptureModule__is_ignore_distance_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_ignore_distance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CaptureModule__capture_node_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn capture_node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43CaptureModule__set_capture_node_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_capture_node_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38CaptureModule__capture_node_value_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn capture_node_value(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CaptureModule__update_node_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_node_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CaptureModule__catch_node_pos_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn catch_node_pos_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50CaptureModule__capture_to_catch_node_pos_diff_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn capture_to_catch_node_pos_diff(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterAreaModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterAreaModuleImpl__enable_fix_jostle_area_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn enable_fix_jostle_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53FighterAreaModuleImpl__enable_fix_jostle_area_xy_implEPNS_26BattleObjectModuleAccessorEffff"]
        pub fn enable_fix_jostle_area_xy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
            arg5: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterAreaModuleImpl__disable_fix_jostle_area_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn disable_fix_jostle_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod GrabModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26GrabModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22GrabModule__clear_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31GrabModule__set_constraint_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_constraint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24GrabModule__is_grab_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_grab(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21GrabModule__size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25GrabModule__set_size_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22GrabModule__pos_x_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn pos_x(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27GrabModule__center_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21GrabModule__node_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GrabModule__set_power_up_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_power_up(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29GrabModule__set_size_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_size_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GrabModule__set_scale_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_scale_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22GrabModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25GrabModule__relocate_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn relocate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32GrabModule__set_check_front_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_check_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28GrabModule__set_rebound_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_rebound(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27GrabModule__is_rebound_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_rebound(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30GrabModule__check_rebound_implEPNS_26BattleObjectModuleAccessorERKS1_"]
        pub fn check_rebound(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterBayonettaFinalModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44FighterBayonettaFinalModule__final_init_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_init(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44FighterBayonettaFinalModule__final_exec_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_exec(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44FighterBayonettaFinalModule__final_exit_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_exit(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterBayonettaFinalModule__final_start_init_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_start_init(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterBayonettaFinalModule__final_start_exec_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_start_exec(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterBayonettaFinalModule__final_start_exit_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_start_exit(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterBayonettaFinalModule__final_scene01_init_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_scene01_init(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterBayonettaFinalModule__final_scene01_exec_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_scene01_exec(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterBayonettaFinalModule__final_scene01_exit_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_scene01_exit(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterBayonettaFinalModule__final_end_init_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_end_init(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterBayonettaFinalModule__final_end_exec_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_end_exec(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterBayonettaFinalModule__final_end_exit_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn final_end_exit(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterBayonettaFinalModule__restart_other_fighter_implEPNS_27FighterBayonettaFinalModuleERNS_26BattleObjectModuleAccessorE"]
        pub fn restart_other_fighter(
            arg1: *mut root::app::FighterBayonettaFinalModule,
            arg2: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod ItemParamAccessor {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ItemParamAccessor__boss_common_param_int_implEPNS_17ItemParamAccessorENS_8ItemKindENS_18BossCommonParamIntE"]
        pub fn boss_common_param_int(
            arg1: *mut root::app::ItemParamAccessor,
            arg2: root::app::ItemKind,
            arg3: root::app::BossCommonParamInt,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ItemParamAccessor__is_valid_self_param_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub fn is_valid_self_param(
            arg1: *mut root::app::ItemParamAccessor,
            arg2: root::app::ItemKind,
            arg3: root::phx::Hash40,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemParamAccessor__get_self_param_float_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub fn get_self_param_float(
            arg1: *mut root::app::ItemParamAccessor,
            arg2: root::app::ItemKind,
            arg3: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ItemParamAccessor__get_self_param_int_implEPNS_17ItemParamAccessorENS_8ItemKindEN3phx6Hash40E"]
        pub fn get_self_param_int(
            arg1: *mut root::app::ItemParamAccessor,
            arg2: root::app::ItemKind,
            arg3: root::phx::Hash40,
        ) -> u64;
    }
}
pub mod Item {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24Item__start_inhaled_implEPNS_4ItemE"]
        pub fn start_inhaled(arg1: *mut root::app::Item) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21Item__end_hooked_implEPNS_4ItemE"]
        pub fn end_hooked(arg1: *mut root::app::Item) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31Item__get_battle_object_id_implEPNS_4ItemE"]
        pub fn get_battle_object_id(arg1: *mut root::app::Item) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind17Item__action_implEPNS_4ItemEif"]
        pub fn action(
            arg1: *mut root::app::Item,
            arg2: libc::c_int,
            arg3: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31Item__item_module_accessor_implEPNS_4ItemE"]
        pub fn item_module_accessor(arg1: *mut root::app::Item) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29Item__send_touch_message_implEPNS_4ItemEjRKN3phx8Vector3fEf"]
        pub fn send_touch_message(
            arg1: *mut root::app::Item,
            arg2: libc::c_uint,
            arg3: *const root::phx::Vector3f,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29Item__common_param_float_implEPNS_4ItemENS_20ItemCommonParamFloatE"]
        pub fn common_param_float(
            arg1: *mut root::app::Item,
            arg2: root::app::ItemCommonParamFloat,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27Item__common_param_int_implEPNS_4ItemENS_18ItemCommonParamIntE"]
        pub fn common_param_int(
            arg1: *mut root::app::Item,
            arg2: root::app::ItemCommonParamInt,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34Item__specialized_param_float_implEPNS_4ItemENS_25ItemSpecializedParamFloatE"]
        pub fn specialized_param_float(
            arg1: *mut root::app::Item,
            arg2: root::app::ItemSpecializedParamFloat,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32Item__specialized_param_int_implEPNS_4ItemENS_23ItemSpecializedParamIntE"]
        pub fn specialized_param_int(
            arg1: *mut root::app::Item,
            arg2: root::app::ItemSpecializedParamInt,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind19Item__owner_id_implEPNS_4ItemE"]
        pub fn owner_id(arg1: *mut root::app::Item) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind17Item__is_had_implEPNS_4ItemEb"]
        pub fn is_had(arg1: *mut root::app::Item, arg2: bool) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21Item__is_eatable_implEPNS_4ItemEj"]
        pub fn is_eatable(arg1: *mut root::app::Item, arg2: libc::c_uint) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23Item__throw_attack_implEPNS_4ItemEfRKN3phx8Vector3fEf"]
        pub fn throw_attack(
            arg1: *mut root::app::Item,
            arg2: f32,
            arg3: *const root::phx::Vector3f,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind15Item__fall_implEPNS_4ItemEfRKN3phx8Vector3fE"]
        pub fn fall(
            arg1: *mut root::app::Item,
            arg2: f32,
            arg3: *const root::phx::Vector3f,
        ) -> u64;
    }
}
pub mod FighterPokemonLinkEventChange {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterPokemonLinkEventChange__load_from_l2c_table_implEPNS_29FighterPokemonLinkEventChangeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPokemonLinkEventChange,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterPokemonLinkEventChange__store_l2c_table_implEPKNS_29FighterPokemonLinkEventChangeE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPokemonLinkEventChange,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterPokemonLinkEventChange__store_l2c_table_implEPKNS_29FighterPokemonLinkEventChangeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPokemonLinkEventChange,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ReflectorModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ReflectorModule__clean_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clean(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectorModule__is_shield_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn is_shield(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectorModule__set_size_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ReflectorModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_12ShieldStatusEi"]
        pub fn set_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::ShieldStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectorModule__set_status_all_implEPNS_26BattleObjectModuleAccessorENS_12ShieldStatusEi"]
        pub fn set_status_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ReflectorModule__is_turn_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectorModule__is_front_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ReflectorModule__is_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ReflectorModule__get_hop_angle_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_hop_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectorModule__is_no_hop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectorModule__set_turn_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_turn(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectorModule__set_front_implEPNS_26BattleObjectModuleAccessorENS_11ShieldFrontEi"]
        pub fn set_front(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldFront,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ReflectorModule__set_hop_implEPNS_26BattleObjectModuleAccessorEbfi"]
        pub fn set_hop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectorModule__set_attack_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectorModule__get_attack_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ReflectorModule__set_speed_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ReflectorModule__get_speed_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ReflectorModule__set_life_mul_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ReflectorModule__get_life_mul_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ReflectorModule__set_attack_limit_implEPNS_26BattleObjectModuleAccessorEfi"]
        pub fn set_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ReflectorModule__get_attack_limit_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_attack_limit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ReflectorModule__set_hit_stop_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_hit_stop_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ReflectorModule__is_no_m_ball_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_no_m_ball(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ReflectorModule__get_part_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_part_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ReflectorModule__get_team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ReflectorModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ReflectorModule__set_shield_type_implEPNS_26BattleObjectModuleAccessorENS_10ShieldTypeEii"]
        pub fn set_shield_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ShieldType,
            arg3: libc::c_int,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectorModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ReflectorModule__clear_all_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_all_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectorModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ReflectorModule__get_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ReflectorModule__get_group_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_group_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectorModule__get_pos_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_pos_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ReflectorModule__set_target_property_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_property(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ReflectorModule__set_target_category_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_target_category(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ReflectorModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
}
pub mod SearchModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SearchModule__clear_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24SearchModule__clear_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SearchModule__set_offset_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SearchModule__active_part_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn active_part_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SearchModule__is_search_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_search(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27SearchModule__get_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27SearchModule__set_size_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38SearchModule__set_target_opponent_implEPNS_26BattleObjectModuleAccessorEiiij"]
        pub fn set_target_opponent(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
            arg5: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26SearchModule__set_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_"]
        pub fn set_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
            arg4: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SearchModule__is_inflict_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_inflict(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37SearchModule__set_check_interval_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_check_interval(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40SearchModule__set_remove_log_attack_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_remove_log_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36SearchModule__remove_log_attack_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn remove_log_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24SearchModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27SearchModule__relocate_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fE"]
        pub fn relocate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38SearchModule__invalid_attack_mask_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn invalid_attack_mask(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42SearchModule__set_invalid_attack_mask_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_invalid_attack_mask(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37SearchModule__set_sync_situation_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sync_situation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30SearchModule__set_sync_lr_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_sync_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SearchModule__situation_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn situation(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34SearchModule__enable_safe_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_safe_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod PhysicsModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PhysicsModule__reset_swing_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_swing(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39PhysicsModule__set_swing_only_anim_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_swing_only_anim(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40PhysicsModule__set_swing_joint_name_implEPNS_26BattleObjectModuleAccessorEbN3phx6Hash40Eb"]
        pub fn set_swing_joint_name(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: root::phx::Hash40,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PhysicsModule__set_swing_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_swing_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37PhysicsModule__set_swing_rebirth_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_swing_rebirth(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__set_2nd_gravity_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PhysicsModule__set_2nd_status_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_2nd_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PhysicsModule__get_2nd_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__set_2nd_end_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fES6_"]
        pub fn set_2nd_end_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39PhysicsModule__get_2nd_line_length_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_2nd_line_length(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43PhysicsModule__set_2nd_line_length_all_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_line_length_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36PhysicsModule__get_2nd_node_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_node_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43PhysicsModule__get_2nd_active_node_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_active_node_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41PhysicsModule__get_2nd_active_length_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_active_length(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43PhysicsModule__set_2nd_active_node_num_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_active_node_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PhysicsModule__is_2nd_precede_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_2nd_precede(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__set_2nd_precede_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_2nd_precede(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PhysicsModule__is_2nd_flip_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_2nd_flip(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32PhysicsModule__set_2nd_flip_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_2nd_flip(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38PhysicsModule__set_2nd_back_speed_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_back_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26PhysicsModule__set_ik_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_ik(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37PhysicsModule__set_ik_target_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_"]
        pub fn set_ik_target_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
            arg4: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37PhysicsModule__set_ik_target_dir_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_"]
        pub fn set_ik_target_dir(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
            arg4: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39PhysicsModule__get_ik_end_joint_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_ik_end_joint_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PhysicsModule__stop_charge_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn stop_charge(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42PhysicsModule__set_2nd_air_resistance_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_air_resistance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44PhysicsModule__set_2nd_air_resistance_2_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_air_resistance_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44PhysicsModule__set_2nd_water_resistance_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_water_resistance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__get_2nd_tension_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_tension(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37PhysicsModule__get_2nd_joint_num_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_2nd_joint_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36PhysicsModule__get_2nd_joint_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_2nd_joint_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42PhysicsModule__set_2nd_collision_size_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_collision_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32PhysicsModule__set_2nd_mass_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_mass(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45PhysicsModule__set_2nd_disable_collision_implEPNS_26BattleObjectModuleAccessorEjb"]
        pub fn set_2nd_disable_collision(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36PhysicsModule__set_2nd_fix_flag_implEPNS_26BattleObjectModuleAccessorEjb"]
        pub fn set_2nd_fix_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42PhysicsModule__set_reflect_param_wall_implEPNS_26BattleObjectModuleAccessorEfff"]
        pub fn set_reflect_param_wall(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42PhysicsModule__set_reflect_param_ceil_implEPNS_26BattleObjectModuleAccessorEfff"]
        pub fn set_reflect_param_ceil(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43PhysicsModule__set_reflect_param_floor_implEPNS_26BattleObjectModuleAccessorEfff"]
        pub fn set_reflect_param_floor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34PhysicsModule__set_2nd_length_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_length(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PhysicsModule__get_2nd_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_2nd_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31PhysicsModule__set_2nd_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_2nd_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33PhysicsModule__get_2nd_speed_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_2nd_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33PhysicsModule__set_2nd_speed_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_2nd_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40PhysicsModule__set_2nd_node_num_max_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_2nd_node_num_max(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__clear_2nd_speed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_2nd_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44PhysicsModule__set_2nd_collision_object_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_2nd_collision_object(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40PhysicsModule__set_2nd_sync_gravity_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_2nd_sync_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45PhysicsModule__set_2nd_restitution_range_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_2nd_restitution_range(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44PhysicsModule__set_2nd_restitution_rate_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn set_2nd_restitution_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46PhysicsModule__set_2nd_restitution_rate_2_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_restitution_rate_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41PhysicsModule__set_2nd_friction_rate_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_2nd_friction_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41PhysicsModule__set_2nd_fixed_tip_num_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_2nd_fixed_tip_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35PhysicsModule__set_2nd_z_range_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn set_2nd_z_range(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49PhysicsModule__get_2nd_touch_ground_line_num_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_2nd_touch_ground_line_num(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51PhysicsModule__set_enable_floor_collision_line_implEPNS_26BattleObjectModuleAccessorEPNS_19GroundCollisionLineE"]
        pub fn set_enable_floor_collision_line(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::app::GroundCollisionLine,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50PhysicsModule__set_swing_ground_collision_all_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_swing_ground_collision_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43PhysicsModule__set_swing_special_state_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_swing_special_state(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
}
pub mod SlowModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24SlowModule__is_slow_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_slow(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind20SlowModule__set_implEPNS_26BattleObjectModuleAccessorEiiibj"]
        pub fn set(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            slow_mul: libc::c_int,
            frames: libc::c_int,
            arg5: bool,
            arg6: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind20SlowModule__mag_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn mag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35SlowModule__rate_ignore_effect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rate_ignore_effect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22SlowModule__frame_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22SlowModule__clear_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24SlowModule__clear_2_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21SlowModule__rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rate(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24SlowModule__is_skip_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_skip(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29SlowModule__is_prev_skip_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_prev_skip(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26SlowModule__set_whole_implEPNS_26BattleObjectModuleAccessorEhi"]
        pub fn set_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            slow_mul: libc::c_uchar,
            frames: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SlowModule__clear_whole_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26SlowModule__whole_mag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn whole_mag(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32SlowModule__set_whole_frame_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_whole_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28SlowModule__whole_frame_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn whole_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod CameraModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28CameraModule__reset_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_all(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CameraModule__update_force_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fES6_"]
        pub fn update_force(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
            arg4: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CameraModule__set_enable_camera_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_enable_camera(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28CameraModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40CameraModule__set_enable_update_pos_implEPNS_26BattleObjectModuleAccessorEhi"]
        pub fn set_enable_update_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45CameraModule__set_enable_interpolate_pos_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_enable_interpolate_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37CameraModule__reset_camera_range_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_camera_range(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42CameraModule__set_camera_range_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn set_camera_range_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42CameraModule__add_camera_range_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn add_camera_range_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40CameraModule__add_camera_range_rect_implEPNS_26BattleObjectModuleAccessorERKN3lib4RectEi"]
        pub fn add_camera_range_rect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::lib::Rect,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35CameraModule__un_regist_camera_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn un_regist_camera(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26CameraModule__zoom_in_implEPNS_26BattleObjectModuleAccessorEiifRKN3phx8Vector2fEb"]
        pub fn zoom_in(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: f32,
            arg5: *const root::phx::Vector2f,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CameraModule__zoom_out_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn zoom_out(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CameraModule__set_status_implEPNS_26BattleObjectModuleAccessorENS_12CameraStatusEi"]
        pub fn set_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::CameraStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CameraModule__set_priority_implEPNS_26BattleObjectModuleAccessorEhi"]
        pub fn set_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31CameraModule__get_priority_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CameraModule__set_player_no_implEPNS_26BattleObjectModuleAccessorEhi"]
        pub fn set_player_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CameraModule__get_player_no_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_player_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33CameraModule__set_damage_fly_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_damage_fly(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34CameraModule__exit_damage_fly_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn exit_damage_fly(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26CameraModule__set_run_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_run(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27CameraModule__exit_run_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn exit_run(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36CameraModule__set_being_carried_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_being_carried(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37CameraModule__exit_being_carried_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn exit_being_carried(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CameraModule__is_clip_in_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn is_clip_in(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33CameraModule__is_clip_in_all_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn is_clip_in_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47CameraModule__set_camera_range_global_rect_implEPNS_26BattleObjectModuleAccessorERKN3lib4RectEi"]
        pub fn set_camera_range_global_rect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::lib::Rect,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45CameraModule__get_main_camera_target_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_main_camera_target_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40CameraModule__get_main_camera_range_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_main_camera_range(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49CameraModule__get_internal_camera_target_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_internal_camera_target_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34CameraModule__get_camera_type_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_camera_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43CameraModule__get_camera_type_for_save_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_camera_type_for_save(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34CameraModule__set_camera_type_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_camera_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28CameraModule__req_quake_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn req_quake(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CameraModule__req_quake_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn req_quake_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29CameraModule__stop_quake_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn stop_quake(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32CameraModule__get_quakeKind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_quakeKind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39CameraModule__start_final_zoom_out_implEPNS_26BattleObjectModuleAccessorEiRKN3lib4RectERKN3phx8Vector3fE"]
        pub fn start_final_zoom_out(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::lib::Rect,
            arg4: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37CameraModule__end_final_zoom_out_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_final_zoom_out(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49CameraModule__get_camera_view_volume_z0_rect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_camera_view_volume_z0_rect(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40CameraModule__reset_main_camera_fov_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_main_camera_fov(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminConstraint {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind70FighterPikminLinkEventWeaponPikminConstraint__load_from_l2c_table_implEPNS_44FighterPikminLinkEventWeaponPikminConstraintERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminConstraint,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66FighterPikminLinkEventWeaponPikminConstraint__store_l2c_table_implEPKNS_44FighterPikminLinkEventWeaponPikminConstraintE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminConstraint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66FighterPikminLinkEventWeaponPikminConstraint__store_l2c_table_implEPKNS_44FighterPikminLinkEventWeaponPikminConstraintERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminConstraint,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod KineticEnergyNormal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergyNormal__get_accel_implEPNS_19KineticEnergyNormalE"]
        pub fn get_accel(arg1: *mut root::app::KineticEnergyNormal) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42KineticEnergyNormal__get_stable_speed_implEPNS_19KineticEnergyNormalE"]
        pub fn get_stable_speed(arg1: *mut root::app::KineticEnergyNormal)
            -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergyNormal__get_brake_implEPNS_19KineticEnergyNormalE"]
        pub fn get_brake(arg1: *mut root::app::KineticEnergyNormal) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41KineticEnergyNormal__get_limit_speed_implEPNS_19KineticEnergyNormalE"]
        pub fn get_limit_speed(arg1: *mut root::app::KineticEnergyNormal) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergyNormal__set_speed_implEPNS_19KineticEnergyNormalERKN3phx8Vector2fE"]
        pub fn set_speed(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyNormal__set_speed_3d_implEPNS_19KineticEnergyNormalERKN3phx8Vector3fE"]
        pub fn set_speed_3d(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergyNormal__set_accel_implEPNS_19KineticEnergyNormalERKN3phx8Vector2fE"]
        pub fn set_accel(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42KineticEnergyNormal__set_stable_speed_implEPNS_19KineticEnergyNormalERKN3phx8Vector2fE"]
        pub fn set_stable_speed(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergyNormal__set_brake_implEPNS_19KineticEnergyNormalERKN3phx8Vector2fE"]
        pub fn set_brake(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41KineticEnergyNormal__set_limit_speed_implEPNS_19KineticEnergyNormalERKN3phx8Vector2fE"]
        pub fn set_limit_speed(
            arg1: *mut root::app::KineticEnergyNormal,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51KineticEnergyNormal__on_consider_ground_normal_implEPNS_19KineticEnergyNormalE"]
        pub fn on_consider_ground_normal(
            arg1: *mut root::app::KineticEnergyNormal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52KineticEnergyNormal__off_consider_ground_normal_implEPNS_19KineticEnergyNormalE"]
        pub fn off_consider_ground_normal(
            arg1: *mut root::app::KineticEnergyNormal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51KineticEnergyNormal__is_consider_ground_normal_implEPNS_19KineticEnergyNormalE"]
        pub fn is_consider_ground_normal(
            arg1: *mut root::app::KineticEnergyNormal,
        ) -> bool;
    }
}
pub mod KineticEnergy {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticEnergy__get_speed_implEPNS_13KineticEnergyE"]
        pub fn get_speed(arg1: *mut root::app::KineticEnergy) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31KineticEnergy__get_speed_x_implEPNS_13KineticEnergyE"]
        pub fn get_speed_x(arg1: *mut root::app::KineticEnergy) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31KineticEnergy__get_speed_y_implEPNS_13KineticEnergyE"]
        pub fn get_speed_y(arg1: *mut root::app::KineticEnergy) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31KineticEnergy__get_speed3f_implEPNS_13KineticEnergyE"]
        pub fn get_speed3f(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32KineticEnergy__get_rotation_implEPNS_13KineticEnergyE"]
        pub fn get_rotation(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32KineticEnergy__reset_energy_implEPNS_13KineticEnergyEiRKN3phx8Vector2fERKNS3_8Vector3fERNS_26BattleObjectModuleAccessorE"]
        pub fn reset_energy(
            arg1: *mut root::app::KineticEnergy,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector2f,
            arg4: *const root::phx::Vector3f,
            arg5: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31KineticEnergy__clear_speed_implEPNS_13KineticEnergyE"]
        pub fn clear_speed(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticEnergy__clear_rot_speed_implEPNS_13KineticEnergyE"]
        pub fn clear_rot_speed(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticEnergy__mul_speed_implEPNS_13KineticEnergyERKN3phx8Vector3fE"]
        pub fn mul_speed(
            arg1: *mut root::app::KineticEnergy,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticEnergy__mul_accel_implEPNS_13KineticEnergyERKN3phx8Vector3fE"]
        pub fn mul_accel(
            arg1: *mut root::app::KineticEnergy,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticEnergy__reflect_speed_implEPNS_13KineticEnergyERKN3phx8Vector3fE"]
        pub fn reflect_speed(
            arg1: *mut root::app::KineticEnergy,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticEnergy__reflect_accel_implEPNS_13KineticEnergyERKN3phx8Vector3fE"]
        pub fn reflect_accel(
            arg1: *mut root::app::KineticEnergy,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47KineticEnergy__on_consider_ground_friction_implEPNS_13KineticEnergyE"]
        pub fn on_consider_ground_friction(
            arg1: *mut root::app::KineticEnergy,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48KineticEnergy__off_consider_ground_friction_implEPNS_13KineticEnergyE"]
        pub fn off_consider_ground_friction(
            arg1: *mut root::app::KineticEnergy,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26KineticEnergy__enable_implEPNS_13KineticEnergyE"]
        pub fn enable(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26KineticEnergy__unable_implEPNS_13KineticEnergyE"]
        pub fn unable(arg1: *mut root::app::KineticEnergy) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticEnergy__is_enable_implEPNS_13KineticEnergyE"]
        pub fn is_enable(arg1: *mut root::app::KineticEnergy) -> bool;
    }
}
pub mod FighterParamAccessor2 {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterParamAccessor2__thrown_offset_implEPNS_21FighterParamAccessor2Eiii"]
        pub fn thrown_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterParamAccessor2__donkey_thrown_offset_implEPNS_21FighterParamAccessor2Eii"]
        pub fn donkey_thrown_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> super::root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterParamAccessor2__ridley_dragged_offset_implEPNS_21FighterParamAccessor2Eii"]
        pub fn ridley_dragged_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterParamAccessor2__diddy_special_s_offset_implEPNS_21FighterParamAccessor2Ei"]
        pub fn diddy_special_s_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterParamAccessor2__miifighter_suplex_offset_implEPNS_21FighterParamAccessor2Ei"]
        pub fn miifighter_suplex_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterParamAccessor2__gaogaen_final_offset_implEPNS_21FighterParamAccessor2Eii"]
        pub fn gaogaen_final_offset(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterParamAccessor2__hit_target_no_implEPNS_21FighterParamAccessor2Eii"]
        pub fn hit_target_no(
            arg1: *mut root::app::FighterParamAccessor2,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
}
pub mod ReflectModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectModule__reset_info_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ReflectModule__object_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn object_id(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ReflectModule__set_object_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ReflectModule__team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn team_no(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ReflectModule__set_team_no_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectModule__attack_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ReflectModule__set_attack_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_attack_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ReflectModule__speed_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn speed_mul(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ReflectModule__set_speed_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ReflectModule__life_mul_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn life_mul(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ReflectModule__set_life_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_life_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ReflectModule__is_reflect_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_reflect(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectModule__set_no_speed_mul_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_speed_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ReflectModule__is_count_max_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_count_max(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ReflectModule__count_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn count(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ReflectModule__get_reverse_lr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_reverse_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ReflectModule__set_collision_no_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_collision_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
}
pub mod GimmickEventNotify {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventNotify__load_from_l2c_table_implEPNS_18GimmickEventNotifyERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventNotify,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventNotify__store_l2c_table_implEPKNS_18GimmickEventNotifyE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventNotify)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventNotify__store_l2c_table_implEPKNS_18GimmickEventNotifyERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventNotify,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminSetPowerMulStatus {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind77FighterPikminLinkEventWeaponPikminSetPowerMulStatus__load_from_l2c_table_implEPNS_51FighterPikminLinkEventWeaponPikminSetPowerMulStatusERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminSetPowerMulStatus,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind73FighterPikminLinkEventWeaponPikminSetPowerMulStatus__store_l2c_table_implEPKNS_51FighterPikminLinkEventWeaponPikminSetPowerMulStatusE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetPowerMulStatus,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind73FighterPikminLinkEventWeaponPikminSetPowerMulStatus__store_l2c_table_implEPKNS_51FighterPikminLinkEventWeaponPikminSetPowerMulStatusERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetPowerMulStatus,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod WeaponKineticEnergyGravity {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42WeaponKineticEnergyGravity__set_accel_implEPNS_26WeaponKineticEnergyGravityEf"]
        pub fn set_accel(arg1: *mut root::app::WeaponKineticEnergyGravity, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42WeaponKineticEnergyGravity__set_speed_implEPNS_26WeaponKineticEnergyGravityEf"]
        pub fn set_speed(arg1: *mut root::app::WeaponKineticEnergyGravity, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48WeaponKineticEnergyGravity__get_limit_speed_implEPNS_26WeaponKineticEnergyGravityE"]
        pub fn get_limit_speed(
            arg1: *mut root::app::WeaponKineticEnergyGravity,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48WeaponKineticEnergyGravity__set_limit_speed_implEPNS_26WeaponKineticEnergyGravityEf"]
        pub fn set_limit_speed(
            arg1: *mut root::app::WeaponKineticEnergyGravity,
            arg2: f32,
        );
    }
}
pub mod WeaponShizueFishingrodLinkEventCliff {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62WeaponShizueFishingrodLinkEventCliff__load_from_l2c_table_implEPNS_36WeaponShizueFishingrodLinkEventCliffERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponShizueFishingrodLinkEventCliff,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponShizueFishingrodLinkEventCliff__store_l2c_table_implEPKNS_36WeaponShizueFishingrodLinkEventCliffE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventCliff,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponShizueFishingrodLinkEventCliff__store_l2c_table_implEPKNS_36WeaponShizueFishingrodLinkEventCliffERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventCliff,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventFinal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkEventFinal__load_from_l2c_table_implEPNS_14LinkEventFinalERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventFinal,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkEventFinal__store_l2c_table_implEPKNS_14LinkEventFinalE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventFinal) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkEventFinal__store_l2c_table_implEPKNS_14LinkEventFinalERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventFinal,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEvent {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkEvent__load_from_l2c_table_implEPNS_9LinkEventERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEvent) -> root::lib::L2CValue;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventCaptureMimikkyu {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkEventCaptureMimikkyu__load_from_l2c_table_implEPNS_24LinkEventCaptureMimikkyuERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCaptureMimikkyu,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46LinkEventCaptureMimikkyu__store_l2c_table_implEPKNS_24LinkEventCaptureMimikkyuE"]
        pub fn store_l2c_table(
            arg1: *const root::app::LinkEventCaptureMimikkyu,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46LinkEventCaptureMimikkyu__store_l2c_table_implEPKNS_24LinkEventCaptureMimikkyuERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCaptureMimikkyu,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventCapture {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkEventCapture__load_from_l2c_table_implEPNS_16LinkEventCaptureERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventCapture,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkEventCapture__store_l2c_table_implEPKNS_16LinkEventCaptureE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventCapture) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkEventCapture__store_l2c_table_implEPKNS_16LinkEventCaptureERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventCapture,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventDrumEscape {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48GimmickEventDrumEscape__load_from_l2c_table_implEPNS_22GimmickEventDrumEscapeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventDrumEscape__store_l2c_table_implEPKNS_22GimmickEventDrumEscapeE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumEscape,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventDrumEscape__store_l2c_table_implEPKNS_22GimmickEventDrumEscapeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventPos {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41GimmickEventPos__load_from_l2c_table_implEPNS_15GimmickEventPosERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GimmickEventPos__store_l2c_table_implEPKNS_15GimmickEventPosE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventPos) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37GimmickEventPos__store_l2c_table_implEPKNS_15GimmickEventPosERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventTornadoEscape {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventTornadoEscape__load_from_l2c_table_implEPNS_25GimmickEventTornadoEscapeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventTornadoEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GimmickEventTornadoEscape__store_l2c_table_implEPKNS_25GimmickEventTornadoEscapeE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventTornadoEscape,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GimmickEventTornadoEscape__store_l2c_table_implEPKNS_25GimmickEventTornadoEscapeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventTornadoEscape,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterKineticEnergyGravity {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43FighterKineticEnergyGravity__set_accel_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_accel(arg1: *mut root::app::FighterKineticEnergyGravity, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43FighterKineticEnergyGravity__set_speed_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_speed(arg1: *mut root::app::FighterKineticEnergyGravity, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterKineticEnergyGravity__set_stable_speed_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_stable_speed(
            arg1: *mut root::app::FighterKineticEnergyGravity,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterKineticEnergyGravity__set_limit_speed_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_limit_speed(
            arg1: *mut root::app::FighterKineticEnergyGravity,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43FighterKineticEnergyGravity__set_brake_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_brake(arg1: *mut root::app::FighterKineticEnergyGravity, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43FighterKineticEnergyGravity__get_accel_implEPNS_27FighterKineticEnergyGravityE"]
        pub fn get_accel(
            arg1: *mut root::app::FighterKineticEnergyGravity,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterKineticEnergyGravity__get_stable_speed_implEPNS_27FighterKineticEnergyGravityE"]
        pub fn get_stable_speed(
            arg1: *mut root::app::FighterKineticEnergyGravity,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterKineticEnergyGravity__get_limit_speed_implEPNS_27FighterKineticEnergyGravityE"]
        pub fn get_limit_speed(
            arg1: *mut root::app::FighterKineticEnergyGravity,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43FighterKineticEnergyGravity__get_brake_implEPNS_27FighterKineticEnergyGravityE"]
        pub fn get_brake(
            arg1: *mut root::app::FighterKineticEnergyGravity,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterKineticEnergyGravity__set_gravity_coefficient_implEPNS_27FighterKineticEnergyGravityEf"]
        pub fn set_gravity_coefficient(
            arg1: *mut root::app::FighterKineticEnergyGravity,
            arg2: f32,
        );
    }
}
pub mod ItemManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ItemManager__get_num_of_ownered_item_implEPNS_11ItemManagerEjNS_8ItemKindE"]
        pub fn get_num_of_ownered_item(
            arg1: *mut root::app::ItemManager,
            arg2: libc::c_uint,
            arg3: root::app::ItemKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemManager__get_num_of_active_item_all_implEPNS_11ItemManagerE"]
        pub fn get_num_of_active_item_all(
            arg1: *mut root::app::ItemManager,
        ) -> u64;
    }
    
    extern "C" {
        #[link_name = "\u{1}_ZN3app12item_manager22get_num_of_active_itemENS_8ItemKindE"]
        pub fn get_num_of_active_item(
            item_kind: i32
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ItemManager__get_active_item_implEPNS_11ItemManagerEm"]
        pub fn get_active_item(
            arg1: *mut root::app::ItemManager,
            arg2: libc::c_ulong,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ItemManager__find_active_item_from_id_implEPNS_11ItemManagerEj"]
        pub fn find_active_item_from_id(
            arg1: *mut root::app::ItemManager,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47ItemManager__find_active_item_from_area_id_implEPNS_11ItemManagerEj"]
        pub fn find_active_item_from_area_id(
            arg1: *mut root::app::ItemManager,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ItemManager__remove_item_from_id_implEPNS_11ItemManagerEj"]
        pub fn remove_item_from_id(
            arg1: *mut root::app::ItemManager,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52ItemManager__is_change_fighter_restart_position_implEPNS_11ItemManagerE"]
        pub fn is_change_fighter_restart_position(
            arg1: *mut root::app::ItemManager,
        ) -> bool;
    }
}
pub mod AreaModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29AreaModule__force_update_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn force_update(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AreaModule__set_auto_layer_update_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_auto_layer_update(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22AreaModule__clean_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clean(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AreaModule__set_layer_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_layer(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22AreaModule__layer_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn layer(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AreaModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AreaModule__get_whole_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26AreaModule__get_group_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35AreaModule__area_instance_size_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn area_instance_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AreaModule__enable_area_implEPNS_26BattleObjectModuleAccessorEibi"]
        pub fn enable_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31AreaModule__is_enable_area_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_enable_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AreaModule__reset_area_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_area(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43AreaModule__set_area_target_group_mask_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_area_target_group_mask(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49AreaModule__set_area_target_local_group_mask_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_area_target_local_group_mask(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AreaModule__set_area_shape_aabb_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector2fES6_"]
        pub fn set_area_shape_aabb(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector2f,
            arg4: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AreaModule__set_area_shape_circle_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector2fEf"]
        pub fn set_area_shape_circle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector2f,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AreaModule__set_area_shape_type_implEPNS_26BattleObjectModuleAccessorEih"]
        pub fn set_area_shape_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30AreaModule__set_center_x0_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_center_x0(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28AreaModule__get_area_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_area_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AreaModule__is_exist_area_instance_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_exist_area_instance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AreaModule__get_area_index_from_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn get_area_index_from_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39AreaModule__get_area_contact_count_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_area_contact_count(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37AreaModule__get_area_contact_log_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_area_contact_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43AreaModule__get_area_contact_target_id_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_area_contact_target_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AreaModule__erase_wind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn erase_wind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AreaModule__add_wind_area_2nd_implEPNS_26BattleObjectModuleAccessorEiffffRKN3phx8Vector2fES6_i"]
        pub fn add_wind_area_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: f32,
            arg5: f32,
            arg6: f32,
            arg7: *const root::phx::Vector2f,
            arg8: *const root::phx::Vector2f,
            arg9: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38AreaModule__add_wind_area_2nd_rad_implEPNS_26BattleObjectModuleAccessorEiffffRKN3phx8Vector2fEfi"]
        pub fn add_wind_area_2nd_rad(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: f32,
            arg5: f32,
            arg6: f32,
            arg7: *const root::phx::Vector2f,
            arg8: f32,
            arg9: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind22AreaModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25AreaModule__is_sleep_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_sleep(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25AreaModule__is_water_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_water(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34AreaModule__get_water_task_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_water_task_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AreaModule__get_water_surface_y_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_water_surface_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27AreaModule__test_water_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fERf"]
        pub fn test_water(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: *mut f32,
        ) -> u64;
    }
}
pub mod ItemCameraModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47ItemCameraModuleImpl__start_camera_subject_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn start_camera_subject(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ItemCameraModuleImpl__end_camera_subject_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn end_camera_subject(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58ItemCameraModuleImpl__set_camera_subject_pos_4_points_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector4fE"]
        pub fn set_camera_subject_pos_4_points(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector4f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51ItemCameraModuleImpl__clamp_camera_subject_pos_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector4fE"]
        pub fn clamp_camera_subject_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector4f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52ItemCameraModuleImpl__set_camera_subject_enable_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_camera_subject_enable(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemCameraModuleImpl__set_camera_subject_pos_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn set_camera_subject_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemCameraModuleImpl__get_camera_subject_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_camera_subject_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51ItemCameraModuleImpl__set_camera_subject_range_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector4fE"]
        pub fn set_camera_subject_range(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector4f,
        );
    }
}
pub mod ItemDamageModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ItemDamageModuleImpl__is_smash_damage_implEPNS_26BattleObjectModuleAccessorEff"]
        pub fn is_smash_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemDamageModuleImpl__damage_log_value_float_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn damage_log_value_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47ItemDamageModuleImpl__damage_log_value_int_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn damage_log_value_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
}
pub mod FighterManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32FighterManager__entry_count_implEPNS_14FighterManagerE"]
        pub fn entry_count(arg1: *mut root::app::FighterManager) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__total_fighter_num_implEPNS_14FighterManagerE"]
        pub fn total_fighter_num(arg1: *mut root::app::FighterManager) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterManager__get_entry_id_implEPNS_14FighterManagerEi"]
        pub fn get_entry_id(
            arg1: *mut root::app::FighterManager,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterManager__get_entry_no_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub fn get_entry_no(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub fn get_fighter_entry(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44FighterManager__get_fighter_information_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub fn get_fighter_information(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
        ) -> *mut root::app::FighterInformation;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterManager__is_melee_mode_homerun_implEPNS_14FighterManagerE"]
        pub fn is_melee_mode_homerun(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__is_homerun_versus_implEPNS_14FighterManagerE"]
        pub fn is_homerun_versus(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterManager__is_melee_mode_online_tournament_implEPNS_14FighterManagerE"]
        pub fn is_melee_mode_online_tournament(
            arg1: *mut root::app::FighterManager,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterManager__one_on_one_ratio_implEPNS_14FighterManagerE"]
        pub fn one_on_one_ratio(arg1: *mut root::app::FighterManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterManager__is_discretion_final_enabled_implEPNS_14FighterManagerE"]
        pub fn is_discretion_final_enabled(
            arg1: *mut root::app::FighterManager,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44FighterManager__get_top_rank_player_num_implEPNS_14FighterManagerE"]
        pub fn get_top_rank_player_num(
            arg1: *mut root::app::FighterManager,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40FighterManager__get_top_rank_player_implEPNS_14FighterManagerEi"]
        pub fn get_top_rank_player(
            arg1: *mut root::app::FighterManager,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32FighterManager__is_ready_go_implEPNS_14FighterManagerE"]
        pub fn is_ready_go(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterManager__set_cursor_whole_implEPNS_14FighterManagerEb"]
        pub fn set_cursor_whole(arg1: *mut root::app::FighterManager, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36FighterManager__get_fighter_pos_implEPNS_14FighterManagerENS_14FighterEntryIDEi"]
        pub fn get_fighter_pos(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterManager__is_available_discretion_final_implEPNS_14FighterManagerE"]
        pub fn is_available_discretion_final(
            arg1: *mut root::app::FighterManager,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45FighterManager__get_final_actor_entry_id_implEPNS_14FighterManagerE"]
        pub fn get_final_actor_entry_id(
            arg1: *mut root::app::FighterManager,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29FighterManager__is_final_implEPNS_14FighterManagerE"]
        pub fn is_final(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30FighterManager__set_final_implEPNS_14FighterManagerENS_14FighterEntryIDENS_21FighterAvailableFinalEj"]
        pub fn set_final(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
            arg3: root::app::FighterAvailableFinal,
            arg4: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterManager__get_no_discretion_final_beat_count_implEPNS_14FighterManagerE"]
        pub fn get_no_discretion_final_beat_count(
            arg1: *mut root::app::FighterManager,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterManager__get_beat_point_diff_from_top_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub fn get_beat_point_diff_from_top(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40FighterManager__set_final_fear_face_implEPNS_14FighterManagerENS_14FighterEntryIDEi"]
        pub fn set_final_fear_face(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterManager__start_finalbg_implEPNS_14FighterManagerEif"]
        pub fn start_finalbg(
            arg1: *mut root::app::FighterManager,
            arg2: libc::c_int,
            arg3: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterManager__exit_finalbg_implEPNS_14FighterManagerE"]
        pub fn exit_finalbg(arg1: *mut root::app::FighterManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40FighterManager__set_visible_finalbg_implEPNS_14FighterManagerEb"]
        pub fn set_visible_finalbg(arg1: *mut root::app::FighterManager, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterManager__notify_log_event_collision_hit_implEPNS_14FighterManagerEjjfib"]
        pub fn notify_log_event_collision_hit(
            arg1: *mut root::app::FighterManager,
            arg2: libc::c_uint,
            arg3: libc::c_uint,
            arg4: f32,
            arg5: libc::c_int,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterManager__is_process_technique_implEPNS_14FighterManagerE"]
        pub fn is_process_technique(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35FighterManager__is_result_mode_implEPNS_14FighterManagerE"]
        pub fn is_result_mode(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterManager__is_disable_ko_camera_implEPNS_14FighterManagerE"]
        pub fn is_disable_ko_camera(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterManager__enable_ko_camera_implEPNS_14FighterManagerE"]
        pub fn enable_ko_camera(arg1: *mut root::app::FighterManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__disable_ko_camera_implEPNS_14FighterManagerE"]
        pub fn disable_ko_camera(arg1: *mut root::app::FighterManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterManager__set_dead_up_camera_hit_cursor_status_implEPNS_14FighterManagerEb"]
        pub fn set_dead_up_camera_hit_cursor_status(
            arg1: *mut root::app::FighterManager,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterManager__set_controller_rumble_all_implEPNS_14FighterManagerEN3phx6Hash40Eibj"]
        pub fn set_controller_rumble_all(
            arg1: *mut root::app::FighterManager,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
            arg5: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterManager__is_rebirth_plate_line_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub fn is_rebirth_plate_line(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__set_position_lock_implEPNS_14FighterManagerENS_14FighterEntryIDEb"]
        pub fn set_position_lock(
            arg1: *mut root::app::FighterManager,
            arg2: root::app::FighterEntryID,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterManager__reset_fighter_implEPNS_14FighterManagerEb"]
        pub fn reset_fighter(
            arg1: *mut root::app::FighterManager,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterManager__prepare_movie_implEPNS_14FighterManagerEN3phx6Hash40E"]
        pub fn prepare_movie(
            arg1: *mut root::app::FighterManager,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__is_prepared_movie_implEPNS_14FighterManagerE"]
        pub fn is_prepared_movie(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31FighterManager__exit_movie_implEPNS_14FighterManagerE"]
        pub fn exit_movie(arg1: *mut root::app::FighterManager) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterManager__is_process_movie_implEPNS_14FighterManagerE"]
        pub fn is_process_movie(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32FighterManager__start_movie_implEPNS_14FighterManagerEf"]
        pub fn start_movie(
            arg1: *mut root::app::FighterManager,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterManager__is_end_movie_implEPNS_14FighterManagerE"]
        pub fn is_end_movie(arg1: *mut root::app::FighterManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterManager__get_jack_final_cut_in_implEPNS_14FighterManagerE"]
        pub fn get_jack_final_cut_in(arg1: *mut root::app::FighterManager)
            -> u64;
    }
}
pub mod FighterStatusModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53FighterStatusModuleImpl__set_fighter_status_data_implEPNS_26BattleObjectModuleAccessorEbibbbmjjj"]
        pub fn set_fighter_status_data(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
            arg4: bool,
            arg5: bool,
            arg6: bool,
            arg7: libc::c_ulong,
            arg8: libc::c_uint,
            arg9: libc::c_uint,
            arg10: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterStatusModuleImpl__reset_log_action_info_implEPNS_26BattleObjectModuleAccessorEm"]
        pub fn reset_log_action_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_ulong,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterStatusModuleImpl__off_disable_intrrupt_warp_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn off_disable_intrrupt_warp(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod KineticEnergyRotNormal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyRotNormal__get_accel_implEPNS_22KineticEnergyRotNormalE"]
        pub fn get_accel(arg1: *mut root::app::KineticEnergyRotNormal) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45KineticEnergyRotNormal__get_stable_speed_implEPNS_22KineticEnergyRotNormalE"]
        pub fn get_stable_speed(
            arg1: *mut root::app::KineticEnergyRotNormal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyRotNormal__get_brake_implEPNS_22KineticEnergyRotNormalE"]
        pub fn get_brake(arg1: *mut root::app::KineticEnergyRotNormal) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44KineticEnergyRotNormal__get_limit_speed_implEPNS_22KineticEnergyRotNormalE"]
        pub fn get_limit_speed(
            arg1: *mut root::app::KineticEnergyRotNormal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42KineticEnergyRotNormal__get_rot_speed_implEPNS_22KineticEnergyRotNormalE"]
        pub fn get_rot_speed(arg1: *mut root::app::KineticEnergyRotNormal)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyRotNormal__set_speed_implEPNS_22KineticEnergyRotNormalERKN3phx8Vector3fE"]
        pub fn set_speed(
            arg1: *mut root::app::KineticEnergyRotNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyRotNormal__set_accel_implEPNS_22KineticEnergyRotNormalERKN3phx8Vector3fE"]
        pub fn set_accel(
            arg1: *mut root::app::KineticEnergyRotNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45KineticEnergyRotNormal__set_stable_speed_implEPNS_22KineticEnergyRotNormalERKN3phx8Vector3fE"]
        pub fn set_stable_speed(
            arg1: *mut root::app::KineticEnergyRotNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticEnergyRotNormal__set_brake_implEPNS_22KineticEnergyRotNormalERKN3phx8Vector3fE"]
        pub fn set_brake(
            arg1: *mut root::app::KineticEnergyRotNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44KineticEnergyRotNormal__set_limit_speed_implEPNS_22KineticEnergyRotNormalERKN3phx8Vector3fE"]
        pub fn set_limit_speed(
            arg1: *mut root::app::KineticEnergyRotNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
}
pub mod TeamModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24TeamModule__team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn team_no(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31TeamModule__team_second_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn team_second_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25TeamModule__set_team_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32TeamModule__set_team_second_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_team_second(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28TeamModule__hit_team_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hit_team_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35TeamModule__hit_team_second_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn hit_team_second_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29TeamModule__set_hit_team_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_hit_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36TeamModule__set_hit_team_second_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_hit_team_second(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34TeamModule__set_team_owner_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_team_owner_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30TeamModule__team_owner_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn team_owner_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47TeamModule__metamon_owner_fighter_entry_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn metamon_owner_fighter_entry_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod ComboModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21ComboModule__set_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: libc::c_int);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23ComboModule__reset_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23ComboModule__count_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn count(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ComboModule__is_enable_combo_input_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_enable_combo_input(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
}
pub mod BossManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33BossManager__is_stoppable_se_implEPNS_11BossManagerE"]
        pub fn is_stoppable_se(arg1: *mut root::app::BossManager) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39BossManager__notify_on_boss_defeat_implEPNS_11BossManagerENS_11FighterKindE"]
        pub fn notify_on_boss_defeat(
            arg1: *mut root::app::BossManager,
            arg2: root::app::FighterKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43BossManager__notify_on_boss_keyoff_bgm_implEPNS_11BossManagerENS_11FighterKindE"]
        pub fn notify_on_boss_keyoff_bgm(
            arg1: *mut root::app::BossManager,
            arg2: root::app::FighterKind,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37BossManager__notify_on_boss_dead_implEPNS_11BossManagerENS_11FighterKindE"]
        pub fn notify_on_boss_dead(
            arg1: *mut root::app::BossManager,
            arg2: root::app::FighterKind,
        ) -> u64;
    }
}
pub mod AttackData {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36AttackData__load_from_l2c_table_implEPNS_10AttackDataERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::AttackData,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackData__store_l2c_table_implEPKNS_10AttackDataE"]
        pub fn store_l2c_table(arg1: *const root::app::AttackData) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32AttackData__store_l2c_table_implEPKNS_10AttackDataERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::AttackData,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterInformation {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34FighterInformation__hit_point_implEPNS_18FighterInformationE"]
        pub fn hit_point(arg1: *mut root::app::FighterInformation) -> f32;
    }
    extern "C" {
        /// In stamina mode, returns the max stamina each fighter starts with, which is set in the ruleset
        ///
        /// # Arguments
        ///
        /// * `FighterInformation` - Pointer to FighterInformation
        ///
        /// * 'bool' - As a result of testing, regardless of whether it is set to true or false, it still returns the max stamina value
        ///
        /// # Example
        ///
        /// ```
        /// // Ruleset has stamina set to 150
        /// let hp_max_1 = hit_point_max(FighterInformation, false); // Returns 150
        /// let hp_max_2 = hit_point_max(FighterInformation, true); // Returns 150
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__hit_point_max_implEPNS_18FighterInformationEb"]
        pub fn hit_point_max(arg1: *mut root::app::FighterInformation, arg2: bool) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__fighter_color_implEPNS_18FighterInformationE"]
        pub fn fighter_color(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterInformation__is_operation_cpu_implEPNS_18FighterInformationE"]
        pub fn is_operation_cpu(arg1: *mut root::app::FighterInformation) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterInformation__get_no_change_hp_implEPNS_18FighterInformationE"]
        pub fn get_no_change_hp(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35FighterInformation__dead_count_implEPNS_18FighterInformationEi"]
        pub fn dead_count(
            arg1: *mut root::app::FighterInformation,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36FighterInformation__stock_count_implEPNS_18FighterInformationE"]
        pub fn stock_count(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__suicide_count_implEPNS_18FighterInformationEi"]
        pub fn suicide_count(
            arg1: *mut root::app::FighterInformation,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterInformation__total_beat_count_implEPNS_18FighterInformationEi"]
        pub fn total_beat_count(
            arg1: *mut root::app::FighterInformation,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45FighterInformation__is_last_dead_suicide_implEPNS_18FighterInformationE"]
        pub fn is_last_dead_suicide(arg1: *mut root::app::FighterInformation) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51FighterInformation__set_add_rebirth_wait_frame_implEPNS_18FighterInformationEf"]
        pub fn set_add_rebirth_wait_frame(
            arg1: *mut root::app::FighterInformation,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__is_on_rebirth_implEPNS_18FighterInformationE"]
        pub fn is_on_rebirth(arg1: *mut root::app::FighterInformation) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41FighterInformation__fighter_category_implEPNS_18FighterInformationE"]
        pub fn fighter_category(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32FighterInformation__gravity_implEPNS_18FighterInformationE"]
        pub fn gravity(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39FighterInformation__summon_boss_id_implEPNS_18FighterInformationE"]
        pub fn summon_boss_id(arg1: *mut root::app::FighterInformation) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterInformation__summon_bomb_ready_frame_implEPNS_18FighterInformationE"]
        pub fn summon_bomb_ready_frame(
            arg1: *mut root::app::FighterInformation,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53FighterInformation__summon_pre_bomb_effect_frame_implEPNS_18FighterInformationE"]
        pub fn summon_pre_bomb_effect_frame(
            arg1: *mut root::app::FighterInformation,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55FighterInformation__summon_suicide_bomb_attack_mul_implEPNS_18FighterInformationE"]
        pub fn summon_suicide_bomb_attack_mul(
            arg1: *mut root::app::FighterInformation,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57FighterInformation__summon_suicide_bomb_reaction_mul_implEPNS_18FighterInformationE"]
        pub fn summon_suicide_bomb_reaction_mul(
            arg1: *mut root::app::FighterInformation,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterInformation__is_battle_event_stick_reverse_implEPNS_18FighterInformationE"]
        pub fn is_battle_event_stick_reverse(
            arg1: *mut root::app::FighterInformation,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36FighterInformation__get_log_int_implEPNS_18FighterInformationEiii"]
        pub fn get_log_int(
            arg1: *mut root::app::FighterInformation,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__is_backshield_implEPNS_18FighterInformationE"]
        pub fn is_backshield(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__is_rabbit_cap_implEPNS_18FighterInformationE"]
        pub fn is_rabbit_cap(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterInformation__is_reflector_implEPNS_18FighterInformationE"]
        pub fn is_reflector(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38FighterInformation__is_rocketbelt_implEPNS_18FighterInformationE"]
        pub fn is_rocketbelt(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterInformation__is_screw_implEPNS_18FighterInformationE"]
        pub fn is_screw(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterInformation__is_superleaf_implEPNS_18FighterInformationE"]
        pub fn is_superleaf(
            arg1: *mut root::app::FighterInformation
        ) -> bool;
    }
}
pub mod FighterPikminLinkEventWeaponPikminOnFlag {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66FighterPikminLinkEventWeaponPikminOnFlag__load_from_l2c_table_implEPNS_40FighterPikminLinkEventWeaponPikminOnFlagERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminOnFlag,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminOnFlag__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminOnFlagE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminOnFlag,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminOnFlag__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminOnFlagERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminOnFlag,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterKineticEnergyController {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__mul_x_speed_max_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn mul_x_speed_max(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__set_accel_x_mul_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn set_accel_x_mul(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__set_accel_x_add_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn set_accel_x_add(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__mul_x_accel_mul_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn mul_x_accel_mul(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__mul_x_accel_add_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn mul_x_accel_add(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__set_accel_y_mul_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn set_accel_y_mul(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__set_accel_y_add_implEPNS_30FighterKineticEnergyControllerEf"]
        pub fn set_accel_y_add(
            arg1: *mut root::app::FighterKineticEnergyController,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__get_accel_x_mul_implEPNS_30FighterKineticEnergyControllerE"]
        pub fn get_accel_x_mul(
            arg1: *mut root::app::FighterKineticEnergyController,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__get_accel_x_add_implEPNS_30FighterKineticEnergyControllerE"]
        pub fn get_accel_x_add(
            arg1: *mut root::app::FighterKineticEnergyController,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__get_accel_y_mul_implEPNS_30FighterKineticEnergyControllerE"]
        pub fn get_accel_y_mul(
            arg1: *mut root::app::FighterKineticEnergyController,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterKineticEnergyController__get_accel_y_add_implEPNS_30FighterKineticEnergyControllerE"]
        pub fn get_accel_y_add(
            arg1: *mut root::app::FighterKineticEnergyController,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminSyncLR {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66FighterPikminLinkEventWeaponPikminSyncLR__load_from_l2c_table_implEPNS_40FighterPikminLinkEventWeaponPikminSyncLRERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminSyncLR,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminSyncLR__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminSyncLRE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSyncLR,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminSyncLR__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminSyncLRERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSyncLR,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod WeaponRobotHominglaserLinkEventSearch {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind63WeaponRobotHominglaserLinkEventSearch__load_from_l2c_table_implEPNS_37WeaponRobotHominglaserLinkEventSearchERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponRobotHominglaserLinkEventSearch,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59WeaponRobotHominglaserLinkEventSearch__store_l2c_table_implEPKNS_37WeaponRobotHominglaserLinkEventSearchE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponRobotHominglaserLinkEventSearch,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59WeaponRobotHominglaserLinkEventSearch__store_l2c_table_implEPKNS_37WeaponRobotHominglaserLinkEventSearchERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponRobotHominglaserLinkEventSearch,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventDrumShake {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GimmickEventDrumShake__load_from_l2c_table_implEPNS_21GimmickEventDrumShakeERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumShake,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GimmickEventDrumShake__store_l2c_table_implEPKNS_21GimmickEventDrumShakeE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumShake,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43GimmickEventDrumShake__store_l2c_table_implEPKNS_21GimmickEventDrumShakeERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumShake,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterSpiritsSupportSkill {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59FighterSpiritsSupportSkill__is_unsync_vis_whole_effect_implEPNS_26FighterSpiritsSupportSkillE"]
        pub fn is_unsync_vis_whole_effect(
            arg1: *mut root::app::FighterSpiritsSupportSkill,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterSpiritsSupportSkill__is_visible_effect_implEPNS_26FighterSpiritsSupportSkillE"]
        pub fn is_visible_effect(
            arg1: *mut root::app::FighterSpiritsSupportSkill,
        ) -> bool;
    }
}
pub mod StageManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33StageManager__stage_all_stop_implEPNS_12StageManagerEb"]
        pub fn stage_all_stop(
            arg1: *mut root::app::StageManager,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46StageManager__is_discretion_final_enabled_implEPNS_12StageManagerE"]
        pub fn is_discretion_final_enabled(arg1: *mut root::app::StageManager) -> bool;
    }
}
pub mod GimmickEventPresenter {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55GimmickEventPresenter__dispatch_event_from_fighter_implEPNS_21GimmickEventPresenterERNS_12GimmickEventE"]
        pub fn dispatch_event_from_fighter(
            arg1: *mut root::app::GimmickEventPresenter,
            arg2: *mut root::app::GimmickEvent,
        ) -> u64;
    }
}
pub mod GimmickEventDrumRelease {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GimmickEventDrumRelease__load_from_l2c_table_implEPNS_23GimmickEventDrumReleaseERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventDrumRelease,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GimmickEventDrumRelease__store_l2c_table_implEPKNS_23GimmickEventDrumReleaseE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventDrumRelease,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45GimmickEventDrumRelease__store_l2c_table_implEPKNS_23GimmickEventDrumReleaseERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventDrumRelease,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminSetInt {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind66FighterPikminLinkEventWeaponPikminSetInt__load_from_l2c_table_implEPNS_40FighterPikminLinkEventWeaponPikminSetIntERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminSetInt,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminSetInt__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminSetIntE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetInt,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62FighterPikminLinkEventWeaponPikminSetInt__store_l2c_table_implEPKNS_40FighterPikminLinkEventWeaponPikminSetIntERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminSetInt,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventTornadoShootInfo {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54GimmickEventTornadoShootInfo__load_from_l2c_table_implEPNS_28GimmickEventTornadoShootInfoERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventTornadoShootInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50GimmickEventTornadoShootInfo__store_l2c_table_implEPKNS_28GimmickEventTornadoShootInfoE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventTornadoShootInfo,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50GimmickEventTornadoShootInfo__store_l2c_table_implEPKNS_28GimmickEventTornadoShootInfoERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventTornadoShootInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterKineticEnergyMotion {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterKineticEnergyMotion__set_angle_implEPNS_26FighterKineticEnergyMotionEf"]
        pub fn set_angle(arg1: *mut root::app::FighterKineticEnergyMotion, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterKineticEnergyMotion__set_angle_whole_implEPNS_26FighterKineticEnergyMotionEfi"]
        pub fn set_angle_whole(
            arg1: *mut root::app::FighterKineticEnergyMotion,
            arg2: f32,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterKineticEnergyMotion__set_speed_mul_implEPNS_26FighterKineticEnergyMotionEf"]
        pub fn set_speed_mul(
            arg1: *mut root::app::FighterKineticEnergyMotion,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterKineticEnergyMotion__set_chara_dir_implEPNS_26FighterKineticEnergyMotionEf"]
        pub fn set_chara_dir(
            arg1: *mut root::app::FighterKineticEnergyMotion,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterKineticEnergyMotion__reverse_chara_dir_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn reverse_chara_dir(
            arg1: *mut root::app::FighterKineticEnergyMotion,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterKineticEnergyMotion__set_update_flag_implEPNS_26FighterKineticEnergyMotionEb"]
        pub fn set_update_flag(
            arg1: *mut root::app::FighterKineticEnergyMotion,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50FighterKineticEnergyMotion__set_speed_mul_2nd_implEPNS_26FighterKineticEnergyMotionERKN3phx8Vector2fE"]
        pub fn set_speed_mul_2nd(
            arg1: *mut root::app::FighterKineticEnergyMotion,
            arg2: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42FighterKineticEnergyMotion__get_angle_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn get_angle(arg1: *mut root::app::FighterKineticEnergyMotion)
            -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterKineticEnergyMotion__get_angle_whole_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn get_angle_whole(
            arg1: *mut root::app::FighterKineticEnergyMotion,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterKineticEnergyMotion__get_chara_dir_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn get_chara_dir(
            arg1: *mut root::app::FighterKineticEnergyMotion,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46FighterKineticEnergyMotion__get_speed_mul_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn get_speed_mul(
            arg1: *mut root::app::FighterKineticEnergyMotion,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54FighterKineticEnergyMotion__is_cliff_ground_trans_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn is_cliff_ground_trans(
            arg1: *mut root::app::FighterKineticEnergyMotion,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49FighterKineticEnergyMotion__set_ground_trans_implEPNS_26FighterKineticEnergyMotionE"]
        pub fn set_ground_trans(arg1: *mut root::app::FighterKineticEnergyMotion);
    }
}
pub mod ModelModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ModelModule__set_rotation_order_implEPNS_26BattleObjectModuleAccessorENS_21MotionNodeRotateOrderE"]
        pub fn set_rotation_order(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::MotionNodeRotateOrder,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ModelModule__rotation_order_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn rotation_order(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> root::app::MotionNodeRotateOrder;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23ModelModule__scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn scale(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ModelModule__set_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_scale(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ModelModule__scale_z_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn scale_z(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ModelModule__set_scale_z_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_scale_z(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ModelModule__set_temporary_scale_z_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_temporary_scale_z(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ModelModule__joint_global_position_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fEb"]
        pub fn joint_global_position(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
            arg4: bool,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51ModelModule__joint_global_position_with_offset_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fERS5_b"]
        pub fn joint_global_position_with_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: *mut root::phx::Vector3f,
            arg5: bool,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ModelModule__joint_global_offset_from_top_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn joint_global_offset_from_top(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54ModelModule__top_joint_global_position_from_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fERS5_"]
        pub fn top_joint_global_position_from_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: *mut root::phx::Vector3f,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ModelModule__joint_global_rotation_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fEb"]
        pub fn joint_global_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
            arg4: bool,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ModelModule__joint_global_axis_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eib"]
        pub fn joint_global_axis(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ModelModule__set_joint_srt_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fES7_S7_"]
        pub fn set_joint_srt(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: *const root::phx::Vector3f,
            arg5: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ModelModule__set_joint_scale_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fE"]
        pub fn set_joint_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ModelModule__set_joint_rotate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fENS_23MotionNodeRotateComposeENS_21MotionNodeRotateOrderE"]
        pub fn set_joint_rotate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: root::app::MotionNodeRotateCompose,
            arg5: root::app::MotionNodeRotateOrder,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ModelModule__set_joint_translate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERKNS3_8Vector3fEbb"]
        pub fn set_joint_translate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *const root::phx::Vector3f,
            arg4: bool,
            arg5: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ModelModule__clear_joint_srt_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn clear_joint_srt(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ModelModule__joint_rotate_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn joint_rotate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ModelModule__set_visibility_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ModelModule__is_visible_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_visible(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ModelModule__set_mesh_visibility_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn set_mesh_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ModelModule__set_alpha_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_alpha(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: f32);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ModelModule__set_color_rgb_implEPNS_26BattleObjectModuleAccessorEfffNS_16MODEL_COLOR_TYPEE"]
        pub fn set_color_rgb(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
            arg5: root::app::MODEL_COLOR_TYPE,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ModelModule__set_emmisive_scale_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_emmisive_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ModelModule__set_render_offset_position_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_render_offset_position(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ModelModule__set_depth_offset_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_depth_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ModelModule__set_depth_stencil_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_depth_stencil(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ModelModule__virtual_joint_tra_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn virtual_joint_tra(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ModelModule__enable_gold_eye_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn enable_gold_eye(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ModelModule__disable_gold_eye_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn disable_gold_eye(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
}
pub mod GimmickEventTornadoMoveInfo {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53GimmickEventTornadoMoveInfo__load_from_l2c_table_implEPNS_27GimmickEventTornadoMoveInfoERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventTornadoMoveInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GimmickEventTornadoMoveInfo__store_l2c_table_implEPKNS_27GimmickEventTornadoMoveInfoE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventTornadoMoveInfo,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49GimmickEventTornadoMoveInfo__store_l2c_table_implEPKNS_27GimmickEventTornadoMoveInfoERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventTornadoMoveInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod KineticModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30KineticModule__get_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticModule__clear_speed_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clear_speed_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41KineticModule__clear_speed_energy_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_speed_energy_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36KineticModule__clear_speed_attr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn clear_speed_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38KineticModule__suspend_energy_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn suspend_energy_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37KineticModule__resume_energy_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn resume_energy_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37KineticModule__unable_energy_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn unable_energy_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__get_sum_speed_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        /// Returns the current x velocity based on the specified kinetic energy attribute
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'kinetic_energy_reserve_attribute' - A KINETIC_ENERGY_RESERVE_ATTRIBUTE_ const
        ///
        /// # Example
        ///
        /// ```
        /// // get current x velocity
        /// let x_vel = KineticModule::get_sum_speed_x(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticModule__get_sum_speed_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_speed_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        /// Returns the current y velocity based on the specified kinetic energy attribute
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'kinetic_energy_reserve_attribute' - A KINETIC_ENERGY_RESERVE_ATTRIBUTE_ const
        ///
        /// # Example
        ///
        /// ```
        /// // get current y velocity
        /// let y_vel = KineticModule::get_sum_speed_y(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticModule__get_sum_speed_y_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_speed_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40KineticModule__get_sum_speed_length_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_speed_length(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        /// Returns the current velocity based on the specified kinetic energy attribute as a Vector3f
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'kinetic_energy_reserve_attribute' - A KINETIC_ENERGY_RESERVE_ATTRIBUTE_ const
        ///
        /// # Example
        ///
        /// ```
        /// // get current velocity as a Vector3f
        /// let vel_3f = KineticModule::get_sum_speed3f(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind35KineticModule__get_sum_speed3f_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_speed3f(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> root::phx::Vector3f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36KineticModule__get_sum_rotation_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_sum_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48KineticModule__set_consider_ground_friction_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_consider_ground_friction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        /// multiplies the current (specified type of) speed by the provided vector3f
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'Vector3f' - A reference to a smash::phx::Vector3f
        ///
        /// * 'kinetic_energy_id' - A KINETIC_ENERGY_ID const
        ///
        /// # Example
        ///
        /// ```
        /// // halt all vertical speed --
        /// let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
        /// KineticModule::mul_speed(module_accessor, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticModule__mul_speed_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn mul_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        /// multiplies the current (specified type of) acceleration by the provided vector3f
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'Vector3f' - A reference to a smash::phx::Vector3f
        ///
        /// * 'kinetic_energy_id' - A KINETIC_ENERGY_ID const
        ///
        /// # Example
        ///
        /// ```
        /// // send the character flying across the screen lol --
        /// let zoom  = smash::phx::Vector3f { x: 10.0, y: 1.0, z: 1.0 };
        /// KineticModule::mul_accel(module_accessor, &zoom, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticModule__mul_accel_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn mul_accel(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__reflect_speed_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn reflect_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__reflect_accel_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn reflect_accel(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        /// Changes the current KINETIC_TYPE to the specified value
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'kinetic_type' - a KINETIC_TYPE_ const
        ///
        /// # Example
        ///
        /// ``` change kinetic type to "fly"
        /// KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FLY);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind34KineticModule__change_kinetic_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn change_kinetic(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        /// Returns the current KINETIC_TYPE
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// let current_kinetic_type = KineticModule::get_kinetic_type(module_accessor);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind36KineticModule__get_kinetic_type_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_kinetic_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__enable_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn enable_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__unable_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unable_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36KineticModule__is_enable_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_enable_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34KineticModule__suspend_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn suspend_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33KineticModule__resume_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn resume_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37KineticModule__is_suspend_energy_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_suspend_energy(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        /// Adds the speed specified by the Vector3f to the battle object's current speed
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'speed_vector' - a reference to a Vector3f that specifies how much speed to add in what direction
        ///
        /// # Example
        ///
        /// ```
        /// // add 5 units of speed upwards
        /// let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
        /// KineticModule::add_speed(module_accessor, &speed);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind29KineticModule__add_speed_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn add_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37KineticModule__add_speed_outside_implEPNS_26BattleObjectModuleAccessorEiRKN3phx8Vector3fE"]
        pub fn add_speed_outside(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25KineticModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
}
pub mod BattleObjectManager {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54BattleObjectManager__is_active_find_battle_object_implEPNS_19BattleObjectManagerEj"]
        pub fn is_active_find_battle_object(
            arg1: *mut root::app::BattleObjectManager,
            arg2: libc::c_uint,
        ) -> bool;
    }
}
pub mod LinkEventStarShot {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkEventStarShot__load_from_l2c_table_implEPNS_17LinkEventStarShotERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventStarShot,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkEventStarShot__store_l2c_table_implEPKNS_17LinkEventStarShotE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventStarShot) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkEventStarShot__store_l2c_table_implEPKNS_17LinkEventStarShotERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventStarShot,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod stWaterAreaInfo {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41stWaterAreaInfo__load_from_l2c_table_implEPNS_15stWaterAreaInfoERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::stWaterAreaInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37stWaterAreaInfo__store_l2c_table_implEPKNS_15stWaterAreaInfoE"]
        pub fn store_l2c_table(arg1: *const root::app::stWaterAreaInfo) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37stWaterAreaInfo__store_l2c_table_implEPKNS_15stWaterAreaInfoERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::stWaterAreaInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventLadder {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventLadder__load_from_l2c_table_implEPNS_18GimmickEventLadderERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventLadder,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventLadder__store_l2c_table_implEPKNS_18GimmickEventLadderE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventLadder)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventLadder__store_l2c_table_implEPKNS_18GimmickEventLadderERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventLadder,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ItemKineticModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ItemKineticModuleImpl__set_throw_param_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fES6_bj"]
        pub fn set_throw_param(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: *const root::phx::Vector3f,
            arg4: bool,
            arg5: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42ItemKineticModuleImpl__set_slope_type_implEPNS_26BattleObjectModuleAccessorENS_13ItemSlopeTypeE"]
        pub fn set_slope_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemSlopeType,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ItemKineticModuleImpl__set_kinetic_flags_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_kinetic_flags(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ItemKineticModuleImpl__get_kinetic_flags_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_kinetic_flags(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ItemKineticModuleImpl__on_kinetic_flag_implEPNS_26BattleObjectModuleAccessorENS_15ItemKineticFlagE"]
        pub fn on_kinetic_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKineticFlag,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemKineticModuleImpl__off_kinetic_flag_implEPNS_26BattleObjectModuleAccessorENS_15ItemKineticFlagE"]
        pub fn off_kinetic_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKineticFlag,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54ItemKineticModuleImpl__add_speed_consider_gravity_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEb"]
        pub fn add_speed_consider_gravity(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemKineticModuleImpl__clear_speed_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn clear_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemKineticModuleImpl__add_speed_damage_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn add_speed_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemKineticModuleImpl__slope_angle_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn slope_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemKineticModuleImpl__set_motion_trans_rate_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_motion_trans_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemKineticModuleImpl__get_motion_trans_rate_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_motion_trans_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50ItemKineticModuleImpl__set_motion_trans_angle_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_motion_trans_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50ItemKineticModuleImpl__get_motion_trans_angle_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_motion_trans_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53ItemKineticModuleImpl__set_motion_trans_rate_2nd_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_motion_trans_rate_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind53ItemKineticModuleImpl__get_motion_trans_rate_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_motion_trans_rate_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54ItemKineticModuleImpl__set_motion_trans_angle_2nd_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_motion_trans_angle_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54ItemKineticModuleImpl__get_motion_trans_angle_2nd_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_motion_trans_angle_2nd(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemKineticModuleImpl__set_param_gemini_implEPNS_26BattleObjectModuleAccessorEffRKN3phx8Vector2fE"]
        pub fn set_param_gemini(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: *const root::phx::Vector2f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ItemKineticModuleImpl__get_sum_speed_simple_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn get_sum_speed_simple(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemKineticModuleImpl__set_rot_along_speed_x_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fEf"]
        pub fn set_rot_along_speed_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ItemKineticModuleImpl__it_ai_move_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector2fES6_S6_S6_S6_bb"]
        pub fn it_ai_move(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector2f,
            arg3: *const root::phx::Vector2f,
            arg4: *const root::phx::Vector2f,
            arg5: *const root::phx::Vector2f,
            arg6: *const root::phx::Vector2f,
            arg7: bool,
            arg8: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ItemKineticModuleImpl__it_ai_type_implEPNS_26BattleObjectModuleAccessorENS_10ItemAiTypeERKN3phx8Vector2fES7_"]
        pub fn it_ai_type(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemAiType,
            arg3: *const root::phx::Vector2f,
            arg4: *const root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemKineticModuleImpl__it_ai_dir_factor_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn it_ai_dir_factor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemKineticModuleImpl__it_ai_distance_factor_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn it_ai_distance_factor(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ItemKineticModuleImpl__it_ai_dir_rot_mode_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn it_ai_dir_rot_mode(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemKineticModuleImpl__it_base_rot_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn it_base_rot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ItemKineticModuleImpl__set_interpolate_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_interpolate_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
}
pub mod ItemModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ItemModule__have_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEiibb"]
        pub fn have_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: libc::c_int,
            arg4: libc::c_int,
            arg5: bool,
            arg6: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ItemModule__have_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEibbbb"]
        pub fn have_item_instance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::app::Item,
            arg3: libc::c_int,
            arg4: bool,
            arg5: bool,
            arg6: bool,
            arg7: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ItemModule__use_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb"]
        pub fn use_item_instance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::app::Item,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25ItemModule__use_item_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn use_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ItemModule__pickup_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemSizeEiiNS_18QuickItemTreatTypeENS_20ItemPickupSearchModeE"]
        pub fn pickup_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemSize,
            arg3: libc::c_int,
            arg4: libc::c_int,
            arg5: root::app::QuickItemTreatType,
            arg6: root::app::ItemPickupSearchMode,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemModule__is_success_pickup_item_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_success_pickup_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ItemModule__success_auto_pickup_item_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn success_auto_pickup_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemModule__is_success_auto_pickup_item_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_success_auto_pickup_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ItemModule__is_have_item_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_have_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ItemModule__get_have_item_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_have_item_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ItemModule__get_have_item_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_have_item_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36ItemModule__get_have_item_trait_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_have_item_trait(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ItemModule__get_have_item_hold_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_have_item_hold_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47ItemModule__get_have_item_status_param_int_implEPNS_26BattleObjectModuleAccessorENS_18ItemStatusParamIntEi"]
        pub fn get_have_item_status_param_int(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemStatusParamInt,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49ItemModule__get_have_item_status_param_float_implEPNS_26BattleObjectModuleAccessorENS_20ItemStatusParamFloatEi"]
        pub fn get_have_item_status_param_float(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemStatusParamFloat,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ItemModule__get_have_item_status_param_bool_implEPNS_26BattleObjectModuleAccessorENS_19ItemStatusParamBoolEi"]
        pub fn get_have_item_status_param_bool(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemStatusParamBool,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ItemModule__get_have_item_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_have_item_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44ItemModule__get_pickable_item_object_id_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_pickable_item_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemModule__get_pickable_item_size_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_pickable_item_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemModule__get_pickable_item_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_pickable_item_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ItemModule__get_pickable_item_trait_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_pickable_item_trait(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ItemModule__remove_item_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn remove_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ItemModule__remove_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn remove_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27ItemModule__throw_item_implEPNS_26BattleObjectModuleAccessorEfffibf"]
        pub fn throw_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: f32,
            arg5: libc::c_int,
            arg6: bool,
            arg7: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ItemModule__drop_item_implEPNS_26BattleObjectModuleAccessorEffi"]
        pub fn drop_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: f32,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ItemModule__warp_item_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fEi"]
        pub fn warp_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ItemModule__warp_attach_item_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn warp_attach_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26ItemModule__born_item_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn born_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ItemModule__shoot_item_bullet_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn shoot_item_bullet(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ItemModule__shoot_item_bullet_blanks_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn shoot_item_bullet_blanks(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ItemModule__get_shoot_item_bullet_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_shoot_item_bullet(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45ItemModule__update_have_item_action_info_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn update_have_item_action_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ItemModule__set_have_item_scale_anim_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_have_item_scale_anim(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ItemModule__set_have_item_action_implEPNS_26BattleObjectModuleAccessorEifi"]
        pub fn set_have_item_action(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41ItemModule__set_have_item_visibility_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_have_item_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ItemModule__set_have_item_hold_anim_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_have_item_hold_anim(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47ItemModule__set_have_item_constraint_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Ei"]
        pub fn set_have_item_constraint_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ItemModule__reset_have_item_constraint_node_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn reset_have_item_constraint_node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ItemModule__set_have_item_hit_sleep_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_have_item_hit_sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ItemModule__attach_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEib"]
        pub fn attach_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: libc::c_int,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30ItemModule__attach_item_2_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb"]
        pub fn attach_item_2(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::app::Item,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ItemModule__attach_item_instance_implEPNS_26BattleObjectModuleAccessorEPNS_4ItemEb"]
        pub fn attach_item_instance(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::app::Item,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31ItemModule__is_attach_item_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindE"]
        pub fn is_attach_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38ItemModule__get_attach_item_count_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_attach_item_count(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ItemModule__set_attach_item_visibility_implEPNS_26BattleObjectModuleAccessorEbh"]
        pub fn set_attach_item_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ItemModule__eject_have_item_implEPNS_26BattleObjectModuleAccessorEibb"]
        pub fn eject_have_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ItemModule__eject_attach_item_implEPNS_26BattleObjectModuleAccessorEibbb"]
        pub fn eject_attach_item(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: bool,
            arg5: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ItemModule__eject_attach_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEbb"]
        pub fn eject_attach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: bool,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28ItemModule__drop_attach_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEff"]
        pub fn drop_attach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: f32,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34ItemModule__drop_attach_group_implEPNS_26BattleObjectModuleAccessorEhff"]
        pub fn drop_attach_group(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uchar,
            arg3: f32,
            arg4: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29ItemModule__scale_attach_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEf"]
        pub fn scale_attach(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: f32,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ItemModule__set_attach_item_action_implEPNS_26BattleObjectModuleAccessorENS_8ItemKindEif"]
        pub fn set_attach_item_action(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::ItemKind,
            arg3: libc::c_int,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ItemModule__set_have_item_team_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_have_item_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ItemModule__set_change_status_event_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_change_status_event(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}
pub mod FighterCloudLinkEventFinal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52FighterCloudLinkEventFinal__load_from_l2c_table_implEPNS_26FighterCloudLinkEventFinalERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterCloudLinkEventFinal,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterCloudLinkEventFinal__store_l2c_table_implEPKNS_26FighterCloudLinkEventFinalE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterCloudLinkEventFinal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48FighterCloudLinkEventFinal__store_l2c_table_implEPKNS_26FighterCloudLinkEventFinalERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterCloudLinkEventFinal,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod ItemMotionAnimcmdModuleImpl {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ItemMotionAnimcmdModuleImpl__set_fix_rate_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_fix_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
}
pub mod StatusModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40StatusModule__change_status_request_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn change_status_request(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        /// Changes the current status_kind for the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'status_kind' - the STATUS_KIND const to change to
        ///
        /// * 'unk' - we dunno
        ///
        /// # Example
        ///
        /// ```
        /// // if your are doing an aerial and your current frame is 5 or fewer frames away from the end of the anim, transition to FALL
        /// if MotionModule::end_frame(module_accessor) - MotionModule::frame(module_accessor) <= 5 {
        ///     StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind52StatusModule__change_status_request_from_script_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn change_status_request_from_script(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind52StatusModule__delete_status_request_from_script_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn delete_status_request_from_script(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46StatusModule__status_kind_que_from_script_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn status_kind_que_from_script(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        /// Returns current status kind of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // fighter is shielding
        /// if StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_GUARD {
        ///     StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind30StatusModule__status_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn status_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35StatusModule__status_kind_next_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn status_kind_next(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44StatusModule__set_status_kind_interrupt_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_status_kind_interrupt(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40StatusModule__status_kind_interrupt_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn status_kind_interrupt(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30StatusModule__is_changing_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_changing(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        /// Returns one of the previous status kinds of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        /// * `index` - index of prev status list
        ///
        /// # Example
        ///
        /// ```
        /// // fighter was shielding for its last status
        /// if StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_GUARD {
        ///     StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind35StatusModule__prev_status_kind_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn prev_status_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            index: libc::c_uint,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38StatusModule__change_status_force_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn change_status_force(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        /// Returns situation kind of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // is in air
        /// if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_AIR {
        ///     // ...
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind33StatusModule__situation_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn situation_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        /// Returns previous situation kind of the object
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// # Example
        ///
        /// ```
        /// // was in air
        /// if StatusModule::prev_situation_kind(module_accessor) == SITUATION_KIND_AIR {
        ///     // ...
        /// }
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind38StatusModule__prev_situation_kind_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn prev_situation_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39StatusModule__is_situation_changed_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_situation_changed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37StatusModule__set_situation_kind_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEb"]
        pub fn set_situation_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::SituationKind,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41StatusModule__set_keep_situation_air_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_keep_situation_air(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35StatusModule__set_succeeds_bit_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_succeeds_bit(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32StatusModule__init_settings_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEijNS_20GroundCliffCheckKindEbiiii"]
        pub fn init_settings(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::SituationKind,
            arg3: libc::c_int,
            arg4: libc::c_uint,
            arg5: root::app::GroundCliffCheckKind,
            arg6: bool,
            arg7: libc::c_int,
            arg8: libc::c_int,
            arg9: libc::c_int,
            arg10: libc::c_int,
        ) -> u64;
    }
}
pub mod VisibilityModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26VisibilityModule__set_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_"]
        pub fn set(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32VisibilityModule__set_int64_implEPNS_26BattleObjectModuleAccessorEll"]
        pub fn set_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
            arg3: libc::c_long,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41VisibilityModule__set_status_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_"]
        pub fn set_status_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47VisibilityModule__set_status_default_int64_implEPNS_26BattleObjectModuleAccessorEll"]
        pub fn set_status_default_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
            arg3: libc::c_long,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43VisibilityModule__reset_status_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn reset_status_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49VisibilityModule__reset_status_default_int64_implEPNS_26BattleObjectModuleAccessorEl"]
        pub fn reset_status_default_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47VisibilityModule__reset_status_default_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn reset_status_default_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38VisibilityModule__set_default_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn set_default_all(module_accessor: *mut root::app::BattleObjectModuleAccessor);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42VisibilityModule__set_mesh_visibility_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eib"]
        pub fn set_mesh_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48VisibilityModule__set_mesh_visibility_int64_implEPNS_26BattleObjectModuleAccessorElib"]
        pub fn set_mesh_visibility_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind49VisibilityModule__set_material_anim_priority_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Eb"]
        pub fn set_material_anim_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32VisibilityModule__set_whole_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32VisibilityModule__get_whole_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_whole(module_accessor: *mut root::app::BattleObjectModuleAccessor)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33VisibilityModule__is_visible_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_visible(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34VisibilityModule__set_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40VisibilityModule__set_default_int64_implEPNS_26BattleObjectModuleAccessorEl"]
        pub fn set_default_int64(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_long,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38VisibilityModule__is_visible_mesh_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_visible_mesh(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42VisibilityModule__set_visibility_mode_implEPNS_26BattleObjectModuleAccessorENS_14VisibilityModeE"]
        pub fn set_visibility_mode(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::VisibilityMode,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40VisibilityModule__set_model_visible_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_model_visible(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}
pub mod ColorBlendModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32ColorBlendModule__off_flash_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn off_flash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ColorBlendModule__set_enable_flash_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_enable_flash(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ColorBlendModule__set_burn_color_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector4fEb"]
        pub fn set_burn_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector4f,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43ColorBlendModule__set_burn_color_frame_implEPNS_26BattleObjectModuleAccessorEfRKN3phx8Vector4fEb"]
        pub fn set_burn_color_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: *const root::phx::Vector4f,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ColorBlendModule__off_burn_color_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn off_burn_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46ColorBlendModule__set_burn_color_priority_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_burn_color_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35ColorBlendModule__set_priority_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_priority(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39ColorBlendModule__set_shadow_bloom_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_shadow_bloom(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48ColorBlendModule__set_last_attack_direction_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_last_attack_direction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37ColorBlendModule__set_main_color_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector4fES6_ffib"]
        pub fn set_main_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector4f,
            arg3: *const root::phx::Vector4f,
            arg4: f32,
            arg5: f32,
            arg6: libc::c_int,
            arg7: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40ColorBlendModule__cancel_main_color_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn cancel_main_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ColorBlendModule__set_status_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_status(module_accessor: *mut root::app::BattleObjectModuleAccessor, arg2: bool);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33ColorBlendModule__get_status_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57ColorBlendModule__set_disable_camera_depth_influence_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_disable_camera_depth_influence(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
}
pub mod HitModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21HitModule__clean_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn clean(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21HitModule__sleep_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn sleep(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26HitModule__set_status_implEPNS_26BattleObjectModuleAccessorEiNS_9HitStatusEi"]
        pub fn set_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::HitStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32HitModule__set_status_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS_9HitStatusEi"]
        pub fn set_status_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::app::HitStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40HitModule__set_status_joint_default_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ENS_9HitStatusEi"]
        pub fn set_status_joint_default(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::app::HitStatus,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30HitModule__set_status_all_implEPNS_26BattleObjectModuleAccessorENS_9HitStatusEi"]
        pub fn set_status_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::HitStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        /// Sets the current "hit status"... I.E. HIT_STATUS_NORMAL, HIT_STATUS_XLU, or HIT_STATUS_INVINCIBLE
        ///
        /// # Arguments
        ///
        /// * `module_accessor` - Pointer to BattleObjectModuleAccessor
        ///
        /// * 'HitStatus' - a transparent struct to denote a HIT_STATUS const
        ///
        /// # Example
        ///
        /// ```
        /// // set character to be entirely intangible
        /// HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        /// ```
        #[link_name = "\u{1}_ZN3app8lua_bind25HitModule__set_whole_implEPNS_26BattleObjectModuleAccessorENS_9HitStatusEi"]
        pub fn set_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::HitStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25HitModule__get_whole_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_whole(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31HitModule__set_check_catch_implEPNS_26BattleObjectModuleAccessorEbi"]
        pub fn set_check_catch(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30HitModule__set_xlu_global_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn set_xlu_global(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33HitModule__cancel_xlu_global_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn cancel_xlu_global(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36HitModule__set_xlu_frame_global_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn set_xlu_frame_global(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43HitModule__set_invincible_frame_global_implEPNS_26BattleObjectModuleAccessorEibi"]
        pub fn set_invincible_frame_global(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
            arg4: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26HitModule__get_status_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32HitModule__get_total_status_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_total_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41HitModule__get_total_status_disguise_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_total_status_disguise(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41HitModule__set_total_status_disguise_implEPNS_26BattleObjectModuleAccessorENS_9HitStatusEi"]
        pub fn set_total_status_disguise(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::HitStatus,
            arg3: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind29HitModule__get_part_size_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_part_size(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27HitModule__set_no_team_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_no_team(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32HitModule__set_hit_stop_mul_implEPNS_26BattleObjectModuleAccessorEfNS_16HitStopMulTargetEf"]
        pub fn set_hit_stop_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: root::app::HitStopMulTarget,
            arg4: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30HitModule__get_center_pos_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38HitModule__set_defense_mul_status_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_defense_mul_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind25HitModule__exist_log_implEPNS_26BattleObjectModuleAccessorEji"]
        pub fn exist_log(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: libc::c_int,
        ) -> u64;
    }
}
pub mod WeaponRobotHominglaserLinkEventBurst {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62WeaponRobotHominglaserLinkEventBurst__load_from_l2c_table_implEPNS_36WeaponRobotHominglaserLinkEventBurstERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponRobotHominglaserLinkEventBurst,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponRobotHominglaserLinkEventBurst__store_l2c_table_implEPKNS_36WeaponRobotHominglaserLinkEventBurstE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponRobotHominglaserLinkEventBurst,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponRobotHominglaserLinkEventBurst__store_l2c_table_implEPKNS_36WeaponRobotHominglaserLinkEventBurstERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponRobotHominglaserLinkEventBurst,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventMask {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkEventMask__load_from_l2c_table_implEPNS_13LinkEventMaskERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventMask,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkEventMask__store_l2c_table_implEPKNS_13LinkEventMaskE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventMask) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkEventMask__store_l2c_table_implEPKNS_13LinkEventMaskERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventMask,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod WeaponSnakeMissileKineticEnergyNormal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind57WeaponSnakeMissileKineticEnergyNormal__set_direction_implEPNS_37WeaponSnakeMissileKineticEnergyNormalERKN3phx8Vector3fE"]
        pub fn set_direction(
            arg1: *mut root::app::WeaponSnakeMissileKineticEnergyNormal,
            arg2: *const root::phx::Vector3f,
        );
    }
}
pub mod LinkModule {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind21LinkModule__link_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn link(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27LinkModule__unlink_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn unlink_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28LinkModule__unlink_node_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unlink_node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32LinkModule__unlink_node_all_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn unlink_node_all(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23LinkModule__unlink_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn unlink(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind24LinkModule__is_link_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_link(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind26LinkModule__is_linked_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_linked(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__get_parent_id_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn get_parent_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__search_parent_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn search_parent(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__search_parent_attr_implEPNS_26BattleObjectModuleAccessorENS_13LinkAttributeE"]
        pub fn search_parent_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LinkAttribute,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28LinkModule__search_node_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn search_node(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__send_event_parents_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40E"]
        pub fn send_event_parents(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__send_event_parents_struct_implEPNS_26BattleObjectModuleAccessorEiRNS_9LinkEventE"]
        pub fn send_event_parents_struct(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *mut root::app::LinkEvent,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkModule__send_event_nodes_struct_implEPNS_26BattleObjectModuleAccessorEiRNS_9LinkEventEj"]
        pub fn send_event_nodes_struct(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *mut root::app::LinkEvent,
            arg4: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33LinkModule__send_event_nodes_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Ej"]
        pub fn send_event_nodes(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkModule__send_event_nodes_throw_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40ES4_biii"]
        pub fn send_event_nodes_throw(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
            arg3: root::phx::Hash40,
            arg4: bool,
            arg5: libc::c_int,
            arg6: libc::c_int,
            arg7: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__get_node_object_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_node_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkModule__is_node_damage_reaction_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_node_damage_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__is_node_damage_capture_cut_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_node_damage_capture_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__chk_link_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn chk_link_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__is_valid_link_stop_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_valid_link_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__chk_link_visibility_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn chk_link_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__get_link_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_link_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind28LinkModule__is_link_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_link_pos(module_accessor: *mut root::app::BattleObjectModuleAccessor) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind32LinkModule__update_link_pos_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn update_link_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__get_link_speed_implEPNS_26BattleObjectModuleAccessorERN3phx8Vector2fE"]
        pub fn get_link_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *mut root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33LinkModule__get_parent_scale_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55LinkModule__get_parent_model_joint_global_position_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Eb"]
        pub fn get_parent_model_joint_global_position(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55LinkModule__get_parent_model_joint_global_rotation_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40Eb"]
        pub fn get_parent_model_joint_global_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkModule__get_parent_motion_frame_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_motion_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45LinkModule__get_parent_motion_whole_rate_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_motion_whole_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkModule__get_parent_motion_rate_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_motion_rate(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkModule__get_parent_motion_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_motion_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__get_parent_situation_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_situation_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkModule__get_parent_status_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_status_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__get_parent_lr_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_lr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__get_parent_sum_speed_implEPNS_26BattleObjectModuleAccessorEii"]
        pub fn get_parent_sum_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__get_parent_object_id_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51LinkModule__is_capture_damage_parent_object_id_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn is_capture_damage_parent_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__get_parent_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__get_parent_prev_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_prev_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__get_parent_rot_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_rot(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34LinkModule__get_parent_damage_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_damage(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__get_parent_damage_reaction_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_damage_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__get_parent_damage_power_max_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_damage_power_max(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__get_parent_top_angle_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_top_angle(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41LinkModule__set_node_scale_power_mul_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_node_scale_power_mul(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__set_node_link_scale_implEPNS_26BattleObjectModuleAccessorEfb"]
        pub fn set_node_link_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__is_parent_damage_reaction_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_damage_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__is_parent_damage_catch_cut_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_damage_catch_cut(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__get_parent_total_hit_status_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_total_hit_status(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkModule__is_valid_parent_shape_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_valid_parent_shape(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__get_parent_shape_center_pos_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_shape_center_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind54LinkModule__get_parent_shape_center_pos_object_id_implEPNS_26BattleObjectModuleAccessorEjRN3phx8Vector2fE"]
        pub fn get_parent_shape_center_pos_object_id(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
            arg3: *mut root::phx::Vector2f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__is_parent_effect_sync_scale_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_effect_sync_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33LinkModule__is_link_power_up_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_link_power_up(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__get_parent_power_up_attack_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_power_up_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__get_parent_power_up_defense_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_power_up_defense(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41LinkModule__get_parent_power_up_attr_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_power_up_attr(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkModule__get_parent_customize_attack_ratio_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_customize_attack_ratio(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__is_link_node_attack_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_link_node_attack(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__get_parent_reaction_mul_4th_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_reaction_mul_4th(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__is_parent_power_up_reaction_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_parent_power_up_reaction(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__set_parent_hit_stop_frame_implEPNS_26BattleObjectModuleAccessorEiib"]
        pub fn set_parent_hit_stop_frame(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_int,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41LinkModule__set_parent_attacker_info_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_parent_attacker_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind39LinkModule__set_node_attacker_info_implEPNS_26BattleObjectModuleAccessorEij"]
        pub fn set_node_attacker_info(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45LinkModule__set_parent_log_damage_player_implEPNS_26BattleObjectModuleAccessorEijii"]
        pub fn set_parent_log_damage_player(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
            arg4: libc::c_int,
            arg5: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__set_node_log_damage_player_implEPNS_26BattleObjectModuleAccessorEijii"]
        pub fn set_node_log_damage_player(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: libc::c_uint,
            arg4: libc::c_int,
            arg5: libc::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkModule__set_node_depth_offset_implEPNS_26BattleObjectModuleAccessorEif"]
        pub fn set_node_depth_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkModule__get_parent_have_item_visibility_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_have_item_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkModule__get_parent_attach_item_visibility_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_attach_item_visibility(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkModule__get_parent_main_color_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_main_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__is_parent_enable_sub_color_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_enable_sub_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__get_parent_sub_color_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_sub_color(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind46LinkModule__get_parent_battle_object_kind_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_battle_object_kind(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60LinkModule__get_parent_object_id_motion_transactor_link_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_parent_object_id_motion_transactor_link(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45LinkModule__set_model_constraint_pos_ort_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ES4_jb"]
        pub fn set_model_constraint_pos_ort(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: root::phx::Hash40,
            arg5: libc::c_uint,
            arg6: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47LinkModule__set_model_constraint_attribute_implEPNS_26BattleObjectModuleAccessorEib"]
        pub fn set_model_constraint_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__get_model_constraint_flag_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__set_model_constraint_flag_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn set_model_constraint_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__off_model_constraint_flag_implEPNS_26BattleObjectModuleAccessorEj"]
        pub fn off_model_constraint_flag(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_uint,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__get_model_constraint_joint_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkModule__get_model_constraint_target_joint_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_target_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__set_model_constraint_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_model_constraint_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkModule__set_model_constraint_target_joint_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn set_model_constraint_target_joint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkModule__get_model_constraint_no_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_no(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkModule__adjust_model_constraint_posture_implEPNS_26BattleObjectModuleAccessorEPKN3phx8Vector3fES6_"]
        pub fn adjust_model_constraint_posture(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
            arg3: *const root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40LinkModule__remove_model_constraint_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn remove_model_constraint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__is_model_constraint_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_model_constraint(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__is_model_constraint_mutual_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_model_constraint_mutual(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58LinkModule__get_model_constraint_target_node_position_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_target_node_position(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59LinkModule__get_model_constraint_joint_global_position_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn get_model_constraint_joint_global_position(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind69LinkModule__get_model_constraint_joint_global_position_recursive_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub fn get_model_constraint_joint_global_position_recursive(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59LinkModule__get_model_constraint_target_joint_rotation_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_model_constraint_target_joint_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34LinkModule__get_node_rotation_implEPNS_26BattleObjectModuleAccessorEiN3phx6Hash40ERNS3_8Vector3fE"]
        pub fn get_node_rotation(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::phx::Hash40,
            arg4: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind43LinkModule__get_constraint_model_scale_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_constraint_model_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkModule__get_constraint_translate_offset_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn get_constraint_translate_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind50LinkModule__is_change_model_constraint_matrix_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn is_change_model_constraint_matrix(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__set_attribute_implEPNS_26BattleObjectModuleAccessorEiNS_13LinkAttributeEb"]
        pub fn set_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::LinkAttribute,
            arg4: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30LinkModule__chk_attribute_implEPNS_26BattleObjectModuleAccessorEiNS_13LinkAttributeE"]
        pub fn chk_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: root::app::LinkAttribute,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__chk_linked_attribute_implEPNS_26BattleObjectModuleAccessorENS_13LinkAttributeE"]
        pub fn chk_linked_attribute(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: root::app::LinkAttribute,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind36LinkModule__is_link_parent_slow_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_link_parent_slow(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__is_parent_flip_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_flip(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__is_parent_hit_stop_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_hit_stop(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__get_parent_stick_x_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_stick_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37LinkModule__get_node_catprue_pos_implEPNS_26BattleObjectModuleAccessorEiRN3phx8Vector3fE"]
        pub fn get_node_catprue_pos(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
            arg3: *mut root::phx::Vector3f,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__get_node_sum_speed_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_node_sum_speed(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind31LinkModule__get_node_scale_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_node_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__set_constraint_rot_offset_x_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_constraint_rot_offset_x(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__set_constraint_rot_offset_y_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_constraint_rot_offset_y(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__set_constraint_rot_offset_z_implEPNS_26BattleObjectModuleAccessorEf"]
        pub fn set_constraint_rot_offset_z(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind42LinkModule__set_constraint_rot_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_constraint_rot_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__set_constraint_scale_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_constraint_scale_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind48LinkModule__set_constraint_translate_offset_implEPNS_26BattleObjectModuleAccessorERKN3phx8Vector3fE"]
        pub fn set_constraint_translate_offset(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: *const root::phx::Vector3f,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind45LinkModule__set_unreference_parent_scale_implEPNS_26BattleObjectModuleAccessorEb"]
        pub fn set_unreference_parent_scale(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: bool,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44LinkModule__model_constraint_node_top_z_implEPNS_26BattleObjectModuleAccessorE"]
        pub fn model_constraint_node_top_z(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind35LinkModule__is_parent_spycloak_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn is_parent_spycloak(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind41LinkModule__get_parent_model_visible_implEPNS_26BattleObjectModuleAccessorEi"]
        pub fn get_parent_model_visible(
            module_accessor: *mut root::app::BattleObjectModuleAccessor,
            arg2: libc::c_int,
        ) -> u64;
    }
}
pub mod WeaponShizueFishingrodLinkEventCut {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind60WeaponShizueFishingrodLinkEventCut__load_from_l2c_table_implEPNS_34WeaponShizueFishingrodLinkEventCutERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponShizueFishingrodLinkEventCut,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56WeaponShizueFishingrodLinkEventCut__store_l2c_table_implEPKNS_34WeaponShizueFishingrodLinkEventCutE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventCut,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind56WeaponShizueFishingrodLinkEventCut__store_l2c_table_implEPKNS_34WeaponShizueFishingrodLinkEventCutERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventCut,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod WeaponSnakeNikitaMissileKineticEnergyNormal {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind59WeaponSnakeNikitaMissileKineticEnergyNormal__set_rot_z_implEPNS_43WeaponSnakeNikitaMissileKineticEnergyNormalEf"]
        pub fn set_rot_z(
            arg1: *mut root::app::WeaponSnakeNikitaMissileKineticEnergyNormal,
            arg2: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind55WeaponSnakeNikitaMissileKineticEnergyNormal__rot_z_implEPNS_43WeaponSnakeNikitaMissileKineticEnergyNormalE"]
        pub fn rot_z(
            arg1: *mut root::app::WeaponSnakeNikitaMissileKineticEnergyNormal,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind64WeaponSnakeNikitaMissileKineticEnergyNormal__set_enable_rot_implEPNS_43WeaponSnakeNikitaMissileKineticEnergyNormalEb"]
        pub fn set_enable_rot(
            arg1: *mut root::app::WeaponSnakeNikitaMissileKineticEnergyNormal,
            arg2: bool,
        );
    }
}
pub mod WeaponShizueFishingrodLinkEventShoot {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind62WeaponShizueFishingrodLinkEventShoot__load_from_l2c_table_implEPNS_36WeaponShizueFishingrodLinkEventShootERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::WeaponShizueFishingrodLinkEventShoot,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponShizueFishingrodLinkEventShoot__store_l2c_table_implEPKNS_36WeaponShizueFishingrodLinkEventShootE"]
        pub fn store_l2c_table(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventShoot,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind58WeaponShizueFishingrodLinkEventShoot__store_l2c_table_implEPKNS_36WeaponShizueFishingrodLinkEventShootERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::WeaponShizueFishingrodLinkEventShoot,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEvent {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38GimmickEvent__load_from_l2c_table_implEPNS_12GimmickEventERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GimmickEvent__store_l2c_table_implEPKNS_12GimmickEventE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEvent) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34GimmickEvent__store_l2c_table_implEPKNS_12GimmickEventERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEvent,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventSlashLockInfo {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind51GimmickEventSlashLockInfo__load_from_l2c_table_implEPNS_25GimmickEventSlashLockInfoERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventSlashLockInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GimmickEventSlashLockInfo__store_l2c_table_implEPKNS_25GimmickEventSlashLockInfoE"]
        pub fn store_l2c_table(
            arg1: *const root::app::GimmickEventSlashLockInfo,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind47GimmickEventSlashLockInfo__store_l2c_table_implEPKNS_25GimmickEventSlashLockInfoERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventSlashLockInfo,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterPikminLinkEventWeaponPikminChangeStatus {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind72FighterPikminLinkEventWeaponPikminChangeStatus__load_from_l2c_table_implEPNS_46FighterPikminLinkEventWeaponPikminChangeStatusERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::FighterPikminLinkEventWeaponPikminChangeStatus,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterPikminLinkEventWeaponPikminChangeStatus__store_l2c_table_implEPKNS_46FighterPikminLinkEventWeaponPikminChangeStatusE"]
        pub fn store_l2c_table(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminChangeStatus,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind68FighterPikminLinkEventWeaponPikminChangeStatus__store_l2c_table_implEPKNS_46FighterPikminLinkEventWeaponPikminChangeStatusERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::FighterPikminLinkEventWeaponPikminChangeStatus,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod GimmickEventSpring {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind44GimmickEventSpring__load_from_l2c_table_implEPNS_18GimmickEventSpringERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::GimmickEventSpring,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventSpring__store_l2c_table_implEPKNS_18GimmickEventSpringE"]
        pub fn store_l2c_table(arg1: *const root::app::GimmickEventSpring)
            -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind40GimmickEventSpring__store_l2c_table_implEPKNS_18GimmickEventSpringERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::GimmickEventSpring,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod LinkEventPos {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind38LinkEventPos__load_from_l2c_table_implEPNS_12LinkEventPosERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::LinkEventPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34LinkEventPos__store_l2c_table_implEPKNS_12LinkEventPosE"]
        pub fn store_l2c_table(arg1: *const root::app::LinkEventPos) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34LinkEventPos__store_l2c_table_implEPKNS_12LinkEventPosERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::LinkEventPos,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod Rhombus2 {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind34Rhombus2__load_from_l2c_table_implEPNS_8Rhombus2ERKN3lib8L2CValueE"]
        pub fn load_from_l2c_table(
            arg1: *mut root::app::Rhombus2,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30Rhombus2__store_l2c_table_implEPKNS_8Rhombus2E"]
        pub fn store_l2c_table(arg1: *const root::app::Rhombus2) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30Rhombus2__store_l2c_table_implEPKNS_8Rhombus2ERKN3lib8L2CValueE"]
        pub fn store_l2c_table1(
            arg1: *const root::app::Rhombus2,
            arg2: *const root::lib::L2CValue,
        ) -> u64;
    }
}
pub mod FighterEntry {
    #[allow(unused_imports)]
    use super::super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind30FighterEntry__fighter_num_implEPNS_12FighterEntryE"]
        pub fn fighter_num(arg1: *mut root::app::FighterEntry) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind33FighterEntry__get_fighter_id_implEPNS_12FighterEntryEib"]
        pub fn get_fighter_id(
            arg1: *mut root::app::FighterEntry,
            arg2: libc::c_int,
            arg3: bool,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind37FighterEntry__current_fighter_id_implEPNS_12FighterEntryE"]
        pub fn current_fighter_id(arg1: *mut root::app::FighterEntry) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind23FighterEntry__heal_implEPNS_12FighterEntryEfbiN3phx6Hash40E"]
        pub fn heal(
            arg1: *mut root::app::FighterEntry,
            arg2: f32,
            arg3: bool,
            arg4: libc::c_int,
            arg5: root::phx::Hash40,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app8lua_bind27FighterEntry__eat_item_implEPNS_12FighterEntryERNS_18LinkEventTouchItemEb"]
        pub fn eat_item(
            arg1: *mut root::app::FighterEntry,
            arg2: *mut root::app::LinkEventTouchItem,
            arg3: bool,
        ) -> u64;
    }
}

pub mod sv_kinetic_energy {
    #[allow(unused_imports)]
    use super::super::super::root;
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11clear_speedEP9lua_State"]
        pub fn clear_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11friction_onEP9lua_State"]
        pub fn friction_on(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_accel_yEP9lua_State"]
        pub fn get_accel_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_brake_xEP9lua_State"]
        pub fn get_brake_x(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_brake_yEP9lua_State"]
        pub fn get_brake_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_speed3fEP9lua_State"]
        pub fn get_speed3f(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_speed_xEP9lua_State"]
        pub fn get_speed_x(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy11get_speed_yEP9lua_State"]
        pub fn get_speed_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy12friction_offEP9lua_State"]
        pub fn friction_off(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy12get_rotationEP9lua_State"]
        pub fn get_rotation(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy12reset_energyEP9lua_State"]
        pub fn reset_energy(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy13get_speed_mulEP9lua_State"]
        pub fn get_speed_mul(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy13set_chara_dirEP9lua_State"]
        pub fn set_chara_dir(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy13set_speed_mulEP9lua_State"]
        pub fn set_speed_mul(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy14clear_speed_exEP9lua_State"]
        pub fn clear_speed_ex(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy14get_rotation_xEP9lua_State"]
        pub fn get_rotation_x(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy14get_rotation_yEP9lua_State"]
        pub fn get_rotation_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy14get_rotation_zEP9lua_State"]
        pub fn get_rotation_z(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15mul_x_accel_addEP9lua_State"]
        pub fn mul_x_accel_add(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15mul_x_accel_mulEP9lua_State"]
        pub fn mul_x_accel_mul(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15mul_x_speed_maxEP9lua_State"]
        pub fn mul_x_speed_max(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15set_accel_x_addEP9lua_State"]
        pub fn set_accel_x_add(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15set_accel_x_mulEP9lua_State"]
        pub fn set_accel_x_mul(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15set_accel_y_addEP9lua_State"]
        pub fn set_accel_y_add(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15set_accel_y_mulEP9lua_State"]
        pub fn set_accel_y_mul(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy15set_limit_speedEP9lua_State"]
        pub fn set_limit_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy16get_speed_lengthEP9lua_State"]
        pub fn get_speed_length(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy16set_damage_speedEP9lua_State"]
        pub fn set_damage_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy16set_ground_transEP9lua_State"]
        pub fn set_ground_trans(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy16set_stable_speedEP9lua_State"]
        pub fn set_stable_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy17get_limit_speed_xEP9lua_State"]
        pub fn get_limit_speed_x(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy17get_limit_speed_yEP9lua_State"]
        pub fn get_limit_speed_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy17set_speed_mul_2ndEP9lua_State"]
        pub fn set_speed_mul_2nd(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy18get_stable_speed_xEP9lua_State"]
        pub fn get_stable_speed_x(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy18get_stable_speed_yEP9lua_State"]
        pub fn get_stable_speed_y(arg1: u64) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy20is_gravity_fix_accelEP9lua_State"]
        pub fn is_gravity_fix_accel(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy21is_cliff_ground_transEP9lua_State"]
        pub fn is_cliff_ground_trans(arg1: u64) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy23set_gravity_coefficientEP9lua_State"]
        pub fn set_gravity_coefficient(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy26controller_set_accel_x_addEP9lua_State"]
        pub fn controller_set_accel_x_add(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy26controller_set_accel_x_mulEP9lua_State"]
        pub fn controller_set_accel_x_mul(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy29set_motion_energy_update_flagEP9lua_State"]
        pub fn set_motion_energy_update_flag(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy6enableEP9lua_State"]
        pub fn enable(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy6resumeEP9lua_State"]
        pub fn resume(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy6unableEP9lua_State"]
        pub fn unable(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy7suspendEP9lua_State"]
        pub fn suspend(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9add_speedEP9lua_State"]
        pub fn add_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9get_accelEP9lua_State"]
        pub fn get_accel(arg1: u64) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9get_speedEP9lua_State"]
        pub fn get_speed(arg1: u64) -> root::phx::Vector2f;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9is_enableEP9lua_State"]
        pub fn is_enable(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9mul_accelEP9lua_State"]
        pub fn mul_accel(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9mul_speedEP9lua_State"]
        pub fn mul_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9set_accelEP9lua_State"]
        pub fn set_accel(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9set_angleEP9lua_State"]
        pub fn set_angle(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9set_brakeEP9lua_State"]
        pub fn set_brake(arg1: u64);
    }
    extern "C" {
        ///Sets the current speed
        ///
        /// This is a lua stack based function - it takes in one argument, the lua state, and relies on the current lua stack to
        // figure out what args it is getting.
        ///
        /// # Arguments
        ///
        /// * `lua_state` - the lua state of the current L2CAgent
        ///
        /// # Example
        ///
        /// ```
        /// l2c_agent.clear_lua_stack(); //clear the stack from any previous args
        /// l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64)); //push the first arg, that being a KINETIC_ENERGY_ID const
        /// l2c_agent.push_lua_stack(&mut L2CValue::new_num(5.0)); //push the second arg, that being a float of the new speed we want to set 
        /// sv_kinetic_energy::set_speed(lua_state); //call the desired function with the lua state which will grab the args we previously pushed
        /// ```
        /// An L2CAgent can be obtained in multiple different contexts. One common place you'd have access to one is in `sys_line_system_control_fighter`
        /// That function runs once-per-frame per-fighter. As an arg it takes an L2CFighterCommon, and when hooking it, we can use that L2CFighterCommon to get
        /// an L2CAgent, which we can then use to manipulate the lua stack. To get an L2CAgent you might do something like this:
        /// ```
        /// use smash::lib::{L2CValue, L2CAgent};
        /// use smash::lua2cpp::L2CFighterCommon;
        /// use smash::app::sv_system;
        /// #[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
        /// pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
        ///     let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); //you can also use an L2CFighterCommon to get a module_accessor
        ///     let mut lua_state = fighter.lua_state_agent;
        ///     let mut l2c_agent = L2CAgent::new(lua_state);
        ///
        ///     original!()(fighter)
        /// }
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy9set_speedEP9lua_State"]
        pub fn set_speed(arg1: u64);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app17sv_kinetic_energy19set_needs_set_paramEP9lua_State"]
        pub fn set_needs_set_param(arg1: u64);
    }
}