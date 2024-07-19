# if parameter -env is dev use relative path to ./target/release/
# otherwise use github path: https://github.com/alvaromagu/rtop/releases/

# The program gets rtop.exe from local path or github

# Usage:
#   -env: environment to get rtop.exe from
#     - dev: get rtop.exe from ./target/release/
#     - prod: get rtop.exe from github

# Example:
#   .\scripts\install.ps1 -env dev
#   .\scripts\install.ps1 -env prod

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
  $gh_path = "https://github.com/alvaromagu/rtop/raw/main/releases/rtop.exe"
  # download file from github
  $req = Invoke-WebRequest -Uri $gh_path
  # if request is successful, init app folder and save content 
  # into rtop.exe file in current user's home app directory ($env:USERPROFILE/.rtop/)
  if ($req.StatusCode -eq 200) {
    init_app_folder
    $req.Content | Set-Content "$app_folder/rtop.exe"
  }
  else {
    Write-Host "Error: $path does not exist"
    exit 1
  }
}

$env_path = [System.Environment]::GetEnvironmentVariable('Path', 'User')
# if rtop.exe is not in current user PATH env, add it
# regex to chheck
if ($env_path -split ";" -notcontains "$env:USERPROFILE/.rtop") {
  # create a backup of user PATH env
  Write-Host "Writing htop to user PATH, backup of user PATH can be found in $env:USERPROFILE/.rtop/backup_path.txt"
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
  Write-Host "rtop.exe added to PATH"
}
else {
  Write-Host "rtop.exe already in PATH"
}
