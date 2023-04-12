use crate::*;


pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(password.as_bytes());
    let hashed = hasher.finish();
    hashed.to_string()
}

pub fn current_time_as_padded_string() -> String {
    let seconds_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let seconds_since_epoch_string = seconds_since_epoch.to_string();
    let padding = u64::MAX.to_string().len() - seconds_since_epoch_string.len();
    let mut date = String::new();
    for _ in 0..padding {
        date.push('0');
    }
    date.push_str(&seconds_since_epoch_string);

    date
}

pub fn padded_time_to_date_string(padded_time: &str, fmt: &str) -> String {
    let seconds_since_epoch = padded_time.parse::<u64>().unwrap();
    let d = UNIX_EPOCH + std::time::Duration::from_secs(seconds_since_epoch);
    let datetime = chrono::DateTime::<chrono::Local>::from(d);
    datetime.format(fmt).to_string()
}