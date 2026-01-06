use gtk4 as gtk;
use gtk::gio;
use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use rand::prelude::*;
use sysinfo::{System, SystemExt, CpuExt, ComponentExt};

mod imp {
    use super::*;

    const RESPONSES: &[&str] = &["бывает", "нельзя", "ну ок"];

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
    }

    impl Default for TweakerWindow {
        fn default() -> Self {
            Self {
                picture: TemplateChild::default(),
                rtx_button: TemplateChild::default(),
                pacman_button: TemplateChild::default(),
                yasha_button: TemplateChild::default(),
                max_button: TemplateChild::default(),
                useless_buttons_box: TemplateChild::default(),
                performance_button: TemplateChild::default(),
                balance_button: TemplateChild::default(),
                save_mode_button: TemplateChild::default(),
                sys_info_label: TemplateChild::default(),
                single_test_button: TemplateChild::default(),
                multi_test_button: TemplateChild::default(),
                view_results_button: TemplateChild::default(),
                benchmark_result_label: TemplateChild::default(),
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
                    let mut rng = thread_rng();
                    let score = rng.gen_range(300..1500);
                    window.imp().benchmark_result_label.set_text(&format!("Your score: {}", score));
                }
            });

            let multi_test_window_weak = window.downgrade();
            self.multi_test_button.connect_clicked(move |_| {
                if let Some(window) = multi_test_window_weak.upgrade() {
                    let mut rng = thread_rng();
                    let score = rng.gen_range(1000..30000);
                    window.imp().benchmark_result_label.set_text(&format!("Your score: {}", score));
                }
            });

            let view_results_window_weak = window.downgrade();
            self.view_results_button.connect_clicked(move |_| {
                if let Some(window) = view_results_window_weak.upgrade() {
                    gtk::show_uri(Some(&window), "https://adderly.top/makubench", 0);
                }
            });

            for i in 1..=50 {
                let button = gtk::Button::with_label(&format!("Useless Button {}", i));
                button.set_margin_top(6);
                button.set_margin_start(12);
                button.set_margin_end(12);
                if i == 50 {
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

            let gpu_info_str = sys.components().iter()
                .filter(|c| {
                    let label = c.label().to_lowercase();
                    label.contains("gpu") || label.contains("vga") || label.contains("nvidia") || label.contains("amd") || label.contains("radeon") || label.contains("geforce")
                })
                .map(|c| format!("  - {}: {:.1}°C", c.label(), c.temperature()))
                .collect::<Vec<_>>()
                .join("\n");

            let final_gpu_info = if gpu_info_str.is_empty() {
                "GPU: Not found".to_string()
            } else {
                format!("GPU(s):\n{}", gpu_info_str)
            };

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
        let mut rng = thread_rng();
        let response = RESPONSES.choose(&mut rng).unwrap_or(&"");

        let dialog = gtk::MessageDialog::new(
            Some(window),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            *response,
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
