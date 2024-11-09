use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Serde generic type
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(untagged)]
pub enum Response {
    Null,
    #[default]
    None,
    U32(u32),
    String(String),
    Map(HashMap<String, Response>),
    List(Vec<Response>),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(untagged)]
pub enum ResponseSpecific {
    #[default]
    Null,
    None,
    String(String),
    VoteCastMap(VoteCastMap),
    DocumentSenate(Box<DocumentSenate>),
    AmendmentSenate(Box<AmendmentSenate>),
    DSMap(Vec<DocumentSenate>),
    ASMap(Vec<AmendmentSenate>),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Response::String(s) => write!(f, "{}", s),
            Response::Null => write!(f, "null"),
            Response::None => write!(f, "none"),
            Response::U32(u) => write!(f, "{}", u),
            Response::Map(m) => {
                write!(f, "{{")?;
                for (k, v) in m.iter() {
                    write!(f, "{}: {}, ", k, v)?;
                }
                write!(f, "}}")
            }
            Response::List(l) => {
                write!(f, "[")?;
                for v in l.iter() {
                    write!(f, "{}, ", v)?;
                }
                write!(f, "]")
            }
        }
    }
}



impl std::fmt::Display for ResponseSpecific {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResponseSpecific::Null => write!(f, "null"),
            ResponseSpecific::None => write!(f, "none"),
            ResponseSpecific::String(s) => write!(f, "{}", s),
            ResponseSpecific::VoteCastMap(v) => {
                write!(f, "{{")?;
                if let Some(content) = &v.name {
                    write!(f, "content: {}, ", content)?;
                }
                if let Some(pair) = &v.name {
                    write!(f, "pair: {}, ", pair)?;
                }
                write!(f, "}}")
            }
            ResponseSpecific::DocumentSenate(d) => {
                write!(f, "{{")?;
                    write!(f, "document_congress: {}, ", d.document_congress)?;
                    write!(f, "document_type: {}, ", d.document_type)?;
                    write!(f, "document_number: {}, ", d.document_number)?;
                    write!(f, "document_name: {}, ", d.document_name)?;
                    write!(f, "document_title: {}, ", d.document_title)?;
                    write!(f, "document_short_title: {}, ", d.document_short_title.clone().unwrap_or("null".to_string()))?;
                write!(f, "}}")
            }
            ResponseSpecific::AmendmentSenate(a) => {
                write!(f, "{{")?;
                    write!(f, "amendment_number: {}, ", a.amendment_number.clone().unwrap_or("null".to_string()))?;
                    write!(f, "amendment_to_amendment_number: {}, ", a.amendment_to_amendment_number.clone().unwrap_or(Response::Null))?;
                    write!(f, "amendment_to_amendment_to_amendment_number: {}, ", a.amendment_to_amendment_to_amendment_number.clone().unwrap_or(Response::Null))?;
                    write!(f, "amendment_to_document_number: {}, ", a.amendment_to_document_number.clone().unwrap_or(Response::Null))?;
                    write!(f, "amendment_to_document_short_title: {}, ", a.amendment_to_document_short_title.clone().unwrap_or(Response::Null))?;
                    write!(f, "amendment_purpose: {}, ", a.amendment_purpose.clone().unwrap_or("null".to_string()))?;
                write!(f, "}}")
            }
            ResponseSpecific::DSMap(m) => {
                write!(f, "[")?;
                for v in m.iter() {
                    write!(f, "{}, ", v)?;
                }
                write!(f, "]")
            }
            ResponseSpecific::ASMap(m) => {
                write!(f, "{{")?;
                for v in m.iter() {
                    write!(f, "{}, ", v)?;
                }
                write!(f, "}}")
            }
        }
    }
}



// Struct for holding either HouseFile or SenateFile
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VoteFile {
    HouseFile(HouseFile),
    SenateFile(SenateFile),
}

// Unified Struct for Database
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UnifiedVote {
    pub congress_number: u16,
    pub chamber: String,
    pub session_number: u8,
    pub rollcall_number: u32,
    pub vote_date: String,
    pub vote_question: String,
    pub vote_result: String,
    pub legislator_id: String,
    pub legislator_name: String,
    pub party: String,
    pub state: String,
    pub vote_cast: String,
}

// Combined Data Struct
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CombinedData {
    pub chambers: HashMap<String, ChamberData>, // "house" or "senate"
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ChamberData {
    pub congresses: HashMap<u16, CongressData>, // e.g., 116, 117
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CongressData {
    pub sessions: HashMap<u8, SessionData>, // e.g., 1, 2
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SessionData {
    pub rollcalls: Vec<RollCallData>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RollCallData {
    pub rollcall_number: u32,
    pub vote_date: String,
    pub vote_question: String,
    pub vote_result: String,
    pub vote_casts: Vec<UnifiedVote>,
}


// House Structs
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct HouseFile {
    #[serde(rename = "rollcall-vote")]
    pub rollcall_vote: RollCallVoteHouse,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RollCallVoteHouse {
    #[serde(rename = "vote-metadata")]
    pub vote_metadata: VoteMetadataHouse,
    #[serde(rename = "vote-data")]
    pub vote_data: Option<VoteDataHouse>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VoteMetadataHouse {
    pub majority: Option<String>,
    pub congress: String,
    pub session: String,
    pub chamber: Option<String>,
    pub committee: Option<String>,
    #[serde(rename = "rollcall-num")]
    pub rollcall_num: Response,
    #[serde(rename = "legis-num")]
    pub legis_num: Option<String>,
    #[serde(rename = "vote-question")]
    pub vote_question: Response,
    #[serde(rename = "vote-type")]
    pub vote_type: Response,
    #[serde(rename = "vote-result")]
    pub vote_result: Response,
    #[serde(rename = "action-date")]
    pub action_date: Response,
    #[serde(rename = "action-time")]
    pub action_time: Option<ActionTimeHouse>,
    #[serde(rename = "vote-desc")]
    pub vote_desc: Response,
    #[serde(rename = "vote-totals")]
    pub vote_totals: Option<VoteTotalsHouse>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ActionTimeHouse {
    pub content: Option<String>,
    #[serde(rename = "time-etz")]
    pub time_etz: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VoteTotalsHouse {
    #[serde(rename = "totals-by-party-header")]
    pub totals_by_party_header: Option<TotalsByPartyHeaderHouse>,
    #[serde(rename = "totals-by-party")]
    pub totals_by_party: Option<Vec<TotalsByPartyHouse>>,
    #[serde(rename = "totals-by-vote")]
    pub totals_by_vote: Option<TotalsByVoteHouse>,
    #[serde(rename = "totals-by-candidate")]
    pub totals_by_candidate: Option<Vec<TotalsByCandidateHouse>>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TotalsByCandidateHouse {
    pub candidate: Option<String>,
    #[serde(rename = "candidate-total")]
    pub candidate_total: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TotalsByPartyHeaderHouse {
    #[serde(rename = "party-header")]
    pub party_header: Option<String>,
    #[serde(rename = "yea-header")]
    pub yea_header: Option<String>,
    #[serde(rename = "nay-header")]
    pub nay_header: Option<String>,
    #[serde(rename = "present-header")]
    pub present_header: Option<String>,
    #[serde(rename = "not-voting-header")]
    pub not_voting_header: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TotalsByPartyHouse {
    pub party: Option<String>,
    #[serde(rename = "yea-total")]
    pub yea_total: Option<String>,
    #[serde(rename = "nay-total")]
    pub nay_total: Option<String>,
    #[serde(rename = "present-total")]
    pub present_total: Option<String>,
    #[serde(rename = "not-voting-total")]
    pub not_voting_total: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TotalsByVoteHouse {
    #[serde(rename = "total-stub")]
    pub total_stub: Option<String>,
    #[serde(rename = "yea-total")]
    pub yea_total: Option<String>,
    #[serde(rename = "nay-total")]
    pub nay_total: Option<String>,
    #[serde(rename = "present-total")]
    pub present_total: Option<String>,
    #[serde(rename = "not-voting-total")]
    pub not_voting_total: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VoteDataHouse {
    #[serde(rename = "recorded-vote")]
    pub recorded_vote: Option<Vec<RecordedVoteHouse>>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RecordedVoteHouse {
    pub legislator: Option<LegislatorHouse>,
    pub vote: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LegislatorHouse {
    pub content: Option<String>,
    #[serde(rename = "name-id")]
    pub name_id: Option<String>,
    #[serde(rename = "sort-field")]
    pub sort_field: Option<String>,
    #[serde(rename = "unaccented-name")]
    pub unaccented_name: Option<String>,
    pub party: Option<String>,
    pub state: Option<String>,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

// Senate Structs
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SenateFile {
    #[serde(rename = "roll_call_vote")]
    pub roll_call_vote: RollCallVoteSenate,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RollCallVoteSenate {
    pub congress: String,
    pub session: String,
    #[serde(rename = "congress_year")]
    pub congress_year: Option<String>,
    #[serde(rename = "vote_number")]
    pub vote_number: Option<String>,
    #[serde(rename = "vote_date")]
    pub vote_date: Response,
    #[serde(rename = "modify_date")]
    pub modify_date: Option<String>,
    #[serde(rename = "vote_question_text")]
    pub vote_question_text: Response,
    #[serde(rename = "vote_document_text")]
    pub vote_document_text: Response,
    #[serde(rename = "vote_result_text")]
    pub vote_result_text: Response,
    pub question: Response,
    #[serde(rename = "vote_title")]
    pub vote_title: Option<String>,
    #[serde(rename = "majority_requirement")]
    pub majority_requirement: Option<String>,
    #[serde(rename = "vote_result")]
    pub vote_result: Option<String>,
    pub document: Option<ResponseSpecific>, // DocumentSenate expected
    pub amendment: Option<ResponseSpecific>, // AmendmentSenate expected
    pub count: Option<CountSenate>,
    #[serde(rename = "tie_breaker")]
    pub tie_breaker: Option<TieBreakerSenate>,
    pub members: Option<MembersSenate>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DocumentSenate {
    #[serde(rename = "document_congress")]
    pub document_congress: Response,
    #[serde(rename = "document_type")]
    pub document_type: Response,
    #[serde(rename = "document_number")]
    pub document_number: Response,
    #[serde(rename = "document_name")]
    pub document_name: Response,
    #[serde(rename = "document_title")]
    pub document_title: Response,
    #[serde(rename = "document_short_title")]
    pub document_short_title: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for DocumentSenate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "document_congress: {}, ", self.document_congress)?;
        write!(f, "document_type: {}, ", self.document_type)?;
        write!(f, "document_number: {}, ", self.document_number)?;
        write!(f, "document_name: {}, ", self.document_name)?;
        write!(f, "document_title: {}, ", self.document_title)?;
        write!(f, "document_short_title: {}, ", self.document_short_title.clone().unwrap_or("null".to_string()))?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AmendmentSenate {
    #[serde(rename = "amendment_number")]
    pub amendment_number: Option<String>,
    #[serde(rename = "amendment_to_amendment_number")]
    pub amendment_to_amendment_number: Option<Response>,
    #[serde(rename = "amendment_to_amendment_to_amendment_number")]
    pub amendment_to_amendment_to_amendment_number: Option<Response>,
    #[serde(rename = "amendment_to_document_number")]
    pub amendment_to_document_number: Option<Response>,
    #[serde(rename = "amendment_to_document_short_title")]
    pub amendment_to_document_short_title: Option<Response>,
    #[serde(rename = "amendment_purpose")]
    pub amendment_purpose: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for AmendmentSenate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "amendment_number: {}, ", self.amendment_number.clone().unwrap_or("null".to_string()))?;
        write!(f, "amendment_to_amendment_number: {}, ", self.amendment_to_amendment_number.clone().unwrap_or(Response::Null))?;
        write!(f, "amendment_to_amendment_to_amendment_number: {}, ", self.amendment_to_amendment_to_amendment_number.clone().unwrap_or(Response::Null))?;
        write!(f, "amendment_to_document_number: {}, ", self.amendment_to_document_number.clone().unwrap_or(Response::Null))?;
        write!(f, "amendment_to_document_short_title: {}, ", self.amendment_to_document_short_title.clone().unwrap_or(Response::Null))?;
        write!(f, "amendment_purpose: {}, ", self.amendment_purpose.clone().unwrap_or("null".to_string()))?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CountSenate {
    pub yeas: Option<String>,
    pub nays: Option<String>,
    pub present: Option<Response>,
    pub absent: Option<Response>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TieBreakerSenate {
    #[serde(rename = "by_whom")]
    pub by_whom: Response,
    #[serde(rename = "tie_breaker_vote")]
    pub tie_breaker_vote: Response, 
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MembersSenate {
    #[serde(rename = "member")]
    pub member: Option<Vec<MemberSenate>>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MemberSenate {
    #[serde(rename = "member_full")]
    pub member_full: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    pub party: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "vote_cast")]
    pub vote_cast: Option<ResponseSpecific>, // VoteCastMap expected
    #[serde(rename = "lis_member_id")]
    pub lis_member_id: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

// Struct for VoteCast option
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VoteCastMap {
    #[serde(rename = "content")]
    pub name: Option<String>,
    #[serde(rename = "pair")]
    pub paired_with: Option<String>,
}

// Struct for General Congressional Member
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CongressionalMember {
    pub last_name: String,
    pub party: String,
    pub state: String,
    // this should be seperate to the unique id applied by a database
    pub generated_id: String,
}


#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SenateInfo {
    pub congress_number: u16,
    pub session_number: u8,
    pub rollcall_number: u32,
    pub chamber: String,
    pub year: u16,
    pub vote_hash: String,
    pub vote_date: String,
    pub vote_modify_date: String,
    pub vote_question: String,
    pub vote_question_text: String,
    pub vote_document_text: String,
    pub vote_result: String,
    pub vote_title: String,
    pub majority_requirement: String,
    pub documents: Vec<DocumentSenate>,
    pub amendments: Vec<AmendmentSenate>,
    pub yay: u32,
    pub nay: u32,
    pub present: u32,
    pub absent: u32,
    pub tie_breaker_name: String,
    pub tie_breaker_paired: String,
    pub members_as_is: Vec<MemberSenate>,
    pub members_to_gen: Vec<CongressionalMember>
}