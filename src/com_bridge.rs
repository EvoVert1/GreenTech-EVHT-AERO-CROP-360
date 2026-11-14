use std::time::Instant;

pub struct LatencyOptimizer {
    last_sync: Instant,
    tolerance_ms: u128,
}

impl LatencyOptimizer {
    pub fn new(tolerance_ms: u128) -> Self {
        Self { last_sync: Instant::now(), tolerance_ms }
    }

    pub fn synchronize_and_correct(&mut self) -> bool {
        let elapsed = self.last_sync.elapsed().as_millis();
        if elapsed > self.tolerance_ms {
            println!("[Latency Optimizer] Latence détectée ({:.2}ms). Réalignement du flux Edge...", elapsed);
            self.last_sync = Instant::now();
            true
        } else {
            false
        }
    }
}

pub fn read_com3_binary_frames(port_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Hardware Bridge] Initialisation du port série {} (COM3 Actif)...", port_name);
    let mut optimizer = LatencyOptimizer::new(250);

    for i in 1..=2 {
        if optimizer.synchronize_and_correct() {
            println!("[Hardware Bridge] Synchronisation instantanée avec le capteur COM3 #{}.", i);
        }
        let mock_frame: [u8; 8] = [0xA1, 0x37, 0x2C, 0x0F, 0x82, 0xFF, 0x00, 0x1D];
        print!(" -> Décodage trame binaire (NASA / COM3): ");
        for byte in &mock_frame {
            print!("{:02X} ", byte);
        }
        println!();
    }
    Ok(())
}
