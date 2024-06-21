use std::{sync::Arc, time::Instant};

mod paint;
use egui_demo_lib::DemoWindows;
use paint::*;

use meterm_server::{
    egui::{mutex::Mutex, CentralPanel, Context, Id, ScrollArea, TextEdit, TopBottomPanel, Ui},
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
    paint: PaintServerData,
    text: String,
    demo: DemoWindows,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Tab {
    #[default]
    Paint,
    Text,
    Demo,
    DemoShared,
}

#[derive(Default)]
struct ClientData {
    tab: Tab,
    paint: PaintClientData,
    demo: DemoWindows,
}

// I'm sure this is fine :)
unsafe impl Send for ClientData {}
unsafe impl Sync for ClientData {}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, ctx: &Context) {
        // Get client data
        let client_stuff = ctx.data_mut(|mem| {
            mem.get_temp_mut_or_default::<Arc<Mutex<ClientData>>>(Id::new("client_stuff"))
                .clone()
        });
        let mut client_stuff = client_stuff.lock();

        TopBottomPanel::top("top_panel").show(ctx, |ui|{
            ui.horizontal(|ui| {
                ui.selectable_value(&mut client_stuff.tab, Tab::Paint, "Paint");
                ui.selectable_value(&mut client_stuff.tab, Tab::Text, "Text");
                ui.selectable_value(&mut client_stuff.tab, Tab::Demo, "Egui Demo");
                ui.selectable_value(&mut client_stuff.tab, Tab::DemoShared, "Egui Demo (shared by everyone)");
            });
        });

        match client_stuff.tab {
            Tab::Paint => paint(ctx, &mut client_stuff.paint, &mut self.paint),
            Tab::Text => other(ctx, &mut self.text),
            Tab::Demo => client_stuff.demo.ui(ctx),
            Tab::DemoShared => self.demo.ui(ctx),
        }
    }
}

pub fn other(ctx: &Context, text: &mut String) {
    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add(TextEdit::multiline(text).min_size(ui.available_size()));
        });
    });
}

