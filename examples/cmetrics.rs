use anyhow::Result;
use concurrency::CmapMetrics;
use rand::Rng;
use std::thread;

const N: usize = 2;
const M: usize = 2;

fn main() -> Result<()> {
    let metrics = CmapMetrics::new();

    // start N works and M requests

    println!("Start {} workers and {} requests", N, M);
    println!("{:?}", metrics);

    for i in 0..N {
        task_worker(i, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
        println!("{}", metrics);
    }

    // Ok(())
}

fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..666)));
            let _ = metrics.inc(format!("call.thread.worker.{}", idx));
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..700)));
            let page = rng.gen_range(1..=256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
