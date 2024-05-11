use crate::models::event::Event;

pub fn print_results(events: Vec<Event>) {
    for event in events {
        println!("{:?}", event);
    }
}