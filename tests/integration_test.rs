use sigmaos_tools_omacut::{TimeCode, TrimJob, TrimQueue};

#[test]
fn test_timecode_conversion_aaa() {
    // Arrange
    let tc = TimeCode::new(0, 1, 30, 500);
    // Act
    let ms = tc.to_milliseconds();
    let reconstructed = TimeCode::from_milliseconds(ms);
    // Assert
    assert_eq!(ms, 90500);
    assert_eq!(reconstructed, tc);
}

#[test]
fn test_trim_job_validation_aaa() {
    // Arrange
    let start = TimeCode::new(0, 2, 0, 0);
    let end = TimeCode::new(0, 1, 0, 0);
    // Act
    let res = TrimJob::new("/tmp/in.mp4", "/tmp/out.mp4", start, end);
    // Assert
    assert!(res.is_err());
}

#[test]
fn test_queue_total_duration_aaa() {
    // Arrange
    let mut queue = TrimQueue::new();
    let job1 = TrimJob::new(
        "/tmp/1.mp4",
        "/tmp/o1.mp4",
        TimeCode::new(0, 0, 0, 0),
        TimeCode::new(0, 0, 10, 0),
    )
    .unwrap();
    let job2 = TrimJob::new(
        "/tmp/2.mp4",
        "/tmp/o2.mp4",
        TimeCode::new(0, 0, 0, 0),
        TimeCode::new(0, 0, 5, 0),
    )
    .unwrap();
    // Act
    queue.add_job(job1);
    queue.add_job(job2);
    // Assert
    assert_eq!(queue.total_duration_ms(), 15000);
}
