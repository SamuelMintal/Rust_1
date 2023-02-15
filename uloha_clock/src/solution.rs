use std::{thread, time::Duration};
use std::sync::{Arc, Mutex, mpsc};

pub struct Clock {
    channel_senders: Arc<Mutex<Vec<mpsc::Sender<u32>>>>,
    tick_period: Duration,
}

pub struct ClockChannelWrapper {
    reciever: mpsc::Receiver<u32>,
}

pub struct Ticker {
    //empty
}

pub struct Alarm {
    //also empty
}

impl Clock {
    pub fn new(tick_period: Duration) -> Clock {
        let mut channel_senders: Arc<Mutex<Vec<mpsc::Sender<u32>>>> = Arc::new(Mutex::new(Vec::new()));

        //Firstly spawn the counting thread of this clock
        let threads_arc = Arc::clone(&channel_senders);
        thread::spawn( move || {
            let mut tick_num = 0;

            while true {
                thread::sleep(tick_period);
                tick_num += 1;

                { //{...} to ensure lock unlocking after sending 
                    let senders_vec = threads_arc.lock().unwrap();

                    for i in 0..senders_vec.len() {
                        senders_vec[i].send(tick_num);
                    }
                }
                
            }
        });

        //then create clock and return it
        return Clock { channel_senders: channel_senders, tick_period: tick_period };

    }

    pub fn channel(&self) -> ClockChannelWrapper {
        let (tx, rx) = mpsc::channel();
        
        //push the sender into clocks vector of senders
        self.channel_senders.lock().unwrap().push(tx);
        
        return ClockChannelWrapper{ reciever: rx };
    }
}

impl ClockChannelWrapper {
    pub fn next(&self) -> u32 {
        return self.reciever.recv().unwrap();
    }
}

impl Ticker {
    pub fn new<FuncT: FnMut() + Send + 'static>(clock: &Clock, func: FuncT) -> Ticker {
        
        //spawn Ticker thread
        let mut func = func;
        let clock_channel_wrapper = clock.channel();
        thread::spawn( move || {
            while true {
                clock_channel_wrapper.next();
                func();
            }
        });
        
        return Ticker{};
    }
}

impl Alarm {
    pub fn new<FuncT: FnOnce() + Send + 'static>(clock: &Clock, delay_ticks: u32, func: FuncT) -> Alarm {

        //spawn Alarm thread
        let mut delay_ticks: u32 = delay_ticks;
        let clock_channel_wrapper = clock.channel();
        thread::spawn( move || {
            
            while true {

                if delay_ticks == 0 {
                    func();
                    
                    //killing the thread
                    break;
                }
                else {
                    delay_ticks -= 1;
                }             
                
                clock_channel_wrapper.next();
            }
        });

        return Alarm{};
    }
}