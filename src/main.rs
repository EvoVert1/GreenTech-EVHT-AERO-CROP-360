mod tokenizer;
mod com_bridge;

fn main() {
    println!("==================================================");
    println!("🌱 AERO-CROP 360 — Unified Sovereign Edge Core");
    println!("==================================================");
    
    tokenizer::ingest_nasa_and_tokenize();

    if let Err(e) = com_bridge::read_com3_binary_frames("COM3") {
        eprintln!("[Erreur Matériel COM3]: {}", e);
    }

    println!("[Cartographie] Grille MGRS 29R QF (Casablanca) synchronisée sans latence.");
    println!("==================================================");
}
