mod app;
mod navigation;
mod secondpage;
mod thirdpage;

use crate::app::App;
use ratatui::crossterm::{event, event::Event, event::KeyEvent};
use std::time::Duration;
use std::{io, io::Error, sync::mpsc, sync::Arc, sync::Mutex, thread};

fn main() {
    let (event_sd, event_rc) = mpsc::channel::<KeyEvent>();
    let mutex_tread_app = Arc::new(Mutex::new(App::default()));

    let handle_tread = thread::Builder::new()
        .name("handle".into())
        .spawn(move || {
            main_handle_events(event_sd)
        } )
        .unwrap();

    let draw_thread = thread::Builder::new()
        .name("draw".into())
        .spawn({
        let draw_app = Arc::clone(&mutex_tread_app);
        move || {
            draw_app.lock().unwrap().run(ratatui::init(), event_rc)
        }
    })
        .unwrap();

    match handle_tread.join().unwrap() {
        Ok(_) => {},
        Err(error) => { event_error_handling(error) }
    }
    match draw_thread.join().unwrap() {
        Ok(_) => {},
        Err(error) => { draw_error_handling(error) }
    }

    ratatui::restore();
}
fn main_handle_events(sd: mpsc::Sender<KeyEvent>) -> io::Result<()> {
    loop {
        if event::poll(Duration::from_nanos(1000000000))? {
            match event::read()? {
                Event::Key(key_env) => { sd.send(key_env).unwrap() },
                _ => {}
            }
        }
    }
}
fn draw_error_handling(err: Error) {
    println!("Ошибка в потоке отрисовки: {}", err);
}
fn event_error_handling(err: Error) {
    println!("Ошибка в потоке считывания клавиш {}", err);
}
