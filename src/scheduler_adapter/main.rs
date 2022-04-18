use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::application::rates::refresh;

pub struct Scheduler {}

impl Scheduler {
    pub fn init() {
        println!("Starting Scheduler...");
        start_scheduler();
    }
}

fn start_scheduler() {
    let scheduler = JobScheduler::new().unwrap();

    add_ecb_rates_job(&scheduler);

    scheduler.start().unwrap();
}

fn add_ecb_rates_job(scheduler: &JobScheduler) {
    let job = Job::new_repeated_async(Duration::from_secs(7), |_uuid, _l| {
        Box::pin(async move {
            refresh().await;
        })
    })
    .unwrap();

    scheduler.add(job).unwrap();
}
