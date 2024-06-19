@echo off
CD
start " "  powershell  $pass = ConvertTo-SecureString '1113' -AsPlainText -Force; $credential = New-Object -TypeName PSCredential -ArgumentList @('DESKTOP-150\\jpse', $pass); Start-Process -FilePath 'C:\\WINDOWS\\system32\\notepad.exe' -Credential $credential; Timeout /T 10; Exit-PSHostProcess
del start_app.bat
exit /B 0