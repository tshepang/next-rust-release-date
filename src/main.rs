use jiff::civil::Date;
use jiff::ToSpan;
use jiff::Zoned;

static WEEKS: i64 = 6;

fn main() {
    for n in 0.. {
        // Rust 1.0 release
        // ... will not panic, because this date exists
        let initial = jiff::civil::date(2015, 5, 15);
        // 1.0 was released on a Friday, but nowadays they happen Thursdays
        let release = initial + WEEKS.weeks() * n - 1.days();
        if release >= Date::from(Zoned::now()) {
            let previous_release = release - WEEKS.week();
            println!("  {} - Rust 1.{}", previous_release, n - 1);
            println!("* {} - Rust 1.{}", release, n);
            let after_next = release + WEEKS.week();
            println!("  {} - Rust 1.{}", after_next, n + 1);
            break;
        }
    }
}
