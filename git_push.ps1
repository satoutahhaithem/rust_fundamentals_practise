Write-Host "Enter commit message:"
$commitMessage = Read-Host

git add .
git commit -m "$commitMessage"
git push origin main
