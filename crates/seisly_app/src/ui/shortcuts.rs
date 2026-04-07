use crate::app::SeislyApp;
use eframe::egui;

pub fn handle_shortcuts(ctx: &egui::Context, app: &mut SeislyApp) {
    ctx.input_mut(|i| {
        // History
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::Z,
        )) {
            app.history.undo(&mut app.interpretation);
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::Y,
        )) {
            app.history.redo(&mut app.interpretation);
        }

        // Project
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::S,
        )) {
            // Save project logic
        }

        // View
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::B,
        )) {
            app.show_sidebar = !app.show_sidebar;
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::J,
        )) {
            app.show_bottom_panel = !app.show_bottom_panel;
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::NONE,
            egui::Key::D,
        )) {
            app.velocity.is_depth_mode = !app.velocity.is_depth_mode;
        }
    });
}
