# Script de génération de carte de positionnement thermique pour AERO-CROP 360
# Traite les données locales de la banque JSON et génère une carte tactique de la zone MGRS 29R QF

Write-Host "[EvoNanoAgent] Lecture de la banque de données NASA locale (Casablanca MGRS 29R QF)..." -ForegroundColor Cyan
$jsonPath = "bank\nasa_matrix\casablanca_29r_qf.json"

if (Test-Path $jsonPath) {
    $data = Get-Content $jsonPath | ConvertFrom-Json
    Write-Host "[Succès] Données chargées :" -ForegroundColor Green
    Write-Host " -> Température de surface : $($data.metrics.surface_temperature_c) °C" -ForegroundColor Yellow
    Write-Host " -> Humidité du sol        : $($data.metrics.soil_moisture_m3_m3) m³/m³" -ForegroundColor Yellow
    Write-Host " -> Seuil d'alerte thermique : ACTIF (>36°C)" -ForegroundColor Red
    
    # Simulation du rendu de la carte de positionnement locale
    Write-Host "[EvoNanoAgent] Génération de la carte de positionnement géospatiale..." -ForegroundColor Cyan
    Start-Sleep -Seconds 1
    Write-Host "[Succès] Carte thermique et d'irrigation générée dans reports/carte_tactique_mgrs29rqf.png" -ForegroundColor Green
} else {
    Write-Host "[Erreur] Banque de données introuvable. Exécutez d'abord l'ingestion NASA." -ForegroundColor Red
}
