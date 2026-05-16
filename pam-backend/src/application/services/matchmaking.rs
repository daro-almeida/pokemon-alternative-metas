use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use dashmap::DashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    application::{AppResult, repositories::matchmaking::MatchmakingRepository},
    domain::arena::{arena_match::ArenaMatch, run::ArenaRunInfo},
};

pub struct QueueEntry {
    pub run_info: ArenaRunInfo,
    pub queued_at: Instant,
    pub sender: mpsc::Sender<MatchResult>,
}

#[derive(Debug, Clone)]
pub enum MatchResult {
    MatchFound(ArenaMatch),
    SearchCancelled,
    Timeout,
}

#[derive(Clone)]
pub struct Matchmaking {
    persistence: Arc<dyn MatchmakingRepository>,
    queue: Arc<DashMap<Uuid, QueueEntry>>,
}

impl Matchmaking {
    pub fn new(persistence: Arc<dyn MatchmakingRepository>) -> Self {
        Self {
            persistence,
            queue: Arc::new(DashMap::new()),
        }
    }

    pub fn start(&self) {
        let this = self.clone();
        tokio::spawn(async move {
            this.matchmaking_loop().await;
        });
    }

    async fn matchmaking_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            self.try_make_matches().await;
            self.cleanup_timeouts().await;
        }
    }

    pub async fn queue_for_match(
        &self,
        run_info: ArenaRunInfo,
    ) -> AppResult<mpsc::Receiver<MatchResult>> {
        let (tx, rx) = mpsc::channel(1);

        let entry = QueueEntry {
            run_info,
            queued_at: Instant::now(),
            sender: tx,
        };

        self.queue.insert(entry.run_info.run_id, entry);

        Ok(rx)
    }

    pub fn cancel_search(&self, run_id: &Uuid) -> bool {
        if let Some((_, entry)) = self.queue.remove(run_id) {
            let _ = entry.sender.try_send(MatchResult::SearchCancelled);
            true
        } else {
            false
        }
    }

    async fn try_make_matches(&self) {
        let mut matched_runs = Vec::new();

        // Collect just the run_ids to reduce cloning
        let run_ids: Vec<Uuid> = self.queue.iter().map(|e| *e.key()).collect();

        for i in 0..run_ids.len() {
            if matched_runs.contains(&run_ids[i]) {
                continue;
            }

            for j in (i + 1)..run_ids.len() {
                if matched_runs.contains(&run_ids[j]) {
                    continue;
                }

                let run_id_1 = run_ids[i];
                let run_id_2 = run_ids[j];

                // Get entries only when needed
                let Some(entry_1) = self.queue.get(&run_id_1) else {
                    continue;
                };
                let Some(entry_2) = self.queue.get(&run_id_2) else {
                    continue;
                };

                if self
                    .can_match(&entry_1.run_info, &entry_2.run_info, entry_1.queued_at)
                    .await
                {
                    // Clone data before async call
                    let sender_1 = entry_1.sender.clone();
                    let sender_2 = entry_2.sender.clone();
                    let run_info_1 = entry_1.run_info.clone();
                    let run_info_2 = entry_2.run_info.clone();

                    // Drop refs before await
                    drop(entry_1);
                    drop(entry_2);

                    match self
                        .persistence
                        .create_match(&run_info_1, &run_info_2)
                        .await
                    {
                        Ok(arena_match) => {
                            let _ = sender_1
                                .send(MatchResult::MatchFound(arena_match.clone()))
                                .await;
                            let _ = sender_2.send(MatchResult::MatchFound(arena_match)).await;

                            self.queue.remove(&run_id_1);
                            self.queue.remove(&run_id_2);

                            matched_runs.push(run_id_1);
                            matched_runs.push(run_id_2);
                            break;
                        }
                        Err(e) => {
                            tracing::error!("Failed to create match: {}", e);
                        }
                    }
                }
            }
        }
    }

    async fn can_match(
        &self,
        run1: &ArenaRunInfo,
        run2: &ArenaRunInfo,
        queued_at: Instant,
    ) -> bool {
        // Check if they've already played each other
        if let Ok(has_played) = self
            .persistence
            .have_runs_played(&run1.run_id, &run2.run_id)
            .await
            && has_played
        {
            return false;
        }

        // Expand range based on time in queue
        let time_in_queue = queued_at.elapsed().as_secs();
        let wins_losses_range = time_in_queue / 10; // Expand by 1 every 10 seconds

        // For now, simple matching - can add elo/MMR later
        let diff = ((run1.wins - run1.losses) as i32).abs_diff((run2.wins - run2.losses) as i32);

        diff <= wins_losses_range as u32
    }

    async fn cleanup_timeouts(&self) {
        let timeout = Duration::from_secs(120); // 2 minute timeout
        let now = Instant::now();

        let timed_out: Vec<_> = self
            .queue
            .iter()
            .filter(|entry| now.duration_since(entry.queued_at) > timeout)
            .map(|entry| *entry.key())
            .collect();

        for run_id in timed_out {
            if let Some((_, entry)) = self.queue.remove(&run_id) {
                let _ = entry.sender.try_send(MatchResult::Timeout);
            }
        }
    }
}
