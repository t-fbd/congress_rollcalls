pub const CURRENT_CONGRESS: u32 = 118;
pub const CURRENT_SESSION: u32 = 2;
pub const OLDEST_CONGRESS: u32 = 101;
pub const CURRENT_ROLL: u32 = 255;

pub mod client {
    use reqwest::Client;

    pub struct RollCallClient {
        client: Client,
    }

    impl RollCallClient {
        pub fn new(proxy: bool) -> Self {

            let client = match proxy {
                true => {
                    let c = match Client::builder()
                    .proxy(reqwest::Proxy::all("").unwrap())
                    .build() {
                        Ok(client) => client,
                        Err(e) => panic!("Error creating client: {}", e),
                    };
                    c
                }
                false => Client::new(),
            };

            RollCallClient {
                client,
            }
        }

        pub async fn fetch_data(&self, url: &str) -> Result<String, reqwest::Error> {
            self.client.get(url).send().await?.text().await
        }
    }
}

pub mod endpoints;
pub mod responses;

pub mod sqllite;
pub mod sql_senate;
pub mod sql_house;

pub mod json_master;

#[cfg(test)]
pub mod tests;

