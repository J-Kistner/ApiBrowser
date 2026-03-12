use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
   pub key: String,
   pub name: String,
   pub event_code: String,
   pub event_type: i32,
   pub start_date: String,
   pub end_date: String,
   pub year: i32,
   pub city: Option<String>,
   pub state_prov: Option<String>,
   pub country: Option<String>,
   pub timezone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Team {
   pub key: String,
   pub team_number: i32,
   pub nickname: String,
   pub name: String,
   pub city: Option<String>,
   pub state_prov: Option<String>,
   pub country: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ranking {
   pub rank: i32,
   pub team_key: String,
   pub sort_orders: Vec<f32>,
   pub record: WLTRecord,
   pub matches_played: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WLTRecord {
   pub wins: i32,
   pub losses: i32,
   pub ties: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Match {
   pub key: String,
   pub comp_level: String,
   pub set_number: i32,
   pub match_number: i32,
   pub alliances: MatchAlliances,
   pub winning_alliance: Option<String>,
   pub time: Option<i64>,
   pub actual_time: Option<i64>,
   pub predicted_time: Option<i64>,
   pub score_breakdown: Option<ScoreBreakdown>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatchAlliances {
   pub blue: Alliance,
   pub red: Alliance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alliance {
   pub score: i32,
   pub team_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScoreBreakdown {
   pub blue: HashMap<String, Value>,
   pub red: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct EventOPRs {
   pub oprs: HashMap<String, f32>,
   pub dprs: HashMap<String, f32>,
   pub ccwms: HashMap<String, f32>,
}

impl Default for EventOPRs {
   fn default() -> Self {
      EventOPRs {
         oprs: HashMap::new(),
         dprs: HashMap::new(),
         ccwms: HashMap::new(),
      }
   }
}

// Helper structs for UI
#[derive(Debug, Clone)]
pub enum MatchStatus {
   Completed {
      blue_score: i32,
      red_score: i32,
      winner: Option<String>,
   },
   Scheduled {
      time: Option<i64>,
   },
}

impl Match {
   pub fn status(&self) -> MatchStatus {
      if self.actual_time.is_some() {
         MatchStatus::Completed {
            blue_score: self.alliances.blue.score,
            red_score: self.alliances.red.score,
            winner: self.winning_alliance.clone(),
         }
      } else {
         MatchStatus::Scheduled {
            time: self.predicted_time.or(self.time),
         }
      }
   }

   pub fn display_name(&self) -> String {
      match self.comp_level.as_str() {
         "qm" => format!("Quals {}", self.match_number),
         "ef" => format!(
            "Eighthfinals {} Match {}",
            self.set_number, self.match_number
         ),
         "qf" => format!(
            "Quarterfinals {} Match {}",
            self.set_number, self.match_number
         ),
         "sf" => format!("Semifinals {} Match {}", self.set_number, self.match_number),
         "f" => format!("Finals Match {}", self.match_number),
         _ => format!("Match {}", self.match_number),
      }
   }
}
