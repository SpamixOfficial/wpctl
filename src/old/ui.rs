use crate::app::App;
use cursive::{
    view::Resizable,
    views::{Button, Dialog, DummyView, LinearLayout, TextView},
    Cursive, CursiveRunnable, View,
};
use std::path::PathBuf;

// we use a struct for sorting, make it look nice essentially
pub struct AppViews {}

impl AppViews {
    /// Install view
    pub fn setupView() -> Dialog {
        // Create config && create dirs
        fn view_2(s: &mut Cursive) {
            s.pop_layer();
            let new_layer = Dialog::new().content(TextView::new("The following actions will be done:\"\n- Create all directories\n- Create all configurations")).button("Install", |s| install(s)).button("Quit", |s| self::AppViews::are_u_sure(s));
            s.add_layer(new_layer);
        }

        fn install(s: &mut Cursive) {
            match App::install(App::config_dir(), App::approot()) {
                Ok(_) => {
                    s.add_layer(
                        Dialog::new()
                            .title("Install Status")
                            .content(TextView::new("Success!\nRestart the app for the install to take effect"))
                            .button("Finish", |s| s.quit()),
                    );
                }
                Err(e) => {
                    s.add_layer(
                        Dialog::new()
                            .title("Install Status")
                            .content(TextView::new(format!(
                                "Installation Failed:\n{}",
                                e.to_string()
                            )))
                            .button("Quit", |s| s.quit()),
                    );
                }
            }
        }

        // Buttons for intro dialog
        let buttons = LinearLayout::horizontal()
            .child(Button::new("Continue", |s| view_2(s)))
            .child(Button::new("Quit", |s| s.quit()));
        // Intro dialog stuff
        Dialog::around(LinearLayout::vertical().child(TextView::new("Welcome!\n\nIf you do not want to set up the application at the moment simply choose \"Quit\" down below or click \'q\'")).child(buttons)).title("wpctl setup")
    }

    /// App/Main menu view
    pub fn appView() -> TextView {
        TextView::new("I am the appview")
    }
    /// Install wallpaper view
    pub fn installWpView() -> DummyView {
        DummyView::new()
    }

    /// Are you sure you want to exit?
    pub fn are_u_sure(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .content(TextView::new("You sure you want to exit?"))
                .button("Yes", |s| s.quit())
                .button("No", |s| {
                    s.pop_layer();
                }),
        );
    }
}

impl App {
    pub fn ui_init(&mut self) {
        self.add_all_global_callbacks();
        if !self.is_setup {
            self.app.add_layer(AppViews::setupView());
        } else {
            self.app.add_layer(AppViews::appView());
        }

        self.app.run();
    }

    fn add_all_global_callbacks(&mut self) {
        self.app.add_global_callback('q', |s| s.quit());
    }
}
