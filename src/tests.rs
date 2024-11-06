use crate::{endpoints::CongressEndpoint as Congress, CURRENT_CONGRESS, OLDEST_CONGRESS};

#[test]
fn test_congress() {
    let congress = Congress::new("house", 116, 2, 1);
    assert_eq!(congress.to_year(), 2020);
    assert_eq!(congress.to_string(), "https://clerk.house.gov/evs/2020/roll001.xml");

    let congress = Congress::new("senate", 116, 2, 1);
    assert_eq!(congress.to_year(), 2020);
    assert_eq!(congress.to_string(), "https://www.senate.gov/legislative/LIS/roll_call_votes/vote1162/vote_116_2_00001.xml");
}

#[test]
fn test_congress_current() {
    let congress = Congress::current("house");
    assert_eq!(congress.to_year(), 2024);
    assert_eq!(congress.to_string(), "https://clerk.house.gov/evs/2024/roll255.xml");

    let congress = Congress::current("senate");
    assert_eq!(congress.to_year(), 2024);
    assert_eq!(congress.to_string(), "https://www.senate.gov/legislative/LIS/roll_call_votes/vote1182/vote_118_2_00255.xml");
}

#[test]
fn test_congress_display() {
    let congress = Congress::new("house", 116, 2, 1);
    assert_eq!(congress.to_string(), "https://clerk.house.gov/evs/2020/roll001.xml");

    let congress = Congress::new("senate", 116, 2, 1);
    assert_eq!(congress.to_string(), "https://www.senate.gov/legislative/LIS/roll_call_votes/vote1162/vote_116_2_00001.xml");
}

#[test]
fn test_all_congresses() {
    let max_congress = CURRENT_CONGRESS;
    let min_congress = OLDEST_CONGRESS;
    let current_roll_senate = 255;
    let current_roll_house = 455;

    for congress in min_congress..=max_congress {
        // session 1 and 2
        for session in 1..3 {
            println!("Congress: {}, Session: {}", congress, session);
            println!("\n\n\n");
            println!("Senate");
            println!("-------------------");
            for roll in 1..=current_roll_senate {
                let congress = Congress::new("senate", congress, session, roll);
                println!("{}", congress);
            }
            println!("\n\n\n");
            println!("House");
            println!("-------------------");
            for roll in 1..=current_roll_house {
                let congress = Congress::new("house", congress, session, roll);
                println!("{}", congress);
            }
        }
    }

}


use crate::client::RollCallClient as Client;

#[tokio::test]
async fn test_client() {
    let client = Client::new(false);
    let data = client.fetch_data(&Congress::current("house").to_string()).await.unwrap();
    println!("{}", data);
    assert_eq!(data.contains("rollcall-vote"), true);
}


