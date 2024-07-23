#[cfg(test)]
mod tests {
    use cron_manager::cron_manager::*;
    use rand::Rng;

    #[test]
    fn test_cron_manager() {
        let mut manager = CronManager::new();

        let original_len = manager.list_jobs().len();
        // Add jobs
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

        // Verify jobs were added
        let jobs = manager.list_jobs();
        assert_eq!(jobs.len(), original_len + 2);

        // Get the comment from one of the jobs
        let comments: Vec<String> = jobs.iter().filter_map(|job| job.comment.clone()).collect();

        assert!(comments.contains(&"testing".to_string()));
        assert!(comments.contains(&"test".to_string()));

        // Remove job by random comment
        let rand_comm = vec!["test", "testing"];

        let mut rng = rand::thread_rng();
        let random_bool = rng.gen();
        let r = if random_bool { 1 } else { 0 };
        manager.remove_job_by_comment(rand_comm[r]);

        // Verify job was removed
        let jobs_after_removal = manager.list_jobs();
        assert!(jobs_after_removal.len() < original_len + 2);

        let remaining_comments: Vec<String> = jobs_after_removal
            .iter()
            .filter_map(|job| job.comment.clone())
            .collect();

        // Check if the correct job was removed
        assert!(!remaining_comments.contains(&rand_comm[r].to_string()));
    }
}
