@echo off
CD
start " " start_app.exe --user="nome_pc\nome_usuário" --pwd="senha_usuário" --path="C:\WINDOWS\system32\notepad.exe"
del config_start_app.bat
exit /B 0