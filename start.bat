@echo off
cd server

if not exist venv (
    echo.Creating virtual environment...
    python -m venv venv
    echo.Installing requirements...
    call ./venv/scripts/activate.bat
    pip install -r requirements.txt
    deactivate
)
echo.Starting server...
start start_server.bat
cd ..
start cn_overlay.exe
