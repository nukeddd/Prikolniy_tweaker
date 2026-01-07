use gtk4 as gtk;
use gtk::gio;
use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use rand::prelude::*;
use sysinfo::{System, SystemExt, CpuExt};

mod imp {
    use gtk4::glib::random_int_range;
use super::*;

    const RESPONSES: &[&str] = &["бывает", "нельзя", "ну ок", "упс", "вжух!", "магия", "а вот и не угадал", "что-то пошло не так", "почти получилось", "могло быть и хуже", "не в этот раз", "попробуй еще раз", "и так сойдет", "ну что ж поделать", "такова жизнь", "это фиаско, братан", "все сломалось", "запрещено", "даже не думай", "руки прочь!", "я бы не стал", "осторожно, злая кнопка", "ладно", "допустим", "и что?", "скучно", "норм", "потом", "завтра", "никогда", "может быть", "возможно...", "загрузка..."];

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/jeremy-compost/tweaker/window.ui")]
    pub struct TweakerWindow {
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub rtx_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub pacman_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub yasha_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub max_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub maku_tweaker_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub useless_buttons_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub performance_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub balance_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub save_mode_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub sys_info_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub single_test_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub multi_test_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub view_results_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub benchmark_result_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub ms_telemetry_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub bubunta_telemetry_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub rkn_telemetry_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub fortuna_telemetry_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub show_hidden_files_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub show_extensions_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub open_this_pc_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub hide_drive_letters_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub select_drive_letters_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub show_all_drive_letters_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub fix_duplicate_drives_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub pause_update_service_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_update_cache_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub block_version_combo: TemplateChild<gtk::ComboBoxText>,
        #[template_child]
        pub block_version_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub disable_updates_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_driver_updates_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_reserved_storage_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub sfc_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub dism_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub temp_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub report_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub disable_sticky_keys_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_core_isolation_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_uac_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_smartscreen_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_hibernation_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_sleep_timeout_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_bing_search_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_vbs_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_swap_file_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub enable_old_bootloader_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub additional_boot_options_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub set_chkdsk_timeout_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub apply_compactos_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_bitlocker_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub uwp_status_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub uwp_view_combo: TemplateChild<gtk::ComboBoxText>,
        #[template_child]
        pub uwp_remove_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub uwp_progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub new_name_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub apply_new_name_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub standard_new_name_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub color_combo: TemplateChild<gtk::ComboBoxText>,
        #[template_child]
        pub apply_color_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub enable_end_task_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub small_window_buttons_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_lock_screen_blur_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_transparency_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub enable_dark_theme_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub verbose_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_boot_logo_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub disable_boot_animation_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub enable_old_context_menu_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub remove_context_menu_delay_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub scan_with_defender_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub open_in_terminal_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub add_to_favorites_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub share_context_menu_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub restore_previous_versions_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub send_to_context_menu_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub copy_as_path_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub pin_to_start_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub pin_to_taskbar_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub open_in_new_tab_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub nvidia_control_panel_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub install_directplay_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub install_dotnet_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub enable_photo_viewer_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub lgp_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub enable_lgp_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub enable_ps_scripts_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub disable_dvr_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub disable_hyperv_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub reset_winsxs_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub hwid_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub office_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub kms_server_combo: TemplateChild<gtk::ComboBoxText>,
        #[template_child]
        pub apply_kms_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub apply_activation_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub activation_progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub activation_status_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub app_install_grid: TemplateChild<gtk::Grid>,
        #[template_child]
        pub app_install_status_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub app_install_progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub install_winget_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub skip_app_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub start_install_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub stop_install_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub quick_show_hidden_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_show_extensions_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_open_this_pc_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_pause_updates_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_show_this_pc_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_remove_shortcut_suffix_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_hide_task_view_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_remove_ads_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_disable_bing_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_fix_duplicates_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_disable_bitlocker_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_disable_sticky_keys_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_enable_clipboard_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_remove_context_delay_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_set_chkdsk_timeout_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_install_directplay_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub quick_install_dotnet_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub start_quick_setup_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub minutes_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub hours_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub start_shutdown_timer_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub cancel_shutdown_timer_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub ten_minutes_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub thirty_minutes_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub one_hour_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub two_hours_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub four_hours_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub six_hours_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub shutdown_time_scale: TemplateChild<gtk::Scale>,
    }

    impl Default for TweakerWindow {
        fn default() -> Self {
            Self {
                picture: TemplateChild::default(),
                rtx_button: TemplateChild::default(),
                pacman_button: TemplateChild::default(),
                yasha_button: TemplateChild::default(),
                max_button: TemplateChild::default(),
                maku_tweaker_button: TemplateChild::default(),
                useless_buttons_box: TemplateChild::default(),
                performance_button: TemplateChild::default(),
                balance_button: TemplateChild::default(),
                save_mode_button: TemplateChild::default(),
                sys_info_label: TemplateChild::default(),
                single_test_button: TemplateChild::default(),
                multi_test_button: TemplateChild::default(),
                view_results_button: TemplateChild::default(),
                benchmark_result_label: TemplateChild::default(),
                ms_telemetry_switch: TemplateChild::default(),
                bubunta_telemetry_switch: TemplateChild::default(),
                rkn_telemetry_switch: TemplateChild::default(),
                fortuna_telemetry_switch: TemplateChild::default(),
                show_hidden_files_switch: TemplateChild::default(),
                show_extensions_switch: TemplateChild::default(),
                open_this_pc_switch: TemplateChild::default(),
                hide_drive_letters_switch: TemplateChild::default(),
                select_drive_letters_button: TemplateChild::default(),
                show_all_drive_letters_button: TemplateChild::default(),
                fix_duplicate_drives_button: TemplateChild::default(),
                pause_update_service_button: TemplateChild::default(),
                clear_update_cache_button: TemplateChild::default(),
                block_version_combo: TemplateChild::default(),
                block_version_button: TemplateChild::default(),
                disable_updates_switch: TemplateChild::default(),
                disable_driver_updates_switch: TemplateChild::default(),
                disable_reserved_storage_switch: TemplateChild::default(),
                sfc_button: TemplateChild::default(),
                dism_button: TemplateChild::default(),
                temp_button: TemplateChild::default(),
                report_button: TemplateChild::default(),
                disable_sticky_keys_switch: TemplateChild::default(),
                disable_core_isolation_switch: TemplateChild::default(),
                disable_uac_switch: TemplateChild::default(),
                disable_smartscreen_switch: TemplateChild::default(),
                disable_hibernation_switch: TemplateChild::default(),
                disable_sleep_timeout_switch: TemplateChild::default(),
                disable_bing_search_switch: TemplateChild::default(),
                disable_vbs_switch: TemplateChild::default(),
                disable_swap_file_switch: TemplateChild::default(),
                enable_old_bootloader_switch: TemplateChild::default(),
                additional_boot_options_switch: TemplateChild::default(),
                set_chkdsk_timeout_switch: TemplateChild::default(),
                apply_compactos_switch: TemplateChild::default(),
                disable_bitlocker_switch: TemplateChild::default(),
                uwp_status_label: TemplateChild::default(),
                uwp_view_combo: TemplateChild::default(),
                uwp_remove_button: TemplateChild::default(),
                uwp_progress_bar: TemplateChild::default(),
                new_name_entry: TemplateChild::default(),
                apply_new_name_button: TemplateChild::default(),
                standard_new_name_button: TemplateChild::default(),
                color_combo: TemplateChild::default(),
                apply_color_button: TemplateChild::default(),
                enable_end_task_switch: TemplateChild::default(),
                small_window_buttons_switch: TemplateChild::default(),
                disable_lock_screen_blur_switch: TemplateChild::default(),
                disable_transparency_switch: TemplateChild::default(),
                enable_dark_theme_switch: TemplateChild::default(),
                verbose_switch: TemplateChild::default(),
                disable_boot_logo_switch: TemplateChild::default(),
                disable_boot_animation_switch: TemplateChild::default(),
                enable_old_context_menu_switch: TemplateChild::default(),
                remove_context_menu_delay_switch: TemplateChild::default(),
                scan_with_defender_switch: TemplateChild::default(),
                open_in_terminal_switch: TemplateChild::default(),
                add_to_favorites_switch: TemplateChild::default(),
                share_context_menu_switch: TemplateChild::default(),
                restore_previous_versions_switch: TemplateChild::default(),
                send_to_context_menu_switch: TemplateChild::default(),
                copy_as_path_switch: TemplateChild::default(),
                pin_to_start_switch: TemplateChild::default(),
                pin_to_taskbar_switch: TemplateChild::default(),
                open_in_new_tab_switch: TemplateChild::default(),
                nvidia_control_panel_switch: TemplateChild::default(),
                install_directplay_button: TemplateChild::default(),
                install_dotnet_button: TemplateChild::default(),
                enable_photo_viewer_button: TemplateChild::default(),
                lgp_box: TemplateChild::default(),
                enable_lgp_button: TemplateChild::default(),
                enable_ps_scripts_button: TemplateChild::default(),
                disable_dvr_button: TemplateChild::default(),
                disable_hyperv_button: TemplateChild::default(),
                reset_winsxs_button: TemplateChild::default(),
                hwid_button: TemplateChild::default(),
                office_button: TemplateChild::default(),
                kms_server_combo: TemplateChild::default(),
                apply_kms_button: TemplateChild::default(),
                apply_activation_button: TemplateChild::default(),
                activation_progress_bar: TemplateChild::default(),
                activation_status_label: TemplateChild::default(),
                app_install_grid: TemplateChild::default(),
                app_install_status_label: TemplateChild::default(),
                app_install_progress_bar: TemplateChild::default(),
                install_winget_button: TemplateChild::default(),
                skip_app_button: TemplateChild::default(),
                start_install_button: TemplateChild::default(),
                stop_install_button: TemplateChild::default(),
                quick_show_hidden_switch: TemplateChild::default(),
                quick_show_extensions_switch: TemplateChild::default(),
                quick_open_this_pc_switch: TemplateChild::default(),
                quick_pause_updates_switch: TemplateChild::default(),
                quick_show_this_pc_switch: TemplateChild::default(),
                quick_remove_shortcut_suffix_switch: TemplateChild::default(),
                quick_hide_task_view_switch: TemplateChild::default(),
                quick_remove_ads_switch: TemplateChild::default(),
                quick_disable_bing_switch: TemplateChild::default(),
                quick_fix_duplicates_switch: TemplateChild::default(),
                quick_disable_bitlocker_switch: TemplateChild::default(),
                quick_disable_sticky_keys_switch: TemplateChild::default(),
                quick_enable_clipboard_switch: TemplateChild::default(),
                quick_remove_context_delay_switch: TemplateChild::default(),
                quick_set_chkdsk_timeout_switch: TemplateChild::default(),
                quick_install_directplay_switch: TemplateChild::default(),
                quick_install_dotnet_switch: TemplateChild::default(),
                start_quick_setup_button: TemplateChild::default(),
                minutes_entry: TemplateChild::default(),
                hours_label: TemplateChild::default(),
                start_shutdown_timer_button: TemplateChild::default(),
                cancel_shutdown_timer_button: TemplateChild::default(),
                ten_minutes_button: TemplateChild::default(),
                thirty_minutes_button: TemplateChild::default(),
                one_hour_button: TemplateChild::default(),
                two_hours_button: TemplateChild::default(),
                four_hours_button: TemplateChild::default(),
                six_hours_button: TemplateChild::default(),
                shutdown_time_scale: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TweakerWindow {
        const NAME: &'static str = "TweakerWindow";
        type Type = super::TweakerWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TweakerWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.picture
                .set_resource(Some("/com/github/jeremy-compost/tweaker/reclama.jpg"));

            let window = self.obj();

            let rtx_window_weak = window.downgrade();
            self.rtx_button.connect_clicked(move |_| {
                if let Some(window) = rtx_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let pacman_window_weak = window.downgrade();
            self.pacman_button.connect_clicked(move |_| {
                if let Some(window) = pacman_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let yasha_window_weak = window.downgrade();
            self.yasha_button.connect_clicked(move |_| {
                if let Some(window) = yasha_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let max_window_weak = window.downgrade();
            self.max_button.connect_clicked(move |_| {
                if let Some(window) = max_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let maku_tweaker_window_weak = window.downgrade();
            self.maku_tweaker_button.connect_clicked(move |_| {
                if let Some(window) = maku_tweaker_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let performance_window_weak = window.downgrade();
            self.performance_button.connect_clicked(move |_| {
                if let Some(window) = performance_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let balance_window_weak = window.downgrade();
            self.balance_button.connect_clicked(move |_| {
                if let Some(window) = balance_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let save_mode_window_weak = window.downgrade();
            self.save_mode_button.connect_clicked(move |_| {
                if let Some(window) = save_mode_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let single_test_window_weak = window.downgrade();
            self.single_test_button.connect_clicked(move |_| {
                if let Some(window) = single_test_window_weak.upgrade() {
                    let score = random_int_range(300, 1500);
                    window.imp().benchmark_result_label.set_text(&format!("Your score: {}", score));
                }
            });

            let multi_test_window_weak = window.downgrade();
            self.multi_test_button.connect_clicked(move |_| {
                if let Some(window) = multi_test_window_weak.upgrade() {
                    let score = random_int_range(1000, 30000);
                    window.imp().benchmark_result_label.set_text(&format!("Your score: {}", score));
                }
            });

            let view_results_window_weak = window.downgrade();
            self.view_results_button.connect_clicked(move |_| {
                if let Some(window) = view_results_window_weak.upgrade() {
                    gtk::show_uri(Some(&window), "https://adderly.top/makubench", 0);
                }
            });

            let ms_telemetry_window_weak = window.downgrade();
            self.ms_telemetry_switch.connect_state_set(move |switch, state| {
                if !state {
                    if let Some(window) = ms_telemetry_window_weak.upgrade() {
                        show_dialog(&window);
                        switch.set_active(true);
                    }
                }
                glib::Propagation::Stop
            });

            let bubunta_telemetry_window_weak = window.downgrade();
            self.bubunta_telemetry_switch.connect_state_set(move |switch, state| {
                if !state {
                    if let Some(window) = bubunta_telemetry_window_weak.upgrade() {
                        show_dialog(&window);
                        switch.set_active(true);
                    }
                }
                glib::Propagation::Stop
            });

            let rkn_telemetry_window_weak = window.downgrade();
            self.rkn_telemetry_switch.connect_state_set(move |switch, state| {
                if !state {
                    if let Some(window) = rkn_telemetry_window_weak.upgrade() {
                        show_dialog(&window);
                        switch.set_active(true);
                    }
                }
                glib::Propagation::Stop
            });

            let fortuna_telemetry_window_weak = window.downgrade();
            self.fortuna_telemetry_switch.connect_state_set(move |switch, state| {
                if !state {
                    if let Some(window) = fortuna_telemetry_window_weak.upgrade() {
                        show_dialog(&window);
                        switch.set_active(true);
                    }
                }
                glib::Propagation::Stop
            });

            let apply_new_name_window_weak = window.downgrade();
            self.apply_new_name_button.connect_clicked(move |_| {
                if let Some(window) = apply_new_name_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let standard_new_name_window_weak = window.downgrade();
            self.standard_new_name_button.connect_clicked(move |_| {
                if let Some(window) = standard_new_name_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let apply_color_window_weak = window.downgrade();
            self.apply_color_button.connect_clicked(move |_| {
                if let Some(window) = apply_color_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let enable_end_task_window_weak = window.downgrade();
            self.enable_end_task_switch.connect_state_set(move |_, _| {
                if let Some(window) = enable_end_task_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let small_window_buttons_window_weak = window.downgrade();
            self.small_window_buttons_switch.connect_state_set(move |_, _| {
                if let Some(window) = small_window_buttons_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let disable_lock_screen_blur_window_weak = window.downgrade();
            self.disable_lock_screen_blur_switch.connect_state_set(move |_, _| {
                if let Some(window) = disable_lock_screen_blur_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let disable_transparency_window_weak = window.downgrade();
            self.disable_transparency_switch.connect_state_set(move |_, _| {
                if let Some(window) = disable_transparency_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let enable_dark_theme_window_weak = window.downgrade();
            self.enable_dark_theme_switch.connect_state_set(move |_, _| {
                if let Some(window) = enable_dark_theme_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let verbose_window_weak = window.downgrade();
            self.verbose_switch.connect_state_set(move |_, _| {
                if let Some(window) = verbose_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let disable_boot_logo_window_weak = window.downgrade();
            self.disable_boot_logo_switch.connect_state_set(move |_, _| {
                if let Some(window) = disable_boot_logo_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let disable_boot_animation_window_weak = window.downgrade();
            self.disable_boot_animation_switch.connect_state_set(move |_, _| {
                if let Some(window) = disable_boot_animation_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let enable_old_context_menu_window_weak = window.downgrade();
            self.enable_old_context_menu_switch.connect_state_set(move |_, _| {
                if let Some(window) = enable_old_context_menu_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let remove_context_menu_delay_window_weak = window.downgrade();
            self.remove_context_menu_delay_switch.connect_state_set(move |_, _| {
                if let Some(window) = remove_context_menu_delay_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let scan_with_defender_window_weak = window.downgrade();
            self.scan_with_defender_switch.connect_state_set(move |_, _| {
                if let Some(window) = scan_with_defender_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let open_in_terminal_window_weak = window.downgrade();
            self.open_in_terminal_switch.connect_state_set(move |_, _| {
                if let Some(window) = open_in_terminal_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let add_to_favorites_window_weak = window.downgrade();
            self.add_to_favorites_switch.connect_state_set(move |_, _| {
                if let Some(window) = add_to_favorites_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let share_context_menu_window_weak = window.downgrade();
            self.share_context_menu_switch.connect_state_set(move |_, _| {
                if let Some(window) = share_context_menu_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let restore_previous_versions_window_weak = window.downgrade();
            self.restore_previous_versions_switch.connect_state_set(move |_, _| {
                if let Some(window) = restore_previous_versions_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let send_to_context_menu_window_weak = window.downgrade();
            self.send_to_context_menu_switch.connect_state_set(move |_, _| {
                if let Some(window) = send_to_context_menu_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let copy_as_path_window_weak = window.downgrade();
            self.copy_as_path_switch.connect_state_set(move |_, _| {
                if let Some(window) = copy_as_path_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let pin_to_start_window_weak = window.downgrade();
            self.pin_to_start_switch.connect_state_set(move |_, _| {
                if let Some(window) = pin_to_start_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let pin_to_taskbar_window_weak = window.downgrade();
            self.pin_to_taskbar_switch.connect_state_set(move |_, _| {
                if let Some(window) = pin_to_taskbar_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let open_in_new_tab_window_weak = window.downgrade();
            self.open_in_new_tab_switch.connect_state_set(move |_, _| {
                if let Some(window) = open_in_new_tab_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let nvidia_control_panel_window_weak = window.downgrade();
            self.nvidia_control_panel_switch.connect_state_set(move |_, _| {
                if let Some(window) = nvidia_control_panel_window_weak.upgrade() {
                    show_dialog(&window);
                }
                glib::Propagation::Stop
            });

            let install_directplay_window_weak = window.downgrade();
            self.install_directplay_button.connect_clicked(move |_| {
                if let Some(window) = install_directplay_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let install_dotnet_window_weak = window.downgrade();
            self.install_dotnet_button.connect_clicked(move |_| {
                if let Some(window) = install_dotnet_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let enable_photo_viewer_window_weak = window.downgrade();
            self.enable_photo_viewer_button.connect_clicked(move |_| {
                if let Some(window) = enable_photo_viewer_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let enable_lgp_window_weak = window.downgrade();
            self.enable_lgp_button.connect_clicked(move |_| {
                if let Some(window) = enable_lgp_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let enable_ps_scripts_window_weak = window.downgrade();
            self.enable_ps_scripts_button.connect_clicked(move |_| {
                if let Some(window) = enable_ps_scripts_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let disable_dvr_window_weak = window.downgrade();
            self.disable_dvr_button.connect_clicked(move |_| {
                if let Some(window) = disable_dvr_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let disable_hyperv_window_weak = window.downgrade();
            self.disable_hyperv_button.connect_clicked(move |_| {
                if let Some(window) = disable_hyperv_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let reset_winsxs_window_weak = window.downgrade();
            self.reset_winsxs_button.connect_clicked(move |_| {
                if let Some(window) = reset_winsxs_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let hwid_window_weak = window.downgrade();
            self.hwid_button.connect_clicked(move |_| {
                if let Some(window) = hwid_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let office_window_weak = window.downgrade();
            self.office_button.connect_clicked(move |_| {
                if let Some(window) = office_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let apply_kms_window_weak = window.downgrade();
            self.apply_kms_button.connect_clicked(move |_| {
                if let Some(window) = apply_kms_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let apply_activation_window_weak = window.downgrade();
            self.apply_activation_button.connect_clicked(move |_| {
                if let Some(window) = apply_activation_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let install_winget_window_weak = window.downgrade();
            self.install_winget_button.connect_clicked(move |_| {
                if let Some(window) = install_winget_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let skip_app_window_weak = window.downgrade();
            self.skip_app_button.connect_clicked(move |_| {
                if let Some(window) = skip_app_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let start_install_window_weak = window.downgrade();
            self.start_install_button.connect_clicked(move |_| {
                if let Some(window) = start_install_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            let stop_install_window_weak = window.downgrade();
            self.stop_install_button.connect_clicked(move |_| {
                if let Some(window) = stop_install_window_weak.upgrade() {
                    show_dialog(&window);
                }
            });

            for i in 1..=69 {
                let button = gtk::Button::with_label(&format!("Кнопка что то делает {}", i));
                button.set_margin_top(6);
                button.set_margin_start(12);
                button.set_margin_end(12);
                if i == 69 {
                    button.set_margin_bottom(12);
                }

                let window_weak = window.downgrade();
                button.connect_clicked(move |_| {
                    if let Some(window) = window_weak.upgrade() {
                        show_dialog(&window);
                    }
                });
                self.useless_buttons_box.append(&button);
            }

            let mut sys = System::new_all();
            sys.refresh_all();

            let cpu_info = format!("CPU: {} ({} cores)", sys.global_cpu_info().brand(), sys.cpus().len());



            let final_gpu_info =
                "GPU: Not found".to_string();


            let info = format!(
                "System name: {}\nKernel version: {}\nOS version: {}\n\n{}\n\n{}\n\nTotal memory: {} GB\nUsed memory: {} MB",
                sys.name().unwrap_or_default(),
                sys.kernel_version().unwrap_or_default(),
                sys.os_version().unwrap_or_default(),
                cpu_info,
                final_gpu_info,
                sys.total_memory() / (1024 * 1024),
                sys.used_memory() / 1024
            );

            self.sys_info_label.set_text(&info);
        }
    }

    fn show_dialog(window: &super::TweakerWindow) {
        let mut rng  = random_int_range(0, RESPONSES.len() as i32);


        let response = RESPONSES[rng as usize];

        let dialog = gtk::MessageDialog::new(
            Some(window),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            response,
        );
        dialog.connect_response(|dialog, _| {
            dialog.close();
        });
        dialog.present();
    }

    impl WidgetImpl for TweakerWindow {}
    impl WindowImpl for TweakerWindow {}
    impl ApplicationWindowImpl for TweakerWindow {}
}

glib::wrapper! {
    pub struct TweakerWindow(ObjectSubclass<imp::TweakerWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl TweakerWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
