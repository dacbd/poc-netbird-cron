mod netbird;
use itertools::Itertools;
use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info, Level};

use netbird::events::Event;

const NETBIRD_BASE_URL: &str = "https://api.netbird.io";

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, env = "NETBIRD_API_TOKEN")]
    api_token: String,
    #[clap(long, default_value = "temp-netbird-event-logs")]
    log_bucket: String,
    #[arg(short, long, default_value_t = Level::INFO)]
    log: Level,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().with_max_level(args.log).init();
    info!(
        log_bucket = args.log_bucket,
        "starting netbird event log sync"
    );
    let bucket = s3::Bucket::new(
        &args.log_bucket,
        s3::Region::UsEast1,
        s3::creds::Credentials::default()?,
    )?;

    let client = reqwest::Client::new();
    let netbird = netbird::Netbird::new(NETBIRD_BASE_URL, client, &args.api_token);
    let mut events = netbird.get_events().await?;
    info!(count = events.len(), "got events");

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    info!("sorted events by timestamp");

    for (key, group) in &events
        .into_iter()
        .group_by(|event| event.timestamp.date_naive())
    {
        let days_events: Vec<Event> = group.collect();
        info!(date = %key, count = days_events.len(), "grouping events");
        let key_str = key.to_string();
        let log_path = format!("{}.json", key_str);

        match bucket.get_object(&log_path).await {
            Ok(response) => {
                let raw_data = response.bytes();
                let data = serde_json::from_slice::<Vec<Event>>(raw_data)?;
                info!(date = %key, count = data.len(), "got existing events from s3");

                let complete_events = merge_events(data, days_events);
                info!(date = %key, count = complete_events.len(), "merged events");
                let json = serde_json::to_string_pretty(&complete_events)?;
                bucket
                    .put_object_with_content_type(&log_path, json.as_bytes(), "application/json")
                    .await?;
            }
            Err(s3_error) => match s3_error {
                s3::error::S3Error::Http(404, _) => {
                    info!(date = %key, "no s3 events for this day");
                    let json = serde_json::to_string_pretty(&days_events)?;
                    bucket
                        .put_object_with_content_type(
                            &log_path,
                            json.as_bytes(),
                            "application/json",
                        )
                        .await?;
                }
                _ => {
                    error!(error = %s3_error, "error getting s3 events");
                    return Err(s3_error.into());
                }
            },
        }
    }
    info!("Job complete!");
    Ok(())
}

fn merge_events(a: Vec<Event>, b: Vec<Event>) -> Vec<Event> {
    let mut events_map: HashMap<String, Event> = HashMap::new();
    for e in a {
        events_map.insert(e.id.clone(), e);
    }
    for e in b {
        events_map.insert(e.id.clone(), e);
    }
    let mut events = events_map.into_values().collect::<Vec<Event>>();
    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    events
}
