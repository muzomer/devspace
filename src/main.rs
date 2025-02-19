mod cli;
mod devspace;
mod ui;

fn main() {
    let mut terminal = ratatui::init();
    let app_result = ui::App::default().run(&mut terminal);
    ratatui::restore();
    print!("{}", app_result.unwrap());
}
