# if parameter -env is dev use relative path to ./target/release/
# otherwise use github path: https://github.com/alvaromagu/rtop/releases/

# The program gets rtop.exe from local path or github

# Usage:
#   -env: environment to get rtop.exe from
#     - prod: get rtop.exe from github # default
#     - dev: get rtop.exe from ./target/release/

# Example:
#   .\scripts\install.ps1 -env prod (or) .\scripts\install.ps1
#   .\scripts\install.ps1 -env dev

param(
  [string]$env = "prod"
)

$app_folder = "$env:USERPROFILE/.rtop/"

# function to init app folder
function init_app_folder {
  if (-not (Test-Path $app_folder)) {
    # do not show standard output
    New-Item -ItemType Directory -Path $app_folder > $null
  }
}

if ($env -eq "dev") {
  $path = "./target/release/rtop.exe"
  # if release doesnt exist, throw error
  if (-not (Test-Path $path)) {
    Write-Host "Error: $path does not exist, please run 'cargo build --release' first"
    exit 1
  }
  init_app_folder
  # copy file in $path to current user's home app directory ($env:USERPROFILE/.rtop/)
  Copy-Item $path $app_folder
}
else {
  # get version from Cargo.toml file stored in github main branch
  # get content of Cargo.toml file
  $cargoToml = (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/alvaromagu/rtop/main/Cargo.toml").Content
  # Define a regular expression to match the version
  $versionRegex = 'version\s*=\s*"(.*?)"'
  # Extract the version using the regular expression
  if ($cargoToml -match $versionRegex) {
    $version = $matches[1]
    Write-Output "Current version: [$version]"
  } else {
    Write-Output "Version not found"
    exit 1
  }
  Write-Host "Downloading rtop version [$version] from github"
  init_app_folder
  # download rtop.exe from github
  # github releases page https://github.com/alvaromagu/rtop/releases/download/v{$version}/rtop.exe
  $gh_path = "https://github.com/alvaromagu/rtop/releases/download/v$version/rtop.exe"
  # download file from github
  Invoke-WebRequest -Uri $gh_path -OutFile "$app_folder/rtop.exe"
}

$env_path = [System.Environment]::GetEnvironmentVariable('Path', 'User')
# if rtop.exe is not in current user PATH env, add it
# regex to chheck
if ($env_path -split ";" -notcontains "$env:USERPROFILE\.rtop") {
  # create a backup of user PATH env
  Write-Host "info: Writing rtop to user PATH, backup of user PATH can be found in $env:USERPROFILE/.rtop/backup_path.txt"
  # take in account that the file might already exist
  # if it does, overwrite it
  # ONLY SAVE USER PATH, SYSTEM PATH IS NOT SAVED
  $env_path | Out-File "$env:USERPROFILE/.rtop/backup_path.txt"
  # if env path ends with ;, remove it
  if ($env_path.EndsWith(";")) {
    $env_path = $env_path.Substring(0, $env_path.Length - 1)
  }
  # add rtop.exe to PATH permanently
  $env_path += ";$env:USERPROFILE\.rtop;"
  [System.Environment]::SetEnvironmentVariable('Path', $env_path, 'User')
  Write-Host "info: rtop.exe added to PATH. Please restart your terminal to use rtop"
}
else {
  Write-Host "info: rtop.exe already in PATH"
}
