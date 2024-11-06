// make a Congress struct that holds the Congress number, session, and roll call number
//
// When displaying the Congress struct, it should return the URL to the roll call vote it
// represents

use crate::{CURRENT_SESSION, CURRENT_CONGRESS, CURRENT_ROLL, OLDEST_CONGRESS};

type CongressNumber = u32;
type CongressSession = u32;
type RollCall = u32;

type Chamber = CongressChamber;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CongressChamber {
    #[default]
    House,
    Senate,
}

impl std::str::FromStr for Chamber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "house" => Ok(CongressChamber::House),
            "senate" => Ok(CongressChamber::Senate),
            _ => Err(format!("Invalid chamber: {}", s)),
        }
    }
}

impl Into<Chamber> for &str {
    fn into(self) -> Chamber {
        match self.to_lowercase().as_str() {
            "house" => CongressChamber::House,
            "senate" => CongressChamber::Senate,
            _ => CongressChamber::House,
        }
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CongressChamber::House => write!(f, "https://clerk.house.gov/evs/"),
            CongressChamber::Senate => write!(f, "https://www.senate.gov/legislative/LIS/roll_call_votes/"),
        }
    }
}

impl Chamber {
    pub fn to_string(&self) -> String {
        match self {
            CongressChamber::House => "house".to_string(),
            CongressChamber::Senate => "senate".to_string(),
        }
    }
}


pub type CongressEndpoint = CongressFields;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CongressFields {
    pub chamber: CongressChamber,
    pub number: CongressNumber,
    pub session: CongressSession,
    pub roll_call: RollCall,
}

impl CongressEndpoint {
    pub fn new(chamber: &str, number: u32, session: u32, roll_call: u32) -> Self {
        CongressFields {
            chamber: chamber.into(),
            number,
            session,
            roll_call,
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn current(chamber: &str) -> Self {
        CongressFields {
            chamber: chamber.into(),
            number: CURRENT_CONGRESS,
            session: CURRENT_SESSION,
            roll_call: CURRENT_ROLL,
        }
    }

    pub fn chamber(&mut self, chamber: &str) -> &mut Self {
        let chamber = chamber.into();
        self.chamber = chamber;
        self
    }

    pub fn number(&mut self, number: u32) -> Result<&mut Self, String> {
        if number < OLDEST_CONGRESS || number > CURRENT_CONGRESS {
            return Err(format!("Invalid Congress number: {}", number));
        }

        self.number = number;
        Ok(self)
    }

    pub fn session(&mut self, session: u32) -> Result<&mut Self, String> {
        if session != 1 && session != 2 {
            return Err(format!("Invalid Congress session: {}", session));
        }

        self.session = session;
        Ok(self)
    }

    pub fn roll_call(&mut self, roll_call: u32) -> &mut Self {
        self.roll_call = roll_call;
        self
    }

    pub fn build(&self) -> CongressEndpoint {
        *self
    }

    // Convert Congress number + session to year
    pub fn to_year(&self) -> u32 {
        match (self.number, self.session) {
            (120, 2) => 2028,
            (120, 1) => 2027,
            (119, 2) => 2026,
            (119, 1) => 2025,
            (118, 2) => 2024,
            (118, 1) => 2023,
            (117, 2) => 2022,
            (117, 1) => 2021,
            (116, 2) => 2020,
            (116, 1) => 2019,
            (115, 2) => 2018,
            (115, 1) => 2017,
            (114, 2) => 2016,
            (114, 1) => 2015,
            (113, 2) => 2014,
            (113, 1) => 2013,
            (112, 2) => 2012,
            (112, 1) => 2011,
            (111, 2) => 2010,
            (111, 1) => 2009,
            (110, 2) => 2008,
            (110, 1) => 2007,
            (109, 2) => 2006,
            (109, 1) => 2005,
            (108, 2) => 2004,
            (108, 1) => 2003,
            (107, 2) => 2002,
            (107, 1) => 2001,
            (106, 2) => 2000,
            (106, 1) => 1999,
            (105, 2) => 1998,
            (105, 1) => 1997,
            (104, 2) => 1996,
            (104, 1) => 1995,
            (103, 2) => 1994,
            (103, 1) => 1993,
            (102, 2) => 1992,
            (102, 1) => 1991,
            (101, 2) => 1990,
            (101, 1) => 1989,
            _ => {
                panic!("Invalid Congress number and session: {} {}", self.number, self.session);
            }
        }
    }
}



impl std::fmt::Display for CongressEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.chamber {
            CongressChamber::House => {
                write!(f, "{}{}/roll{roll:0>3}.xml", self.chamber, self.to_year(), roll = self.roll_call)
            }
            CongressChamber::Senate => {
                write!(f, "{}vote{congress}{session}/vote_{congress}_{session}_{roll:0>5}.xml", self.chamber, congress = self.number, session = self.session, roll = self.roll_call)
            }
        }
    }
}


