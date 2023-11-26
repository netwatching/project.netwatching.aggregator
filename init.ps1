# Clear contents of CHANGELOG.md
Set-Content -Path "CHANGELOG.md" -Value ""

# Set contents of version.txt to "0.0.0"
Set-Content -Path "version.txt" -Value "0.0.0"

# Set contents of .release-please-manifest.json
$jsonContent = @"
{
  ".": "0.0.0"
}
"@
Set-Content -Path ".release-please-manifest.json" -Value $jsonContent

# Clear or create README.md
if (Test-Path -Path "README.md") {
    # Clear contents if file exists
    Set-Content -Path "README.md" -Value ""
} else {
    # Create an empty file if it does not exist
    New-Item -Path "README.md" -ItemType "file"
}

# Get basic repo information and write to README.md
$repoInfo = git config --get remote.origin.url
$repoName = $repoInfo.Split('/')[-1].Replace('.git','')
$readmeContent = "# $repoName`  `n- More information will be added soon`  `n- Please update your GitHub settings as described [here](https://github.com/netwatching/template.docker/blob/main/README.md) "
Set-Content -Path "README.md" -Value $readmeContent

# Add changes to staging
git add .

# Commit changes with a custom message
git commit -m "chore: release 0.0.1" -m "Release-As: 0.0.1"

# Push changes to the repository
git push 

# Schedule the deletion of this script (init.ps1) after a short delay
Start-Sleep -Seconds 1
Remove-Item -Path "init.ps1" -Force
