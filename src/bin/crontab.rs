use cron_manager::cron_manager::CronManager;

fn main() {
    let mut manager = CronManager::new();
    for (i, job) in manager.list_jobs().iter().enumerate() {
        println!(
            "{}: {} {} # {:?}",
            i, job.schedule, job.command, job.comment
        );
    }
}
