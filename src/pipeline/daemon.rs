use crate::pipeline::databank::NasaDataBank;
use std::time::Duration;
use tokio::time::sleep;

pub struct SovereignDaemon;

impl SovereignDaemon {
    pub async fn run_background_loop(interval_secs: u64) {
        let bank = NasaDataBank::open();
        println!(
            "[DAEMON-EVHT] Moteur autonome actif (100% Pure Rust | Intervalle: {}s | Cellules: {})",
            interval_secs,
            bank.count_records()
        );

        loop {
            let synced = bank.sync_strategic_sectors().await;
            let total = bank.count_records();
            println!(
                "[DAEMON-EVHT] Cycle Pipeline : {} secteurs synchronisés | Banque = {} cellules scellées.",
                synced, total
            );
            sleep(Duration::from_secs(interval_secs)).await;
        }
    }
}
