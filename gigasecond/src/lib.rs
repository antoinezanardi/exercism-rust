use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_seconds = 1000000000;
    start.checked_add(giga_seconds.seconds()).unwrap()
}
