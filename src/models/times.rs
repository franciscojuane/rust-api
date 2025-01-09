use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Times {
    creation_time: Option<DateTime<Utc>>,
    update_time: Option<DateTime<Utc>>,
    effective_time: Option<DateTime<Utc>>,
    expiration_time: Option<DateTime<Utc>>
}


impl Times {
    pub fn is_effective(&self, date: DateTime<Utc> ) -> bool {
        if  self.effective_time.is_some() && self.expiration_time.is_some() {
            self.effective_time.unwrap() > date && self.expiration_time.unwrap() < date
        } else if self.effective_time.is_some() {
            self.effective_time.unwrap() < date
        } else if self.expiration_time.is_some() {
            self.expiration_time.unwrap() > date
        } else{
            true
        }

    }

    pub fn is_effective_now(&self) -> bool {
        self.is_effective(Utc::now())
    }
    
    pub fn new_with_current_creation_time() -> Self {
        Self {
            creation_time: Option::Some(Utc::now()),
            update_time: None,
            effective_time: None,
            expiration_time: None,
        }
    }
}