# Snask Installer Script
$ErrorActionPreference = "Stop"

Write-Host "Iniciando instalacao do Snask 0.9.2..." -ForegroundColor Cyan

# 1. Definir diretorios
$installDir = "$env:LOCALAPPDATA\Snask"
$currentDir = Get-Location
$binDir = "$installDir\bin"
$sourceExe = "$currentDir\target\release\snask.exe"

Write-Host "Diretorio de instalacao: $installDir"

# 2. Verificar se o binario existe
if (-not (Test-Path $sourceExe)) {
    Write-Error "Erro: executavel nao encontrado. Certifique-se de ter compilado o projeto."
}

# 3. Preparar diretorio de instalacao
if (Test-Path $installDir) {
    Write-Host "Diretorio ja existe. Atualizando..." -ForegroundColor Yellow
    if (Test-Path $binDir) { Remove-Item -Path $binDir -Recurse -Force }
} else {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

New-Item -ItemType Directory -Path $binDir | Out-Null

# 4. Copiar arquivos
Write-Host "Copiando arquivos..."

# Copiar executavel
Copy-Item -Path $sourceExe -Destination "$binDir\snask.exe" -Force
Write-Host "  Executavel copiado"

# Copiar codigo fonte
$exclude = @("target", ".git", ".vscode", "install.ps1", "bin")
Get-ChildItem -Path $currentDir -Exclude $exclude | Copy-Item -Destination $installDir -Recurse -Force
Write-Host "  Codigo fonte copiado"

# 5. Configurar Variaveis de Ambiente
Write-Host "Configurando variaveis de ambiente..."

# Definir SNASK_HOME
[System.Environment]::SetEnvironmentVariable("SNASK_HOME", $installDir, "User")
Write-Host "  SNASK_HOME definido"

# Adicionar ao PATH
$currentPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$binDir*") {
    $newPath = "$currentPath;$binDir"
    [System.Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "  Adicionado ao PATH"
} else {
    Write-Host "  Ja esta no PATH"
}

Write-Host "Instalacao concluida com sucesso!" -ForegroundColor Green
Write-Host "Por favor, reinicie seu terminal." -ForegroundColor Yellow
