use crate::app::AppState;
use crate::telemetry::PidAxis;
use bevy_egui::egui;
use egui::Color32;
use egui_plot::{Legend, Line, Plot};

/// Renders the attitude plot (Roll, Pitch, Yaw)
pub fn render_attitude_plot(ui: &mut egui::Ui, state: &AppState) {
    let max_width = ui.ctx().screen_rect().width() - 32.0;
    ui.set_max_width(max_width);
    ui.group(|ui| {
        ui.set_max_width(max_width - 16.0);
        ui.label("Attitude (Roll, Pitch, Yaw)");
        let buffer = state.data_buffer.lock().unwrap();
        let plot_height = (ui.ctx().screen_rect().height() * 0.25).min(300.0);
        let plot_width = ui.available_width();

        Plot::new("attitude_plot")
            .legend(Legend::default())
            .height(plot_height)
            .width(plot_width)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(buffer.get_roll_data())
                        .name("Roll")
                        .color(Color32::from_rgb(255, 0, 0)),
                );
                plot_ui.line(
                    Line::new(buffer.get_pitch_data())
                        .name("Pitch")
                        .color(Color32::from_rgb(0, 255, 0)),
                );
                plot_ui.line(
                    Line::new(buffer.get_yaw_data())
                        .name("Yaw")
                        .color(Color32::from_rgb(0, 0, 255)),
                );
            });
    });
}

/// Renders the PID plot for the selected axis
pub fn render_pid_plot(ui: &mut egui::Ui, state: &mut AppState) {
    let max_width = ui.ctx().screen_rect().width() - 32.0;
    ui.set_max_width(max_width);
    ui.group(|ui| {
        ui.set_max_width(max_width - 16.0);
        ui.horizontal(|ui| {
            ui.label("PID Axis:");
            ui.selectable_value(&mut state.selected_pid_axis, PidAxis::Roll, "Roll");
            ui.selectable_value(&mut state.selected_pid_axis, PidAxis::Pitch, "Pitch");
            ui.selectable_value(&mut state.selected_pid_axis, PidAxis::Yaw, "Yaw");
        });

        let selected_axis = state.selected_pid_axis;
        let axis_name = match selected_axis {
            PidAxis::Roll => "Roll",
            PidAxis::Pitch => "Pitch",
            PidAxis::Yaw => "Yaw",
        };

        ui.label(format!("{axis_name} PID Values (P, I, D)"));

        let buffer = state.data_buffer.lock().unwrap();
        let plot_height = (ui.ctx().screen_rect().height() * 0.20).min(200.0);
        let plot_width = ui.available_width();

        Plot::new("pid_plot")
            .legend(Legend::default())
            .height(plot_height)
            .width(plot_width)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(buffer.get_pid_p_data(selected_axis))
                        .name("P")
                        .color(Color32::from_rgb(255, 100, 100)),
                );
                plot_ui.line(
                    Line::new(buffer.get_pid_i_data(selected_axis))
                        .name("I")
                        .color(Color32::from_rgb(100, 255, 100)),
                );
                plot_ui.line(
                    Line::new(buffer.get_pid_d_data(selected_axis))
                        .name("D")
                        .color(Color32::from_rgb(100, 100, 255)),
                );
            });
    });
}
