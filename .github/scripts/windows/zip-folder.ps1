# reads the provided artifacts folder path argument
$artifactsPath=$args[0]

# creates program's folder
$programFolder="$artifactsPath/server"
New-Item -Path $programFolder -ItemType Directory

# moves program's file to folder
Move-Item -Path "./target/release/*.exe" -Destination $programFolder
Move-Item -Path "./resources/*" -Destination $programFolder

# zips folder and give it the name server-portable-win-x64.zip
Compress-Archive -Path $programFolder -DestinationPath "$artifactsPath/server-portable-win-x64.zip"

# cleanup
Remove-Item $programFolder -Recurse
