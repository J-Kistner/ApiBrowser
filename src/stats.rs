use crate::api::models::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct TeamStats {
   pub team_key: String,
   pub team_number: i32,
   pub team_name: String,
   pub rank: i32,
   pub total_teams: i32,
   pub record: String,
   pub matches_played: usize,
   pub rp_average: f32,
   pub auto_average: f32,
   pub teleop_average: f32,
   pub endgame_average: f32,
   pub opr: f32,
   #[allow(dead_code)]
   pub dpr: f32,
   #[allow(dead_code)]
   pub ccwm: f32,
}

pub fn calculate_team_stats(
   team_key: &str,
   teams: &[Team],
   rankings: &[Ranking],
   matches: &[Match],
   oprs: &HashMap<String, f32>,
) -> Option<TeamStats> {
   // Find team info
   let team = teams.iter().find(|t| t.key == team_key)?;

   // Find ranking info
   let ranking = rankings.iter().find(|r| r.team_key == team_key)?;

   // Get completed matches for this team
   let completed_matches: Vec<_> = matches
      .iter()
      .filter(|m| m.actual_time.is_some())
      .filter(|m| team_is_in_match(team_key, m))
      .collect();

   let matches_played = completed_matches.len();

   // Calculate averages
   let (auto_total, teleop_total, endgame_total) =
      completed_matches
         .iter()
         .fold((0.0, 0.0, 0.0), |(auto, teleop, endgame), m| {
            let (a, t, e) = extract_points_for_team(team_key, m);
            (auto + a, teleop + t, endgame + e)
         });

   let auto_average = if matches_played > 0 {
      auto_total / matches_played as f32
   } else {
      0.0
   };
   let teleop_average = if matches_played > 0 {
      teleop_total / matches_played as f32
   } else {
      0.0
   };
   let endgame_average = if matches_played > 0 {
      endgame_total / matches_played as f32
   } else {
      0.0
   };

   // Calculate RP average from ranking
   let rp_average = if matches_played > 0 {
      ranking.sort_orders.first().copied().unwrap_or(0.0)
   } else {
      0.0
   };

   // Get OPR/DPR/CCWM
   let opr = oprs.get(team_key).copied().unwrap_or(0.0);
   let dpr = 0.0; // Would need separate DPR map
   let ccwm = 0.0; // Would need separate CCWM map

   Some(TeamStats {
      team_key: team_key.to_string(),
      team_number: team.team_number,
      team_name: team.nickname.clone(),
      rank: ranking.rank,
      total_teams: rankings.len() as i32,
      record: format!(
         "{}-{}-{}",
         ranking.record.wins, ranking.record.losses, ranking.record.ties
      ),
      matches_played,
      rp_average,
      auto_average,
      teleop_average,
      endgame_average,
      opr,
      dpr,
      ccwm,
   })
}

pub fn create_unranked_team_stats(
   team_key: &str,
   teams: &[Team],
   matches: &[Match],
   oprs: &HashMap<String, f32>,
) -> TeamStats {
   // Find team info
   let team = teams.iter().find(|t| t.key == team_key);

   // Get completed matches for this team
   let completed_matches: Vec<_> = matches
      .iter()
      .filter(|m| m.actual_time.is_some())
      .filter(|m| team_is_in_match(team_key, m))
      .collect();

   let matches_played = completed_matches.len();

   // Calculate averages
   let (auto_total, teleop_total, endgame_total) =
      completed_matches
         .iter()
         .fold((0.0, 0.0, 0.0), |(auto, teleop, endgame), m| {
            let (a, t, e) = extract_points_for_team(team_key, m);
            (auto + a, teleop + t, endgame + e)
         });

   let auto_average = if matches_played > 0 {
      auto_total / matches_played as f32
   } else {
      0.0
   };
   let teleop_average = if matches_played > 0 {
      teleop_total / matches_played as f32
   } else {
      0.0
   };
   let endgame_average = if matches_played > 0 {
      endgame_total / matches_played as f32
   } else {
      0.0
   };

   // Get OPR
   let opr = oprs.get(team_key).copied().unwrap_or(0.0);

   if let Some(team) = team {
      TeamStats {
         team_key: team_key.to_string(),
         team_number: team.team_number,
         team_name: team.nickname.clone(),
         rank: 0, // No rank yet
         total_teams: teams.len() as i32,
         record: "0-0-0".to_string(), // No record yet
         matches_played,
         rp_average: 0.0,
         auto_average,
         teleop_average,
         endgame_average,
         opr,
         dpr: 0.0,
         ccwm: 0.0,
      }
   } else {
      TeamStats::default()
   }
}

fn team_is_in_match(team_key: &str, match_data: &Match) -> bool {
   match_data
      .alliances
      .blue
      .team_keys
      .contains(&team_key.to_string())
      || match_data
         .alliances
         .red
         .team_keys
         .contains(&team_key.to_string())
}

fn extract_points_for_team(team_key: &str, match_data: &Match) -> (f32, f32, f32) {
   // Determine which alliance the team is on
   let alliance = if match_data
      .alliances
      .blue
      .team_keys
      .contains(&team_key.to_string())
   {
      "blue"
   } else {
      "red"
   };

   // Extract score breakdown if available
   if let Some(breakdown) = &match_data.score_breakdown {
      let alliance_breakdown = if alliance == "blue" {
         &breakdown.blue
      } else {
         &breakdown.red
      };

      // Try to extract 2025/2026 specific fields
      // This is a simplified version - actual fields depend on the game
      let auto = alliance_breakdown
         .get("autoPoints")
         .and_then(|v| v.as_f64())
         .unwrap_or(0.0) as f32;
      let teleop = alliance_breakdown
         .get("teleopPoints")
         .and_then(|v| v.as_f64())
         .unwrap_or(0.0) as f32;
      let endgame = alliance_breakdown
         .get("endgamePoints")
         .and_then(|v| v.as_f64())
         .unwrap_or(0.0) as f32;

      return (auto, teleop, endgame);
   }

   // No breakdown available
   (0.0, 0.0, 0.0)
}
