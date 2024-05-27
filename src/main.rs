use chrono::{Datelike, TimeZone, Utc};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let now = Utc::now();
    let pos = now.ordinal() as u64;

    let timezone = now.timezone();
    let year = now.year();

    let start = timezone.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
    let end = timezone.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap();
    let len = (end - start).num_days() as u64;

    let pb = ProgressBar::new(len);
    pb.set_prefix(format!("{} is {:.0}% complete", year, (pos as f64 / len as f64) * 100.0));
    let remaining = format!("{} days remaining", len - pos);
    pb.set_style(ProgressStyle::with_template(&format!("{{prefix:.bold}} {{bar:40.blue}} {{pos}}/{{len}} [{}]", remaining))
        .unwrap()
        .progress_chars("█▓▒░")
    );
    pb.inc(pos);
    pb.abandon();
}