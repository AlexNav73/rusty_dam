Get-ChildItem -Path .\ -Include *.bk -Recurse | foreach { $_.Delete() }
