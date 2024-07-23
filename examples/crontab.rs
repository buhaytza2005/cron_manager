use cron_manager::cron_manager::{CronJob, CronManager};

fn main() {
    let mut manager = CronManager::new();
    for (i, job) in manager.list_jobs().iter().enumerate() {
        println!(
            "{}: {} {} # {:?}",
            i, job.schedule, job.command, job.comment
        );
    }

    manager.add_job(CronJob {
        schedule: "* * * * *".to_string(),
        command: "/bin/echo hello > /tmp/hello".to_string(),
        comment: Some("testing".to_string()),
    });
    manager.add_job(CronJob {
        schedule: "* * * * *".to_string(),
        command: "/bin/echo hello > /tmp/hello".to_string(),
        comment: Some("test".to_string()),
    });

    manager.remove_job_by_comment("testing");
}
