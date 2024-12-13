use std::time::Duration;
use final_project::site_status_check::{run_monitoring, verifySite, statsCalculation, WebsiteStatus};

#[test]
fn testMonitoring() {
    let websites = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.github.com".to_string(),
    ];

    let runtime = Duration::from_secs(5);
    let count = 2;
    let monitorPeriod = Duration::from_secs(20);
    let maximumTries = 1;

    run_monitoring(websites, runtime, count, monitorPeriod, maximumTries);
}

#[test]
fn testSuccess() {
    let url = "https://www.rust-lang.org";
    let runtime = Duration::from_secs(5);
    let result = verifySite(url, runtime);

    assert!(result.status.is_ok(), "Expected successful status, got {:?}", result.status);
}

#[test]
fn testFailure() {
    let url = "https://invalid.url";
    let runtime = Duration::from_secs(5);
    let result = verifySite(url, runtime);

    assert!(result.status.is_err(), "Expected error status, got {:?}", result.status);
}

#[test]
fn testCalculation() {
    let results = vec![
        WebsiteStatus {
            url: "https://www.rust-lang.org".to_string(),
            status: Ok(200),
            response_time: Duration::from_millis(100),
            timestamp: chrono::Utc::now(),
            validSSL: true,
        },
        WebsiteStatus {
            url: "https://www.github.com".to_string(),
            status: Ok(200),
            response_time: Duration::from_millis(150),
            timestamp: chrono::Utc::now(),
            validSSL: true,
        }
    ];
    statsCalculation(&results);
}
