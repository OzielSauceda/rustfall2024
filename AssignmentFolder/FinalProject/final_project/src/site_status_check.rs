use chrono::{Utc, DateTime};
use std::sync::{Arc, Mutex};
use serde::Serialize;
use std::time::{Duration, Instant};
use std::thread;
use ureq;
use std::sync::mpsc;

//This fucntion shows the status of the website.
#[derive(Debug, Serialize, PartialEq)]
pub struct WebsiteStatus{
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
    pub validSSL: bool,    
}

//In order to run the websites concurrently, were going to be using this function
pub fn run_monitoring(
    websites: Vec<String>,
    timeout: Duration,
    threads: usize,
    interval: Duration,
    max_retries: usize,
){
    let originalSites = websites.clone();
    let websites = Arc::new(Mutex::new(websites));
    loop {{
        //This will rrefill the queue if it happens to be empty
            let mut websites_lock = websites.lock().unwrap();
            if websites_lock.is_empty(){
                for url in &originalSites{
                    websites_lock.push(url.clone());}
            }
        }

        let clonedSites = Arc::clone(&websites);
        let mut handler = vec![];
        let (transmitter, reciever)= mpsc::channel();

        //The for loop will spawn the tjhreads to check the sites
        for _ in 0..threads{
            let websites = Arc::clone(&clonedSites);
            let transmitter = transmitter.clone();

            let handle = thread::spawn(move ||{
                loop {
                    //Gets a random URL from the queue
                    let url ={
                        let mut lockedSites = websites.lock().unwrap();
                        if let Some(x) = lockedSites.pop() {x} 
                            else{
                            break;
                        }
                    };
                    //Does the same logic for the site verification
                    let mut attempts = 0;
                    let mut result = None;

                    while attempts < max_retries {
                        let status = verifySite(&url, timeout);
                        if status.status.is_ok(){
                            result = Some(status);
                            break;}
                        attempts += 1;
                    }
                    //Right before it fails, it retreies one last time
                    if result.is_none(){
                        result = Some(verifySite(&url, timeout));
                    }
                    transmitter.send(result.unwrap()).unwrap();
                }
            });
            handler.push(handle);
        }

        drop(transmitter);
        let mut results: Vec<WebsiteStatus>= vec![];
        //This while loop will collect and process the resuklts of what it gets.
        while let Ok(status) = reciever.recv(){
            println!(
                "WebsiteStatus {{ url: \"{}\", responseTime: {:?}, status: {:?}, isValidSSL: {}, date: {}}}",
                status.url,
                status.response_time,
                status.status,
                status.validSSL,
                status.timestamp,
            );
            results.push(status);
            statsCalculation(&results);
        }
        for handle in handler{
            handle.join().unwrap();
        }
        println!("\n--Hold for the upcoming cycle--\n");
        thread::sleep(interval);
    }
}

//Checks the status of one website at a time
pub fn verifySite(url: &str, timeout: Duration) -> WebsiteStatus {
    let tester = ureq::AgentBuilder::new().timeout(timeout).build();
    let startTime = Instant::now();
    let responseResult = tester.get(url).call();
    let timePassed = startTime.elapsed();
    let validSSL = url.starts_with("https://");

    let status = match responseResult {
        Ok(resp) => Ok(resp.status()),
        Err(_) => Err("Failed to get the URL".to_string()),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: timePassed,
        timestamp: Utc::now(),
        validSSL,
    }
}

//This fucntion does the calculation and prints the sites stats
pub fn statsCalculation(results: &Vec<WebsiteStatus>) {
    let total = results.len();
    let working: Vec<&WebsiteStatus> = results.iter().filter(|s| s.status.is_ok()).collect();
    let averageTime = if !working.is_empty() {
        working
            .iter().map(|s| s.response_time).sum::<Duration>() / working.len() as u32
    } else {
        Duration::new(0, 0)
    };

    println!(
        "Analysis:\nTotal Websites: {}\nHow many are Successful: {}\nAverage Response Time: {:?}\n",
        total,
        working.len(),
        averageTime
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testSiteVerification(){
        let status = verifySite("https://www.rust-lang.org", Duration::from_secs(5));
        assert!(status.status.is_ok());
    }
    #[test]
    fn testError() {
        let status = verifySite("https://nonexistent.website", Duration::from_secs(5));
        assert!(status.status.is_err());
    }
    #[test]
    fn testCalculation(){
        let results = vec![
            WebsiteStatus{
                url: "https://example.com".to_string(),
                status: Ok(200),
                response_time: Duration::from_secs(1),
                timestamp: Utc::now(),
                validSSL: true,
            }
        ];
        statsCalculation(&results);
        assert_eq!(results.len(), 1);
    }
}