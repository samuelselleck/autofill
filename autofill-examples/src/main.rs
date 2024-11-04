// OBS this example
mod adveture_game;
// mod time_tracking_cli_app;

fn main() {
    // TODO: this dodesn't reliably compile at the moment
    // time_tracking_cli_app::time_tracking_app();
    adveture_game::play_game();
}

autofill::autofill! {
    /// < 0 should return 1
    fn factorial(i: i32) -> i32 { todo!() }

    /// project u onto v
    fn project(u: (f64, f64), v: (f64, f64)) -> (f64, f64) { todo!() }
}
