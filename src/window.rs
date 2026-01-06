use gtk4 as gtk;
use gtk::gio;
use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use rand::prelude::*;

mod imp {
    use super::*;

    const RESPONSES: &[&str] = &[
        "упс",
        "вжух!",
        "магия",
        "а вот и не угадал",
        "что-то пошло не так",
        "почти получилось",
        "могло быть и хуже",
        "не в этот раз",
        "попробуй еще раз",
        "бывает",
        "и так сойдет",
        "ну, что ж поделать",
        "такова жизнь",
        "это фиаско, братан",
        "все сломалось",
        "нельзя",
        "запрещено",
        "даже не думай",
        "руки прочь!",
        "я бы не стал",
        "осторожно, злая кнопка",
        "ну ок",
        "ладно",
        "допустим",
        "и что?",
        "скучно",
        "норм",
        "потом",
        "завтра",
        "никогда",
        "может быть",
        "возможно...",
        "загрузка...",
    ];

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
    }

    impl Default for TweakerWindow {
        fn default() -> Self {
            Self {
                picture: TemplateChild::default(),
                rtx_button: TemplateChild::default(),
                pacman_button: TemplateChild::default(),
                yasha_button: TemplateChild::default(),
                max_button: TemplateChild::default(),
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
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl TweakerWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
