use std::{sync::Arc, time::Instant};

use metacontrols_server::{
    egui::{mutex::Mutex, CentralPanel, Context, Id, Ui},
    Server,
};

fn main() {
    let mut server = Server::new("0.0.0.0:5000");
    let mut app = App::new();

    let desired_tickrate = 90.0;

    loop {
        let tick_start = Instant::now();

        server.show_on_clients(|ctx| {
            app.run(ctx);
        });

        let tick_time = tick_start.elapsed();
        let remaining_time = (1. / desired_tickrate - tick_time.as_secs_f32()).max(0.0);
        std::thread::sleep(std::time::Duration::from_secs_f32(remaining_time));
    }
}

#[derive(Default)]
struct App {
    paint: PaintSeverData,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Tab {
    #[default]
    Paint,
    OtherTest,
}

#[derive(Default, Clone)]
struct ClientData {
    tab: Tab,
    paint: PaintClientData,
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Get client data
            let client_stuff = ctx.data_mut(|mem| {
                mem.get_temp_mut_or_default::<Arc<Mutex<ClientData>>>(Id::new("client_stuff"))
                    .clone()
            });
            let mut client_stuff = client_stuff.lock();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut client_stuff.tab, Tab::Paint, "Paint");
                ui.selectable_value(&mut client_stuff.tab, Tab::OtherTest, "Test");
            });

            match client_stuff.tab {
                Tab::Paint => paint(ui, &mut client_stuff.paint, &mut self.paint),
                Tab::OtherTest => other(ui),
            }
        });
    }
}

#[derive(Default, Clone)]
struct PaintClientData {
}

#[derive(Default)]
struct PaintSeverData {
}

fn paint(ui: &mut Ui, client: &mut PaintClientData, server: &mut PaintSeverData) {
    ui.label("Paint stuff");
}

fn other(ui: &mut Ui) {
    ui.label("Other stuff");
}
