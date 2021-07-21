mod battery;
mod cli;
mod notification;

fn main() {
    let opts: cli::Opts = cli::parse();

    let percentage = battery::get_battery_percentage().expect("could not process battery output");

    if percentage >= opts.threshold {
        notification::notify(percentage);
    }
}
