use core::time;
use std::collections::HashMap;

use crate::responses::*;
use rusqlite::Result;

pub async fn get_house_vote(
    tx: &rusqlite::Transaction<'_>,
    info_pulled: &(u16, u8, u32, String, u16),
    vote: &RollCallVoteHouse
) -> Result<()> {
    
    let (congress, session, roll_call, chamber, year) = info_pulled;

    let vote_metadata = vote.vote_metadata.clone();

    let vote_data = if let Some(vote_data) = vote.vote_data.clone() {
        vote_data
    } else {
        VoteDataHouse::default()
    };

    let majority =  vote_metadata.majority.unwrap_or("null".to_string());

    let committee = vote_metadata.committee.unwrap_or("null".to_string());

    let legis_num = vote_metadata.legis_num.unwrap_or("null".to_string());

    let vote_question = match vote_metadata.vote_question {
        Response::String(vote_question) => vote_question,
        _ => "null".to_string()
    };

    let vote_type =  match vote_metadata.vote_type {
        Response::String(vote_type) => vote_type,
        _ => "null".to_string()
    };

    let vote_result = match vote_metadata.vote_result {
        Response::String(vote_result) => vote_result,
        _ => "null".to_string()
    };

    let action_date = match vote_metadata.action_date {
        Response::String(action_date) => action_date,
        _ => "null".to_string()
    };

    let action_time = if let Some(action_time) = vote_metadata.action_time {
        let content = if let Some(content) = action_time.content {
            content
        } else {
            "null".to_string()
        };

        let time = if let Some(time) = action_time.time_etz {
            time
        } else {
            "null".to_string()
        };

        (content, time)
    } else {
        ("null".to_string(), "null".to_string())
    };

    let vote_desc = match vote_metadata.vote_desc {
        Response::String(vote_desc) => vote_desc,
        _ => "null".to_string()
    };

    let vote_totals = vote_metadata.vote_totals.unwrap_or(VoteTotalsHouse::default());

    let totals_by_candidate = vote_totals.totals_by_candidate.unwrap_or(Vec::from([TotalsByCandidateHouse::default()]));

    let totals_by_party = vote_totals.totals_by_party.unwrap_or(Vec::from([TotalsByPartyHouse::default()]));

    // total, yea, nay, present, absent
    let totals_by_vote = vote_totals.totals_by_vote.unwrap_or(TotalsByVoteHouse::default());


    Ok(())
}

