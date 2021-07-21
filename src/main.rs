mod battery;
mod notification;

fn main() {
    let percentage = battery::get_battery_percentage().expect("could not process battery output");

    notification::notify(percentage);
}
