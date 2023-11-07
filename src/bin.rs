use atproto_rs::ATP;
use eframe::egui;

#[derive(Default)]
struct BlueskyClient {
    identity: String,
    provider: String,
    password: String
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0,240.0)),
        ..Default::default()
    };

    let mut provider = String::new();
    let mut identity = String::new();
    let mut password = String::new();


    eframe::run_native("Bluesky Native", options, Box::new(|cc| Box::new(BlueskyClient::new(cc))));
}

impl BlueskyClient {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for BlueskyClient {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            ui.horizontal(|ui| {
                let provider_label = ui.label("Provider: ");
                ui.text_edit_singleline(&mut self.provider)
                    .labelled_by(provider_label.id);
            });

            ui.horizontal(|ui| {
                let identity_label = ui.label("Identitifier: ");
                ui.text_edit_singleline(&mut self.identity)
                    .labelled_by(identity_label.id);
            });
            
            ui.horizontal(|ui| {
                let password_label = ui.label("Password: ");
                ui.text_edit_singleline(&mut self.password)
                    .labelled_by(password_label.id);
            });
            if ui.button("Login").clicked() {
                let mut atproto = ATP::new(&self.provider);
                match atproto.login(&self.identity, self.password.to_string()) {
                    Ok(_) => {
                        println!("logged in");
                        ui.label(format!("Logged in as {}", self.identity));

                        let mut post_text = String::new();
                        let post_text_label = ui.label("Post:");
                        ui.text_edit_multiline(&mut post_text)
                            .labelled_by(post_text_label.id);
                        if ui.button("Post").clicked() {
                            match atproto.post(post_text) {
                                Ok(_) => println!("Post made"),
                                Err(_) => println!("Post could not be made")
                            }
                        }

                    },
                    Err(_) => println!("Not Logged in")
                }
            }
        });
    }
}