mod tokenizer;

fn main() {
    println!("==================================================");
    println!("🌱 AERO-CROP 360 — Sovereign Rust Edge Engine");
    println!("==================================================");
    
    // Exécution du pipeline de tokenisation et d'ingestion NASA
    tokenizer::ingest_nasa_and_tokenize();
    
    println!("==================================================");
    println!("[Nanoagent] Wallet & Tokenizer Pipeline fully integrated.");
}
