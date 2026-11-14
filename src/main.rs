mod core;
mod export;
mod ingest;
mod pillars;
mod pipeline;
mod server;

use clap::{Parser, Subcommand};
use core::models::AgentRequest;
use core::orchestrator::AeroCropNanoAgent;
use export::markdown::export_study_report;
use ingest::photography::PhotographyEngine;
use pipeline::daemon::SovereignDaemon;
use pipeline::databank::NasaDataBank;

#[derive(Parser)]
#[command(name = "aerocrop-nanoagent")]
#[command(about = "Evo High-Tech — NanoAgent Autonome Sans-LLM, Banque NASA, Bootstrap & Cockpit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Outil d'auto-amorçage : force l'aspiration et l'indexation immédiate des banques NASA
    Bootstrap,
    SyncBank,
    Daemon {
        #[arg(short, long, default_value_t = 60)]
        interval: u64,
    },
    Audit {
        #[arg(short, long)]
        query: Option<String>,
        #[arg(long)]
        lat: Option<f64>,
        #[arg(long)]
        lon: Option<f64>,
        #[arg(short, long)]
        image: Option<String>,
        #[arg(long, default_value_t = false)]
        mars: bool,
    },
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    Sandbox,
}

#[tokio::main]
async fn main() {
    if !std::env::args().any(|a| a == "serve") { ensure_daemon_running(8080); }
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap => {
            println!("[NANOAGENT-BOOTSTRAP] Lancement de l'outil autonome d'aspiration NASA...");
            let bank = NasaDataBank::open();
            println!("[NANOAGENT-BOOTSTRAP] État initial de la banque : {} cellules scellées.", bank.count_records());
            let synced = bank.sync_strategic_sectors().await;
            println!("[NANOAGENT-BOOTSTRAP] Succès ! {} secteurs synchronisés et indexés dans bank/index/sovereign_bank.json", synced);
            println!("[NANOAGENT-BOOTSTRAP] Total cellules en banque : {}", bank.count_records());
        }
        Commands::SyncBank => {
            let bank = NasaDataBank::open();
            let count = bank.sync_strategic_sectors().await;
            println!("[EVHT-PIPELINE] {} secteurs synchronisés. Total cellules : {}", count, bank.count_records());
        }
        Commands::Daemon { interval } => {
            SovereignDaemon::run_background_loop(interval).await;
        }
        Commands::Audit { query, lat, lon, image, mars } => {
            let report = AeroCropNanoAgent::execute(AgentRequest {
                query, lat, lon, image_path: image, force_mars: mars,
            }).await;
            let path = export_study_report(&report).unwrap_or_else(|_| "N/A".into());
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
            eprintln!("\n[EVHT] Rapport exporté : {}", path);
        }
        Commands::Serve { port } => {
            server::start_server(port).await;
        }
        Commands::Sandbox => {
            println!("=========================================================================");
            println!("  EVO HIGH-TECH — BAC À SABLE & AUTO-AMORÇAGE (TERRE & MARS)             ");
            println!("=========================================================================\n");

            let bank = NasaDataBank::open();
            let synced = bank.sync_strategic_sectors().await;
            println!("[1/4] Banque NASA amorcée : {} secteurs | {} cellules indexées.", synced, bank.count_records());

            let sample_img = "bank/captures/sandbox_chaouia_parcelle.png";
            let _ = PhotographyEngine::generate_sandbox_satellite_capture(sample_img);
            println!("[2/4] Capture satellite spectrale générée : {}", sample_img);

            let earth_report = AeroCropNanoAgent::execute(AgentRequest {
                query: Some("Audit hydrique urgent parcelle Chaouia 33.5731, -7.5898".into()),
                lat: Some(33.5731),
                lon: Some(-7.5898),
                image_path: Some(sample_img.into()),
                force_mars: false,
            }).await;
            let earth_md = export_study_report(&earth_report).unwrap_or_default();
            println!("[3/4] Terre -> {} (Rapport: {})", earth_report.executive_synthesis, earth_md);

            let mars_report = AeroCropNanoAgent::execute(AgentRequest {
                query: Some("Simulation serre pressurisée CELSS Cratère Jezero Mars".into()),
                lat: Some(18.3800),
                lon: Some(77.5800),
                image_path: None,
                force_mars: true,
            }).await;
            let mars_md = export_study_report(&mars_report).unwrap_or_default();
            println!("[4/4] Mars  -> {} (Rapport: {})\n", mars_report.executive_synthesis, mars_md);
            println!("Pour lancer l'outil d'auto-amorçage seul : cargo run --release -- bootstrap");
        }
    }
}


/// Vérifie si le Cockpit/Démon écoute sur le port 8080 ; sinon le détache automatiquement en arrière-plan
pub fn ensure_daemon_running(port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    if std::net::TcpStream::connect(&addr).is_err() {
        if let Ok(exe) = std::env::current_exe() {
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                const DETACHED_PROCESS: u32 = 0x00000008;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                let _ = std::process::Command::new(exe)
                    .args(["serve", "--port", &port.to_string()])
                    .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
                    .spawn();
            }
            #[cfg(not(windows))]
            {
                let _ = std::process::Command::new(exe)
                    .args(["serve", "--port", &port.to_string()])
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            }
            std::thread::sleep(std::time::Duration::from_millis(600));
            println!("[EVHT-DAEMON] Cockpit 3D & API auto-réveillés en tâche de fond sur http://localhost:{}", port);
        }
    }
}
