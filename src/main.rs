mod app;

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
    let app_result = App::default().run(ratatui::init());
    ratatui::restore();
    app_result
}
