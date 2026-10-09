use egui::{Key, Modifiers};
use serde_json::json;
use soundcraft_ui_egui::{Services, SoundApp, shortcuts};

fn press(ctx: &egui::Context, app: &mut SoundApp, key: Key, modifiers: Modifiers) {
    let events = [true, false].map(|pressed| egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers }).to_vec();
    ctx.run_ui(egui::RawInput { events, ..Default::default() }, |ui| shortcuts::handle(app, ui.ctx())).textures_delta.clear();
}

#[test]
fn ctrl_shift_arrows_nudge_the_selected_clip_gain() {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    let kick = app.engine.session().track_by_name("Kick").unwrap().clips()[0].id;
    app.engine.execute("edit.select", &json!({"clips": [kick.0]})).unwrap();
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Mac);
    let ctrl_shift = Modifiers { ctrl: true, shift: true, ..Modifiers::NONE };
    press(&ctx, &mut app, Key::ArrowUp, ctrl_shift);
    press(&ctx, &mut app, Key::ArrowUp, ctrl_shift);
    press(&ctx, &mut app, Key::ArrowDown, ctrl_shift);
    assert_eq!(app.engine.session().find_clip(kick).unwrap().1.gain_db, 1.0);
}
