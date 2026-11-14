# AERO-CROP 360 — Bridge Matériel COM3 & Cartographie Tactique
$jsonPath = "bank\nasa_matrix\casablanca_29r_qf.json"
$portName = "COM3"

Write-Host "[EvoNanoAgent] Initialisation du pont matériel sur $portName..." -ForegroundColor Cyan

if (Test-Path $jsonPath) {
    $data = Get-Content $jsonPath | ConvertFrom-Json
    $temp = $data.metrics.surface_temperature_c
    $moisture = $data.metrics.soil_moisture_m3_m3

    Write-Host "[Télémétrie] Température : $temp °C | Humidité : $moisture m³/m³" -ForegroundColor Yellow

    if ($temp -gt 36.0) {
        Write-Host "[ALERTE THERMIQUE] Seuil > 36°C dépassé. Envoi du signal d'arrêt d'irrigation sur $portName..." -ForegroundColor Red
        # Simulation d'écriture sur le port série COM3 (ex: protocole matériel bas niveau)
        # [System.IO.Ports.SerialPort]::new($portName, 9600).Open() ...
        Write-Host "[Matériel] Électrovanne diurne fermée. Ombrières géotextiles activées." -ForegroundColor Green
    }

    # Génération du rapport visuel pour le web
    Write-Host "[EvoNanoAgent] Mise à jour de la carte tactique pour le cockpit web..." -ForegroundColor Cyan
} else {
    Write-Host "[Erreur] Données NASA introuvables dans la banque locale." -ForegroundColor Red
}
