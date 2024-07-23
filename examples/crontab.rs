use cron_manager::cron_manager::{CronJob, CronManager};

fn main() {
    let mut manager = CronManager::new();

    let _ = list_cronjobs(&mut manager);
    let _ = add_cronjob(&mut manager);

    let _ = remove_job_by_id(&mut manager, 0);
    let _ = remove_job_by_comment(&mut manager, "test");
}

fn list_cronjobs(manager: &mut CronManager) {
    for (i, job) in manager.list_jobs().iter().enumerate() {
        println!("{}: {}", i, job);
    }
}

fn add_cronjob(manager: &mut CronManager) {
    manager.add_job(CronJob {
        schedule: "* * * * *".to_string(),
        command: "/bin/echo hello > /tmp/hello".to_string(),
        comment: Some("test".to_string()),
    });
}

fn remove_job_by_comment(manager: &mut CronManager, arg: &str) {
    manager.remove_job_by_comment(arg);
}

fn remove_job_by_id(manager: &mut CronManager, arg: usize) {
    manager.remove_job(arg);
}
