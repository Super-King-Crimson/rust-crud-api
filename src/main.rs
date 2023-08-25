mod app;

fn main() {
    if let Err(e) = app::set_database() {
        eprintln!("Error occured: {e}");
        return;
    }

    app::start("0.0.0.0:8080");
}