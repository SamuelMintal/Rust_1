mod solution;
use solution::{Clock, Ticker, Alarm};
use std::{time::Duration, thread::sleep_ms};

fn main() {
    let mut clock = Clock::new(Duration::from_secs(1));
    let ticker = Ticker::new(&clock, || println!("TICKER"));
    let alarm = Alarm::new(&clock, 3, || println!("ALARM"));

    let channel = clock.channel();
    for _ in 0..5 {
        println!("clock tick num {}", channel.next());
    }

    sleep_ms(5500);
}