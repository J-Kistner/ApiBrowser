use crate::api::client::ApiClient;
use crate::api::models::*;
use crate::cache::Cache;
use anyhow::Result;

pub struct ApiEndpoints<'a> {
   client: &'a ApiClient,
   cache: &'a Cache,
   event_key: String,
}

impl<'a> ApiEndpoints<'a> {
   pub fn new(client: &'a ApiClient, cache: &'a Cache, event_key: String) -> Self {
      ApiEndpoints {
         client,
         cache,
         event_key,
      }
   }

   pub fn get_event(&self) -> Result<Event> {
      let cache_key = "event";
      let endpoint = format!("/event/{}", self.event_key);

      // Try to get from cache with ETag
      if let Some((cached_data, etag)) = self.cache.get::<Event>(cache_key) {
         // Try conditional request
         match self.client.get::<Event>(&endpoint, etag.as_deref()) {
            Ok((data, new_etag)) => {
               // Got new data, update cache
               self.cache.set(cache_key, &data, new_etag)?;
               return Ok(data);
            }
            Err(_) => {
               // 304 or error, use cached data
               return Ok(cached_data);
            }
         }
      }

      // No cache, fetch fresh
      let (data, etag) = self.client.get::<Event>(&endpoint, None)?;
      self.cache.set(cache_key, &data, etag)?;
      Ok(data)
   }

   pub fn get_teams(&self) -> Result<Vec<Team>> {
      let cache_key = "teams";
      let endpoint = format!("/event/{}/teams", self.event_key);

      if let Some((cached_data, etag)) = self.cache.get::<Vec<Team>>(cache_key) {
         match self.client.get::<Vec<Team>>(&endpoint, etag.as_deref()) {
            Ok((data, new_etag)) => {
               self.cache.set(cache_key, &data, new_etag)?;
               return Ok(data);
            }
            Err(_) => return Ok(cached_data),
         }
      }

      let (data, etag) = self.client.get::<Vec<Team>>(&endpoint, None)?;
      self.cache.set(cache_key, &data, etag)?;
      Ok(data)
   }

   pub fn get_rankings(&self) -> Result<Vec<Ranking>> {
      let cache_key = "rankings";
      let endpoint = format!("/event/{}/rankings", self.event_key);

      if let Some((cached_data, etag)) = self.cache.get::<serde_json::Value>(cache_key) {
         match self
            .client
            .get::<serde_json::Value>(&endpoint, etag.as_deref())
         {
            Ok((data, new_etag)) => {
               self.cache.set(cache_key, &data, new_etag)?;
               return extract_rankings(data);
            }
            Err(_) => return extract_rankings(cached_data),
         }
      }

      let (data, etag) = self.client.get::<serde_json::Value>(&endpoint, None)?;
      self.cache.set(cache_key, &data, etag)?;
      extract_rankings(data)
   }

   pub fn get_matches(&self) -> Result<Vec<Match>> {
      let cache_key = "matches";
      let endpoint = format!("/event/{}/matches", self.event_key);

      if let Some((cached_data, etag)) = self.cache.get::<Vec<Match>>(cache_key) {
         match self.client.get::<Vec<Match>>(&endpoint, etag.as_deref()) {
            Ok((data, new_etag)) => {
               self.cache.set(cache_key, &data, new_etag)?;
               return Ok(data);
            }
            Err(_) => return Ok(cached_data),
         }
      }

      let (data, etag) = self.client.get::<Vec<Match>>(&endpoint, None)?;
      self.cache.set(cache_key, &data, etag)?;
      Ok(data)
   }

   pub fn get_oprs(&self) -> Result<EventOPRs> {
      let cache_key = "oprs";
      let endpoint = format!("/event/{}/oprs", self.event_key);

      if let Some((cached_data, etag)) = self.cache.get::<EventOPRs>(cache_key) {
         match self.client.get::<EventOPRs>(&endpoint, etag.as_deref()) {
            Ok((data, new_etag)) => {
               self.cache.set(cache_key, &data, new_etag)?;
               return Ok(data);
            }
            Err(_) => return Ok(cached_data),
         }
      }

      let (data, etag) = self.client.get::<EventOPRs>(&endpoint, None)?;
      self.cache.set(cache_key, &data, etag)?;
      Ok(data)
   }
}

fn extract_rankings(value: serde_json::Value) -> Result<Vec<Ranking>> {
   // TBA rankings endpoint returns: { "rankings": [...] }
   if let Some(rankings_array) = value.get("rankings") {
      let rankings: Vec<Ranking> = serde_json::from_value(rankings_array.clone())?;
      Ok(rankings)
   } else {
      Ok(Vec::new())
   }
}
