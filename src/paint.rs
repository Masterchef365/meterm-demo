use egui_extras::Column;
use metacontrols_server::egui::{
    ahash::HashMap, CentralPanel, Color32, Context, DragValue, Frame, Pos2, Rect, Rounding, Sense,
    Shape, SidePanel, Stroke, Ui, Widget,
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
    completed: Vec<Drawing>,
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

    let mut hover_rect = None;

    SidePanel::left("Config").show(ctx, |ui| {
        ui.label("Pen");
        for pen in &mut client.palette {
            ui.horizontal(|ui| {
                ui.label("Color: ");
                ui.color_edit_button_srgba(&mut pen.stroke.color);
            });
            ui.add(edit_line_width(&mut pen.stroke.width).prefix("Line width: "));
        }
        ui.separator();

        ui.label("Scribbles");
        let table = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .sense(Sense::click())
            .column(Column::auto())
            .column(Column::remainder());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Index");
                });
                header.col(|ui| {
                    ui.strong("Controls");
                });
            })
            .body(|mut body| {
                let mut do_delete = None;
                for (idx, shape) in server.completed.iter_mut().enumerate() {
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", idx));
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                // Delete button
                                let resp = ui.button("Delete");
                                if resp.clicked() {
                                    do_delete = Some(idx);
                                }
                                if resp.hovered() {
                                    let mut rect = Rect::NOTHING;
                                    for pos in &shape.points {
                                        rect.max = rect.max.max(*pos);
                                        rect.min = rect.min.min(*pos);
                                    }

                                    hover_rect = Some((rect, shape.pen.stroke));
                                }

                                // Edit color and stroke
                                ui.color_edit_button_srgba(&mut shape.pen.stroke.color);
                                ui.add(edit_line_width(&mut shape.pen.stroke.width));
                            });
                        });
                    });
                }
                if let Some(idx) = do_delete {
                    server.completed.remove(idx);
                }
            });
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
                server.completed.push(drawing);
            }

            // Display
            ui.set_clip_rect(resp.rect);
            for shape in &server.completed {
                ui.painter().add(shape.clone());
            }

            for prog in &server.in_progress {
                ui.painter().add(prog.clone());
            }

            // Extra graphics
            if let Some((rect, stroke)) = hover_rect {
                ui.painter().rect_stroke(rect, Rounding::ZERO, stroke);
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

fn edit_line_width(width: &mut f32) -> DragValue {
    let speed = *width * 1e-2;

    DragValue::new(width)
        .clamp_range(0.0..=f32::INFINITY)
        .speed(speed)
        .suffix(" px")
}
