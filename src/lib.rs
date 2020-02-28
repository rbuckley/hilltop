use date_time::date_tuple::Date;
use date_time::time_tuple::TimeOfDay;

use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq)]
enum Status {
    Waitlisted(i32, Date),
    Enrolled(Date),
    Withdrawn(Date),
    Interested(Date),
}

#[derive(Debug, Hash, PartialEq)] 
struct Child {
    first_name: String,
    last_name: String,
    dob: Date,
    start_date: Option<Date>,
    end_date: Option<Date>,
    start_time: Option<TimeOfDay>,
    end_time: Option<TimeOfDay>,
    /// bit mask for M Tu W Th F
    /// 0x0 = 0 days
    /// 0x1F = MTuWThF
    /// 0x01 = M
    /// 0x03 = MTu
    /// etc etc
    attendance_days: u8,
    status: Status,
}

struct Roster {
    roster: HashSet<Child>,
}

impl Roster {
    fn add(c: Child) {
        roster.insert(c);
    }

    fn remove (&c: Child) {
        roster.remove(&c);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_roster() {
        let roster = Roster::new();
        assert!(roster);
    }
}
