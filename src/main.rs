mod atproto;
use atproto::ATP;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0,240.0)),
        ..Default::default()
    };
    let base = String::from("https://bluesky.benradford.me/");
    // let mut atproto = ATP::new(&base);
    println!("Provider: {}",base);

    let _ = eframe::run_simple_native("Bluesky Unofficial Native",options, move |ctx, _frame| {
        let mut provider = String::new();
        let mut identity = String::new();
        let mut password = String::new();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            ui.horizontal(|ui| {
                
                let provider_label = ui.label("Provider: ");
                ui.text_edit_singleline(&mut provider)
                    .labelled_by(provider_label.id);
            });
            ui.horizontal(|ui| {
                
                let identity_label = ui.label("Identitifier: ");
                ui.text_edit_singleline(&mut identity)
                    .labelled_by(identity_label.id);
            });
            ui.horizontal(|ui| {
                
                let password_label = ui.label("Password: ");
                ui.text_edit_singleline(&mut password)
                    .labelled_by(password_label.id);
            });
            if ui.button("Login").clicked() {
                let mut atproto = ATP::new(&provider);
                match atproto.login(&identity, password) {
                    Ok(_) => {
                        println!("logged in");
                        ui.label(format!("Logged in as {}", identity));
                    },
                    Err(_) => println!("Not Logged in")
                }
            }
        });
    });
}