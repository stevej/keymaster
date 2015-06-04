use std::sync::Mutex;

extern crate iron;
use iron::prelude::*;
use iron::status;

extern crate time;

pub struct Keymaster {
    worker_id: u64, // this should be a 48-bit value that uniquely identifies the machine. It's designed with a MAC address in mind.
    sequence_id: Mutex<u32>
}

impl Keymaster {
    pub fn create(worker_id: u64) -> Keymaster {
        Keymaster {
            worker_id: worker_id,
            sequence_id: Mutex::new(0 as u32)
        }
    }

    fn next_sequence_id(&self) -> u32 {
        let mut data = self.sequence_id.lock().unwrap();
        *data += 1;
        let id = *data;
        return id;
    }

    pub fn next_id(&self) -> (u64, u64) {
        // Yes, I know that we're only shifting 16 bits but sequence_id is 32 bits.
        // Rust has no way of specifying a 16-bit value. If you care, please send a pull request. thanks!
        let state_id = (self.worker_id << 16) | self.next_sequence_id() as u64;
        // generate a new id based on the time and a worker id.
        return (time::get_time().sec as u64, state_id)
    }
}

fn main() {
    let km = Keymaster::create(0 as u64);

    Iron::new(move |_: &mut Request| {
        let (high, low) = km.next_id();
        Ok(Response::with((status::Ok, format!("{}{}\n", high, low ))))
    }).http("localhost:3000").unwrap();

    println!("On 3000");
}

