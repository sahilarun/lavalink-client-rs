use super::track::{Track, UnresolvedTrack};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueueTrack {
    Resolved(Track),
    Unresolved(UnresolvedTrack),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultStoredQueue {
    pub current: Option<Track>,
    pub previous: Vec<Track>,
    pub tracks: Vec<QueueTrack>,
}

pub trait StoredQueue: Send + Sync {
    fn current(&self) -> Option<&Track>;
    fn previous(&self) -> &[Track];
    fn tracks(&self) -> &[QueueTrack];
}

#[async_trait]
pub trait QueueStoreManager: Send + Sync {
    async fn get(&self, guild_id: &str) -> Option<String>;
    async fn set(&self, guild_id: &str, value: &str) -> bool;
    async fn delete(&self, guild_id: &str) -> bool;
    async fn stringify(&self, value: &dyn StoredQueue) -> String;
    // We can use the default stored queue directly or a string
    async fn parse(&self, value: &str) -> Option<DefaultStoredQueue>;
}

pub trait QueueChangesWatcher: Send + Sync {
    fn tracks_add(
        &self,
        guild_id: &str,
        tracks: &[QueueTrack],
        position: usize,
        old_stored_queue: &dyn StoredQueue,
        new_stored_queue: &dyn StoredQueue,
    );
    fn tracks_removed(
        &self,
        guild_id: &str,
        tracks: &[QueueTrack],
        position: &[usize],
        old_stored_queue: &dyn StoredQueue,
        new_stored_queue: &dyn StoredQueue,
    );
    fn shuffled(
        &self,
        guild_id: &str,
        old_stored_queue: &dyn StoredQueue,
        new_stored_queue: &dyn StoredQueue,
    );
}

use std::sync::Arc;

pub struct ManagerQueueOptions {
    pub max_previous_tracks: i32,
    pub queue_store: Option<Arc<dyn QueueStoreManager>>,
    pub queue_changes_watcher: Option<Arc<dyn QueueChangesWatcher>>,
}

impl Clone for ManagerQueueOptions {
    fn clone(&self) -> Self {
        Self {
            max_previous_tracks: self.max_previous_tracks,
            queue_store: self.queue_store.clone(),
            queue_changes_watcher: self.queue_changes_watcher.clone(),
        }
    }
}

impl std::fmt::Debug for ManagerQueueOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagerQueueOptions")
            .field("max_previous_tracks", &self.max_previous_tracks)
            .finish()
    }
}

impl Default for ManagerQueueOptions {
    fn default() -> Self {
        Self {
            max_previous_tracks: 25,
            queue_store: None,
            queue_changes_watcher: None,
        }
    }
}
