# poc-netbird-cron
extract netbird log events

[Netbird events api](https://docs.netbird.io/api/resources/events)
[Service Users/tokens](https://app.netbird.io/team/service-users)

- queries the netbird api for events (gets the last 10k)
- ensures events are sorted by date
- checks s3 log bucket for events that have been saved already
- merges new and old events
- saves events to log bucket

## Usage

```bash
export NETBIRD_API_TOKEN="some bot users token"
# + AWS credentials
cargo run
```
