use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use clokwerk::AsyncJob;
use clokwerk::AsyncScheduler;
use clokwerk::Interval;
use clokwerk::Interval::*;
use serde_json::Value;

#[derive(Clone)]
pub struct AsyncTask;

impl AsyncTask {
  pub fn every_seconds<F, Fut>(seconds: u32, task: F) -> ()
  where
    F: FnMut() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
  {
    let mut scheduler = AsyncScheduler::new();
    let seconds = Interval::Seconds(seconds);
    scheduler.every(seconds).run(task);

    tokio::spawn(async move {
      loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
      }
    });
  }
}
