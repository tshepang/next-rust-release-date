use chrono::{Duration, Local, NaiveDate};

static WEEKS: i64 = 6;

fn main() {
    for n in 0.. {
        // Rust 1.0 release
        // ... will not panic, because this date exists
        let initial = NaiveDate::from_ymd_opt(2015, 5, 15).unwrap();
        // 1.0 was released on a Friday, but nowadays they happen Thursdays
        let release = initial
            .checked_add_signed(Duration::weeks(WEEKS * n))
            .unwrap()
            - Duration::days(1);
        if release >= Local::now().date_naive() {
            let previous_release = release.checked_sub_signed(Duration::weeks(WEEKS)).unwrap();
            println!("  {} - Rust 1.{}", previous_release, n - 1);
            println!("* {} - Rust 1.{}", release, n);
            let after_next = release.checked_add_signed(Duration::weeks(WEEKS)).unwrap();
            println!("  {} - Rust 1.{}", after_next, n + 1);
            break;
        }
    }
}
