// sqllite.rs

use crate::responses::*;
use crate::sqllite::generate_hash;

pub async fn insert_vote_metadata_senate(tx: &rusqlite::Transaction<'_>, metadata: (u16, u8, u32, String, u16, String, String, String, String, String, String, String, String, Vec<DocumentSenate>, Vec<AmendmentSenate>, String, (u32, u32, u32, u32, String, String)))
{
    println!("Inserting vote metadata into senate_votes table");
}

pub async fn insert_vote_count_senate(tx: &rusqlite::Transaction<'_>, count_data: (String, u16, u8, u32, String, u16, u32, u32, u32, u32, String, String))
{
    println!("Inserting vote count into senate_votes table");
}

pub async fn insert_vote_members_senate(tx: &rusqlite::Transaction<'_>, member_data: (String, u16, u8, u32, String, u16, Vec<MemberSenate>, Vec<CongressionalMember>))
{
    println!("Inserting vote members into senate_votes table");
}

pub async fn get_senate_vote(
    tx: &rusqlite::Transaction<'_>,
    info_pulled: &(u16, u8, u32, String, u16),
    vote: &RollCallVoteSenate
) -> (u16, u8, u32, String, u16, String, String, String, String, String, String, String, String, String, Vec<DocumentSenate>, Vec<AmendmentSenate>, u32, u32, u32, u32, String, String, Vec<MemberSenate>, Vec<CongressionalMember>) {
    let (congress_number, session_number, rollcall_number, chamber, year) = info_pulled.clone();

    let vote_hash = generate_hash(&serde_json::to_string(vote).unwrap()).await;

    let vote_date = match vote.vote_date.clone() {
        Response::String(date) => date,
        _ => {
            "null".to_string()
        }
    };

    let vote_modify_date = match vote.modify_date.clone() {
        Some(s) => s,
        _ => {
            "null".to_string()
        }
    };

    let vote_question = match vote.question.clone() {
        Response::String(question) => question,
        _ => {
            "null".to_string()
        }
    };

    let vote_question_text = match vote.vote_question_text.clone() {
        Response::String(question_text) => question_text,
        _ => {
            "null".to_string()
        }
    };

    let vote_document_text = match vote.vote_document_text.clone() {
        Response::String(document_text) => document_text,
        _ => {
            "null".to_string()
        }
    };

    let vote_result = match vote.vote_result.clone() {
        Some(s) => s,
        _ => {
            "null".to_string()
        }
    };

    let vote_title = match vote.vote_title.clone() {
        Some(s) => s,
        _ => {
            "null".to_string()
        }
    };

    let majority_requirement = match vote.majority_requirement.clone() {
        Some(s) => s,
        _ => {
            "null".to_string()
        }
    };

    let documents = if let Some(doc) = &vote.document {

        let mut docs = vec![];

        match doc {
            ResponseSpecific::DocumentSenate(doc) => {
                docs.push(*doc.clone());
            }
            ResponseSpecific::DSMap(doc_vector) => {
                for doc in doc_vector {
                    docs.push(doc.clone());
                }
            }
            _ => {
                docs.push(DocumentSenate::default());
            }
        }

        docs
    } else {
        vec![DocumentSenate::default()]
    };

    let amendments = if let Some(amend) = &vote.amendment {

        let mut amends = vec![];

        match amend {
            ResponseSpecific::AmendmentSenate(amend) => {
                amends.push(*amend.clone());
            }
            ResponseSpecific::ASMap(amend_vector) => {
                for amend in amend_vector {
                    amends.push(amend.clone());
                }
            }
            _ => {
                amends.push(AmendmentSenate::default());
            }
        }

        amends
    } else {
        vec![AmendmentSenate::default()]
    };

    let (yay, nay, present, absent) = if let Some(count) = &vote.count {
        let yay = match count.yeas.clone() {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => {
            0
            }
        };

        let nay = match count.nays.clone() {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => {
            0
            }
        };

        let present = if let Some(s) = count.present.clone() {
            match s {
                Response::String(s) => {
                    s.parse::<u32>().unwrap()
                }
                Response::U32(n) => {
                    n
                }
                _ => {
                    0
                }
            }
        } else {
            0
        };

        let absent = if let Some(s) = count.absent.clone() {
            match s {
                Response::String(s) => {
                    s.parse::<u32>().unwrap()
                }
                Response::U32(n) => {
                    n
                }
                _ => {
                    0
                }
            }
        } else {
            0
        };

        (yay, nay, present, absent)
    } else {
        (0, 0, 0, 0)
    };

    let tie_breaker = if let Some(tie) = &vote.tie_breaker {
        let by_whom = match tie.by_whom.clone() {
            Response::String(s) => s,
            _ => {
                "null".to_string()
            }
        };
        let tie_breaker_vote = match tie.tie_breaker_vote.clone() {
            Response::String(s) => s,
            _ => {
                "null".to_string()
            }
        };
        
        (by_whom, tie_breaker_vote)
    } else {
        ("null".to_string(), "null".to_string())
    };

    let members = if let Some(mem) = &vote.members {
        // struct MemberSenate for senate specific filing
        let mut senate_mems = vec![];
        // struct CongressionalMember for general filing
        let mut mems = vec![];

        if let Some(mem_vector) = mem.member.clone() {
            for member in mem_vector {
                let full = match member.member_full.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let last_name = match member.last_name.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let first_name = match member.first_name.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let party = match member.party.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let state = match member.state.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let vote_cast = if let Some(vote) = member.vote_cast.clone() {
                    match vote {
                        ResponseSpecific::VoteCastMap(name_and_paired) => {
                            let name = match name_and_paired.name.clone() {
                                Some(s) => s,
                                _ => {
                                    "null".to_string()
                                }
                            };

                            let paired = match name_and_paired.paired_with.clone() {
                                Some(s) => s,
                                _ => {
                                    "null".to_string()
                                }
                            };

                            VoteCastMap {
                                name: Some(name),
                                paired_with: Some(paired)
                            }
                        }
                        ResponseSpecific::String(s) => {
                            let name = Some(s.clone());
                            let paired_with = Some("null".to_string());

                            VoteCastMap {
                                name,
                                paired_with
                            }
                        }
                        _ => {
                            VoteCastMap {
                                name: Some("null".to_string()),
                                paired_with: Some("null".to_string())
                            }
                        }
                    }
                } else {
                    VoteCastMap {
                        name: Some("null".to_string()),
                        paired_with: Some("null".to_string())
                    }
                };

                let lis_member_id = match member.lis_member_id.clone() {
                    Some(s) => s,
                    _ => {
                        "null".to_string()
                    }
                };

                let hash_input = (last_name.trim().to_lowercase(), party.clone(), state.clone());

                let member_hash = generate_hash(&serde_json::to_string(&hash_input).unwrap()).await;

                let m = MemberSenate {
                    member_full: Some(full),
                    last_name: Some(last_name.clone()),
                    first_name: Some(first_name),
                    party: Some(party.clone()),
                    state: Some(state.clone()),
                    vote_cast: Some(ResponseSpecific::VoteCastMap(vote_cast)),
                    lis_member_id: Some(lis_member_id),
                    extra: Some(std::collections::HashMap::from_iter(vec![
                        ("generated_id".to_string(), serde_json::to_value(member_hash.clone()).unwrap())
                    ]))
                };

                senate_mems.push(m);

                let m = CongressionalMember {
                    last_name,
                    party,
                    state,
                    generated_id: member_hash,
                };

                mems.push(m);
            }
        } else {
            println!("❌ No members found in vote: {}", vote_hash);
            mems.push(CongressionalMember {
                last_name: "null".to_string(),
                party: "null".to_string(),
                state: "null".to_string(),
                generated_id: "null".to_string(),
            });
        }

        (senate_mems, mems)
    } else {
        (vec![], vec![])
    };

    let senate_info = (
        congress_number,
        session_number,
        rollcall_number,
        chamber.clone(),
        year,
        // hash generated from the vote object
        vote_hash.clone(),
        vote_date,
        vote_modify_date,
        vote_question,
        vote_question_text,
        vote_document_text,
        vote_result,
        vote_title,
        majority_requirement,
        // vec of possible documents as json strings
        documents,
        // vec of possible amendments as json strings
        amendments,
        yay,
        nay,
        present,
        absent,
        // name
        tie_breaker.0,
        // paired_with
        tie_breaker.1,
        // vec of members as MemberSenate objects
        members.0,
        // vec of members as CongressionalMember objects
        members.1
    );


    senate_info
}
