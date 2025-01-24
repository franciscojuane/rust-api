#![allow(dead_code)]
#[derive(Debug)]
pub enum RegionEnum{
    AL, AK, AZ, AR, CA, CO, CT, DE, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY,
    NC, ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV,
    WI, WY
}


impl RegionEnum {
    pub fn name(&self) -> &str{
        match self {
            RegionEnum::AL => "Alabama",
            RegionEnum::AK => "Alaska",
            RegionEnum::AZ => "Arizona",
            RegionEnum::AR => "Arkansas",
            RegionEnum::CA => "California",
            RegionEnum::CO => "Colorado",
            RegionEnum::CT => "Connecticut",
            RegionEnum::DE => "Delaware",
            RegionEnum::FL => "Florida",
            RegionEnum::GA => "Georgia",
            RegionEnum::HI => "Hawaii",
            RegionEnum::ID => "Idaho",
            RegionEnum::IL => "Illinois",
            RegionEnum::IN => "Indiana",
            RegionEnum::IA => "Iowa",
            RegionEnum::KS => "Kansas",
            RegionEnum::KY => "Kentucky",
            RegionEnum::LA => "Louisiana",
            RegionEnum::ME => "Maine",
            RegionEnum::MD => "Maryland",
            RegionEnum::MA => "Massachusetts",
            RegionEnum::MI => "Michigan",
            RegionEnum::MN => "Minnesota",
            RegionEnum::MS => "Mississippi",
            RegionEnum::MO => "Missouri",
            RegionEnum::MT => "Montana",
            RegionEnum::NE => "Nebraska",
            RegionEnum::NV => "Nevada",
            RegionEnum::NH => "New Hampshire",
            RegionEnum::NJ => "New Jersey",
            RegionEnum::NM => "New Mexico",
            RegionEnum::NY => "New York",
            RegionEnum::NC => "North Carolina",
            RegionEnum::ND => "North Dakota",
            RegionEnum::OH => "Ohio",
            RegionEnum::OK => "Oklahoma",
            RegionEnum::OR => "Oregon",
            RegionEnum::PA => "Pennsylvania",
            RegionEnum::RI => "Rhode Island",
            RegionEnum::SC => "South Carolina",
            RegionEnum::SD => "South Dakota",
            RegionEnum::TN => "Tennessee",
            RegionEnum::TX => "Texas",
            RegionEnum::UT => "Utah",
            RegionEnum::VT => "Vermont",
            RegionEnum::VA => "Virginia",
            RegionEnum::WA => "Washington",
            RegionEnum::WV => "West Virginia",
            RegionEnum::WI => "Wisconsin",
            RegionEnum::WY => "Wyoming"
        }
    }


}