#!/usr/bin/env pwsh

# PowerShell script to add a Kafka profile
# Usage: .\run-profile-add.ps1

Write-Host "Reading Kafka..." -ForegroundColor Green

cargo run -- read -b kuberunner227d:32764 -t impressions -p profile001

if ($LASTEXITCODE -eq 0) {
    Write-Host "Profile added successfully!" -ForegroundColor Green
} else {
    Write-Host "Failed to add profile. Exit code: $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}
