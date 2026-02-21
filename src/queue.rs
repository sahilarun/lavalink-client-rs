use crate::types::queue::{ManagerQueueOptions, QueueStoreManager, QueueChangesWatcher, QueueTrack, DefaultStoredQueue, StoredQueue};
use crate::types::track::{Track, UnresolvedTrack};
use crate::utils::ManagerUtils;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use async_trait::async_trait;

pub struct QueueSaver {
    pub store: Arc<dyn QueueStoreManager>,
    pub max_previous_tracks: i32,
}

impl QueueSaver {
    pub fn new(options: &ManagerQueueOptions) -> Self {
        let store = options.queue_store.clone().unwrap_or_else(|| Arc::new(DefaultQueueStore::new()));
        Self {
            store,
            max_previous_tracks: options.max_previous_tracks,
        }
    }

    pub async fn get(&self, guild_id: &str) -> Option<DefaultStoredQueue> {
        let value = self.store.get(guild_id).await?;
        self.store.parse(&value).await
    }

    pub async fn delete(&self, guild_id: &str) -> bool {
        self.store.delete(guild_id).await
    }

    pub async fn set(&self, guild_id: &str, value: &dyn StoredQueue) -> bool {
        let stringified = self.store.stringify(value).await;
        self.store.set(guild_id, &stringified).await
    }

    pub async fn sync(&self, guild_id: &str) -> Option<DefaultStoredQueue> {
        self.get(guild_id).await
    }
}

pub struct DefaultQueueStore {
    data: RwLock<HashMap<String, String>>,
}

impl DefaultQueueStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl QueueStoreManager for DefaultQueueStore {
    async fn get(&self, guild_id: &str) -> Option<String> {
        self.data.read().await.get(guild_id).cloned()
    }

    async fn set(&self, guild_id: &str, value: &str) -> bool {
        self.data.write().await.insert(guild_id.to_string(), value.to_string());
        true
    }

    async fn delete(&self, guild_id: &str) -> bool {
        self.data.write().await.remove(guild_id).is_some()
    }

    async fn stringify(&self, value: &dyn StoredQueue) -> String {
        // Mock stringify for default
        // In reality, serialize `value` to JSON string.
        let default_queue = DefaultStoredQueue {
            current: value.current().cloned(),
            previous: value.previous().to_vec(),
            tracks: value.tracks().to_vec(),
        };
        serde_json::to_string(&default_queue).unwrap_or_default()
    }

    async fn parse(&self, value: &str) -> Option<DefaultStoredQueue> {
        serde_json::from_str(value).ok()
    }
}

#[derive(Clone)]
pub struct Queue {
    pub current: Option<Track>,
    pub previous: Vec<Track>,
    pub tracks: Vec<QueueTrack>,
    pub options: ManagerQueueOptions,
    pub guild_id: String,
    // Using Arc to make Queue cloneable
    pub queue_saver: Option<Arc<QueueSaver>>,
    queue_changes_watcher: Option<Arc<dyn QueueChangesWatcher>>,
}

impl std::fmt::Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("current", &self.current)
            .field("previous", &self.previous)
            .field("tracks", &self.tracks)
            .field("options", &self.options)
            .field("guild_id", &self.guild_id)
            .finish()
    }
}

impl StoredQueue for Queue {
    fn current(&self) -> Option<&Track> {
        self.current.as_ref()
    }
    fn previous(&self) -> &[Track] {
        &self.previous
    }
    fn tracks(&self) -> &[QueueTrack] {
        &self.tracks
    }
}

impl Queue {
    pub fn new(guild_id: String, data: Option<DefaultStoredQueue>, queue_saver: Option<Arc<QueueSaver>>, options: ManagerQueueOptions) -> Self {
        let mut queue = Self {
            current: None,
            previous: vec![],
            tracks: vec![],
            queue_changes_watcher: options.queue_changes_watcher.clone(),
            options: options.clone(),
            guild_id,
            queue_saver,
        };

        if let Some(_qs) = queue.queue_saver.clone() {
            // max previous tracks
        }

        if let Some(d) = data {
            queue.current = d.current;
            queue.previous = d.previous;
            queue.tracks = d.tracks;
        }

        queue
    }

    pub async fn save(&mut self) -> bool {
        if self.previous.len() > self.options.max_previous_tracks as usize {
            self.previous.drain(self.options.max_previous_tracks as usize..);
        }
        if let Some(saver) = &self.queue_saver {
            return saver.set(&self.guild_id, self as &dyn StoredQueue).await;
        }
        false
    }

    pub async fn sync(&mut self, override_queue: bool, dont_sync_current: bool) -> Result<(), String> {
        if let Some(saver) = &self.queue_saver {
            let data = saver.get(&self.guild_id).await;
            if data.is_none() {
                return Err(format!("No data found to sync for guildId: {}", self.guild_id));
            }
            let data = data.unwrap();
            
            if !dont_sync_current && self.current.is_none() && data.current.is_some() {
                self.current = data.current;
            }

            if !data.tracks.is_empty() {
                if override_queue {
                    self.tracks.clear();
                }
                self.tracks.extend(data.tracks);
            }

            if !data.previous.is_empty() {
                if override_queue {
                    self.previous.clear();
                }
                self.previous.extend(data.previous);
            }
            
            self.save().await;
            Ok(())
        } else {
            Err("No QueueSaver setup".to_string())
        }
    }

    pub async fn destroy(&self) -> bool {
        if let Some(saver) = &self.queue_saver {
            return saver.delete(&self.guild_id).await;
        }
        false
    }
    
    pub fn to_json(&mut self) -> DefaultStoredQueue {
        if self.previous.len() > self.options.max_previous_tracks as usize {
            self.previous.drain(self.options.max_previous_tracks as usize..);
        }
        DefaultStoredQueue {
            current: self.current.clone(),
            previous: self.previous.clone(),
            tracks: self.tracks.clone(),
        }
    }

    pub fn total_duration(&self) -> i64 {
        let mut total = self.current.as_ref().map(|c| c.info.duration).unwrap_or(0);
        for t in &self.tracks {
            match t {
                QueueTrack::Resolved(tr) => { total += tr.info.duration; }
                QueueTrack::Unresolved(_) => { /* Unresolved tracks don't have known accurate duration in this array sum typically */ }
            }
        }
        total
    }

    pub async fn shuffle(&mut self) -> usize {
        let _old_stored = if self.queue_changes_watcher.is_some() { Some(self.to_json()) } else { None };

        if self.tracks.len() <= 1 {
            return self.tracks.len();
        }

        if self.tracks.len() == 2 {
            self.tracks.swap(0, 1);
        } else {
            let mut rng = rand::rng();
            self.tracks.shuffle(&mut rng);
        }

        if let Some(_watcher) = &self.queue_changes_watcher {
            // log shuffled
        }

        self.save().await;
        self.tracks.len()
    }

    pub async fn add(&mut self, mut new_tracks: Vec<QueueTrack>, index: Option<usize>) -> usize {
        let _old_stored = if self.queue_changes_watcher.is_some() { Some(self.to_json()) } else { None };
        
        if let Some(mut idx) = index {
            if idx > self.tracks.len() {
                idx = self.tracks.len();
            }
            let tail = self.tracks.split_off(idx);
            self.tracks.append(&mut new_tracks);
            self.tracks.extend(tail);
        } else {
            self.tracks.append(&mut new_tracks);
        }

        if let Some(_watcher) = &self.queue_changes_watcher {
            // Call watcher tracks_add
        }

        self.save().await;
        self.tracks.len()
    }

    pub async fn splice(&mut self, index: usize, amount: usize, insert_tracks: Option<Vec<QueueTrack>>) -> Vec<QueueTrack> {
        let _old_stored = if self.queue_changes_watcher.is_some() { Some(self.to_json()) } else { None };
        
        if self.tracks.is_empty() {
            if let Some(tracks) = insert_tracks {
                self.add(tracks, None).await;
            }
            return vec![];
        }

        let end_idx = (index + amount).min(self.tracks.len());
        let mut removed = vec![];
        for _ in index..end_idx {
            if index < self.tracks.len() {
                removed.push(self.tracks.remove(index));
            }
        }

        if let Some(mut tracks) = insert_tracks {
            let tail = self.tracks.split_off(index.min(self.tracks.len()));
            self.tracks.append(&mut tracks);
            self.tracks.extend(tail);
        }
        
        if let Some(_watcher) = &self.queue_changes_watcher {
            // Call watcher tracks removed / added
        }

        self.save().await;
        removed
    }

    pub async fn remove(&mut self, index: usize) -> Option<QueueTrack> {
        if index < self.tracks.len() {
            let _old_stored = if self.queue_changes_watcher.is_some() { Some(self.to_json()) } else { None };
            let removed = self.tracks.remove(index);
            self.save().await;
            Some(removed)
        } else {
            None
        }
    }

    pub async fn shift_previous(&mut self) -> Option<Track> {
        if !self.previous.is_empty() {
            let removed = self.previous.remove(0);
            self.save().await;
            Some(removed)
        } else {
            None
        }
    }

    pub fn get_tracks(&self, start: usize, end: Option<usize>) -> Vec<QueueTrack> {
        let end_idx = end.unwrap_or(self.tracks.len()).min(self.tracks.len());
        if start >= end_idx {
            return vec![];
        }
        self.tracks[start..end_idx].to_vec()
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }
}
