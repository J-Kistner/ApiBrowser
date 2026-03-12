use crate::api::models::*;
use crate::stats::TeamStats;

#[derive(Debug, Clone)]
pub enum View {
   TeamList {
      selected_index: usize,
      search_query: String,
      searching: bool,
   },
   TeamDetail {
      team_key: String,
      selected_match_index: usize,
   },
   MatchDetail {
      match_key: String,
      selected_team_index: usize,
   },
   CompetitionBrowser {
      selected_index: usize,
   },
}

pub struct AppData {
   pub event: Event,
   pub teams: Vec<Team>,
   #[allow(dead_code)]
   pub rankings: Vec<Ranking>,
   pub matches: Vec<Match>,
   #[allow(dead_code)]
   pub oprs: EventOPRs,
   pub team_stats: Vec<TeamStats>,
}

pub struct App {
   pub data: AppData,
   pub current_view: View,
   pub view_history: Vec<View>,
   pub test_mode: bool,
}

impl App {
   pub fn new(
      event: Event,
      teams: Vec<Team>,
      rankings: Vec<Ranking>,
      matches: Vec<Match>,
      oprs: EventOPRs,
      test_mode: bool,
   ) -> Self {
      let team_stats: Vec<TeamStats> = if rankings.is_empty() {
         // No rankings yet - create stats for all teams sorted by team number
         let mut stats: Vec<TeamStats> = teams
            .iter()
            .map(|team| {
               crate::stats::create_unranked_team_stats(&team.key, &teams, &matches, &oprs.oprs)
            })
            .collect();

         // Sort by team number
         stats.sort_by_key(|s| s.team_number);
         stats
      } else {
         // We have rankings - create stats for ranked teams
         let mut ranked_stats: Vec<TeamStats> = rankings
            .iter()
            .filter_map(|ranking| {
               crate::stats::calculate_team_stats(
                  &ranking.team_key,
                  &teams,
                  &rankings,
                  &matches,
                  &oprs.oprs,
               )
            })
            .collect();

         // Sort ranked teams by rank
         ranked_stats.sort_by_key(|s| s.rank);

         // Find teams that don't have rankings yet
         let ranked_team_keys: std::collections::HashSet<String> =
            rankings.iter().map(|r| r.team_key.clone()).collect();

         let mut unranked_stats: Vec<TeamStats> = teams
            .iter()
            .filter(|team| !ranked_team_keys.contains(&team.key))
            .map(|team| {
               crate::stats::create_unranked_team_stats(&team.key, &teams, &matches, &oprs.oprs)
            })
            .collect();

         // Sort unranked teams by team number
         unranked_stats.sort_by_key(|s| s.team_number);

         // Combine: ranked first, then unranked
         ranked_stats.extend(unranked_stats);
         ranked_stats
      };

      let data = AppData {
         event,
         teams,
         rankings,
         matches,
         oprs,
         team_stats,
      };

      App {
         data,
         current_view: View::TeamList {
            selected_index: 0,
            search_query: String::new(),
            searching: false,
         },
         view_history: Vec::new(),
         test_mode,
      }
   }

   pub fn navigate_to(&mut self, view: View) {
      let old_view = self.current_view.clone();
      self.view_history.push(old_view);
      self.current_view = view;
   }

   pub fn navigate_back(&mut self) {
      if let Some(prev_view) = self.view_history.pop() {
         self.current_view = prev_view;
      }
   }

   pub fn get_team_matches(&self, team_key: &str) -> Vec<Match> {
      let mut matches: Vec<Match> = self
         .data
         .matches
         .iter()
         .filter(|m| {
            m.alliances.blue.team_keys.contains(&team_key.to_string())
               || m.alliances.red.team_keys.contains(&team_key.to_string())
         })
         .cloned()
         .collect();

      // Sort by time (chronological)
      matches.sort_by_key(|m| {
         m.actual_time
            .or(m.predicted_time)
            .or(m.time)
            .unwrap_or(i64::MAX)
      });

      matches
   }

   pub fn get_match(&self, match_key: &str) -> Option<&Match> {
      self.data.matches.iter().find(|m| m.key == match_key)
   }

   pub fn get_team_stats(&self, team_key: &str) -> Option<&TeamStats> {
      self.data.team_stats.iter().find(|s| s.team_key == team_key)
   }

   pub fn get_team(&self, team_key: &str) -> Option<&Team> {
      self.data.teams.iter().find(|t| t.key == team_key)
   }
}
