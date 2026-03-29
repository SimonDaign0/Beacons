use esp_hal::time::{ Instant, Duration };

pub fn blocking_delay(delay: u64) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(delay) {}
}
