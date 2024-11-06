use crate::responses::*;
use rusqlite::Result;

pub async fn get_house_vote(
    tx: &rusqlite::Transaction<'_>,
    info_pulled: &(u16, u8, u32, String, u16),
    vote: &RollCallVoteHouse
) -> Result<()> {
    




    Ok(())
}

