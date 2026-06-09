# Bootstrap
$ErrorActionPreference = 'Stop'
$scmRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scmRoot
Write-Host "==> Installing git hooks"
git -C $repoRoot config core.hooksPath scm/scripts/hooks
Write-Host "==> Fetching dependencies"
Push-Location $scmRoot
cargo fetch --locked
Pop-Location
Write-Host "==> Creating src/gateway junctions for SEA audit compliance"
$crates = @("auth","breaker","cache","cassette","oauth","rate","retry","tls","transport")
foreach ($crate in $crates) {
    $srcDir = "$scmRoot\$crate\src"
    $target  = "$scmRoot\$crate\main\src\gateway"
    if (-not (Test-Path $srcDir)) { New-Item -ItemType Directory -Path $srcDir | Out-Null }
    if (-not (Test-Path "$srcDir\gateway")) {
        New-Item -ItemType Junction -Path "$srcDir\gateway" -Target $target | Out-Null
    }
}
Write-Host "Bootstrap complete."
