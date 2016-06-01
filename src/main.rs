extern crate inotify;

use inotify::INotify;
use inotify::ffi::*;
use std::path::Path;

enum ChangeEvent {
    LostTrack,
    PathChanged(String),
}

fn fire(event: ChangeEvent) {
}

fn main() {
    let mut inotify = INotify::init().unwrap();
    inotify.add_watch(Path::new("/home/chad"), IN_MODIFY | IN_ATTRIB | IN_CLOSE_WRITE | IN_MOVED_FROM | IN_MOVED_TO | IN_CREATE | IN_DELETE | IN_DELETE_SELF | IN_MOVE_SELF);
    loop {
        let events = inotify.wait_for_events().unwrap();
        for event in events.iter() {
            if event.is_ignored() {

            }
            if event.is_queue_overflow() {
                fire(ChangeEvent::LostTrack);
            } else {
                fire(ChangeEvent::PathChanged(event.name.clone()));
            }
            println!("event: {}", event.name);
        }
    }
}
