use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use keepassxc_syncer::syncer::{Fetcher, Pusher};

fn spawn_fetcher<FR, MR>(mutex: &Arc<Mutex<i8>>, fetcher: &'static dyn Fetcher<FetchResult=FR, MergeResult=MR>) -> thread::JoinHandle<()> {
    let mutex = Arc::clone(mutex);
    thread::spawn(move || loop {
        let fr = &fetcher.fetch();
    })
}

fn spawn_pusher<PR>(mutex: &Arc<Mutex<i8>>, pusher: &'static dyn Pusher<PushResult=PR>) -> thread::JoinHandle<()> {
    let mutex = Arc::clone(mutex);
    thread::spawn(move || loop {
        
    })
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || loop {
            {
                let lck = mutex.lock();
                let mut mutex = lck.unwrap();
                println!("T1: {}", mutex);
                *mutex += 1;
            }
            thread::sleep(time::Duration::from_millis(1000));
        });
        handles.push(handle);
    }
    {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || loop {
            {
                let lck = mutex.lock();
                let mut mutex = lck.unwrap();
                println!("T2: {}", mutex);
                *mutex += 1;
            }
            thread::sleep(time::Duration::from_millis(1000));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join();
    }
}
