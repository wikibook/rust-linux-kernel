use chrono::{Utc, Local, FixedOffset};
use chrono::offset::TimeZone;

fn main() {
    // UTC시간 획득
    let now_utc = Utc::now();
    println!("UTC 시각: {}", now_utc);

    // 로컬 시간
    let now_local = Local::now();
    println!("로컬 시각: {}", now_local);

    // 서울 시간 획득 UTC+9
    let seoul_offset = FixedOffset::east(9 * 3600); // +9
    let seoul = seoul_offset.from_utc_datetime(&now_utc.naive_utc());
    println!("한국시각: {}", seoul);
}
