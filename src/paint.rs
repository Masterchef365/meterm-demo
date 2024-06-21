use metacontrols_server::egui::{
    ahash::HashMap, CentralPanel, Color32, Context, DragValue, Frame, Pos2, Sense, Shape,
    SidePanel, Stroke, Ui,
};

#[derive(Clone)]
pub struct PaintClientData {
    id: Option<usize>,
    palette: Vec<Pen>,
    palette_select: usize,
}

#[derive(Default, Clone, Copy)]
struct Pen {
    stroke: Stroke,
}

#[derive(Default)]
pub struct PaintServerData {
    /// All completed drawings
    completed: Vec<Shape>,
    /// Exactly one drawing per client
    in_progress: Vec<Drawing>,
}

#[derive(Default, Clone)]
struct Drawing {
    points: Vec<Pos2>,
    pen: Pen,
}

pub fn paint(ctx: &Context, client: &mut PaintClientData, server: &mut PaintServerData) {
    // Register client
    let client_id = *client.id.get_or_insert_with(|| {
        let ret = server.in_progress.len();
        server.in_progress.push(Drawing::default());
        ret
    });

    SidePanel::left("Config").show(ctx, |ui| {
        for pen in &mut client.palette {
            ui.color_edit_button_srgba(&mut pen.stroke.color);
            ui.add(
                DragValue::new(&mut pen.stroke.width)
                    .speed(1e-2)
                    .clamp_range(0.0..=20.0)
                    .prefix("Line width: ")
                    .suffix(" px"),
            );
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        Frame::canvas(ui.style()).show(ui, |ui| {
            // Handle drawing
            let resp = ui.allocate_response(ui.available_size(), Sense::click_and_drag());

            if resp.dragged() {
                server.in_progress[client_id].pen = client.palette[client.palette_select];

                if let Some(pos) = resp.interact_pointer_pos() {
                    server.in_progress[client_id].points.push(pos);
                }
            }

            if resp.drag_stopped() {
                let drawing = std::mem::take(&mut server.in_progress[client_id]);
                server.completed.push(drawing.into());
            }

            // Display
            for prog in &server.in_progress {
                ui.painter().add(prog.clone());
            }

            for shape in &server.completed {
                ui.painter().add(shape.clone());
            }
        });
    });
}

impl Into<Shape> for Drawing {
    fn into(self) -> Shape {
        Shape::line(self.points, self.pen.stroke)
    }
}

impl Default for PaintClientData {
    fn default() -> Self {
        Self {
            id: None,
            palette: vec![Pen {
                stroke: Stroke::new(1., Color32::WHITE),
            }],
            palette_select: 0,
        }
    }
}
