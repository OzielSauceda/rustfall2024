mod site_status_check;
use std::time::Duration;

fn main() {
    let websites = vec![
        "https://www.google.com",
        "https://www.rust-lang.org",
        "https://www.github.com",
        "https://www.microsoft.com",
        "https://www.apple.com",
        "https://www.amazon.com",
        "https://www.facebook.com",
        "https://www.youtube.com",
        "https://www.airbnb.com",
        "https://www.wikipedia.org",
        "https://www.twitter.com",
        "https://www.linkedin.com",
        "https://www.netflix.com",
        "https://www.instagram.com",
        "https://www.stackoverflow.com",
        "https://www.nytimes.com",
        "https://www.bbc.com",
        "https://www.cnn.com",
        "https://www.yahoo.com",
        "https://www.espn.com",
        "https://www.twitch.tv",
        "https://www.pinterest.com",
        "https://www.imdb.com",
        "https://www.walmart.com",
        "https://www.bbc.co.uk",
        "https://www.paypal.com",
        "https://www.dropbox.com",
        "https://www.medium.com",
        "https://www.quora.com",
        "https://www.appleinsider.com",
        "https://www.theguardian.com",
        "https://www.aljazeera.com",
        "https://www.weather.com",
        "https://www.skysports.com",
        "https://www.techcrunch.com",
        "https://www.wired.com",
        "https://www.forbes.com",
        "https://www.nbcnews.com",
        "https://www.zdnet.com",
        "https://www.usatoday.com",
        "https://www.gizmodo.com",
        "https://www.lifehacker.com",
        "https://www.engadget.com",
        "https://www.theverge.com",
        "https://www.cnet.com",
        "https://www.businessinsider.com",
        "https://www.bloomberg.com",
        "https://www.ft.com",
        "https://www.economist.com",
        "https://www.nationalgeographic.com",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let Runtime = Duration::from_secs(5);
    let count = 8;
    let monitorPeriod = Duration::from_secs(20);
    let maximumTries = 3;

    site_status_check::run_monitoring(websites, Runtime, count, monitorPeriod, maximumTries);
}