use std::time::Instant;

use metacontrols_server::{
    egui::{CentralPanel, Context, Id},
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

struct App {
    client_counter: usize,
}

impl App {
    pub fn new() -> Self {
        Self { client_counter: 0 }
    }

    pub fn run(&mut self, ctx: &Context) {
        let user_number = ctx.memory_mut(|mem| {
            *mem.data
                .get_temp_mut_or_insert_with(Id::new("user_number"), || {
                    self.client_counter += 1;
                    self.client_counter
                })
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Hello, client #{}", user_number));
        });
    }
}
