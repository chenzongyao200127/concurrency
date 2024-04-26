use anyhow::{anyhow, Result};
use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

const NUM_PER_THREAD: usize = 4;

#[derive(Debug)]
#[allow(dead_code)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for idx in 0..NUM_PER_THREAD {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }

    // drop the last tx so that the rx will be closed
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer thread done");
        Arc::new("secret".to_string())
    });

    let secert = consumer
        .join()
        .map_err(|e| anyhow!("Consumer thread failed: {:?}", e))?;

    println!("Secret: {}", secert);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let num = rand::random::<usize>();
        tx.send(Msg::new(idx, num))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // randomly stop the thread
        if rand::random::<bool>() {
            break;
        }
    }

    // Do more work here
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
