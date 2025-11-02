use bevy::app::App;
use bevy::DefaultPlugins;

pub struct UI {
    app: App
}

impl UI {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins);

        Self {
            app
        }
    }

    pub fn run(&mut self) {
        self.app.run();
    }
}