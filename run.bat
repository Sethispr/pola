@echo off
setlocal

:: Define the project path
set "PROJECT_DIR=%USERPROFILE%\Documents\pola"

:: Check if the directory exists
if not exist "%PROJECT_DIR%" (
    echo [ERROR] Project directory not found: "%PROJECT_DIR%"
    echo Make sure the path is correct.
    pause
    exit /b 1
)

:: Change to project directory
cd /d "%PROJECT_DIR%" || (
    echo [ERROR] Failed to change directory.
    pause
    exit /b 1
)

:: Run Cargo with release mode
echo Running Cargo...
cargo run --release

:: Check if Cargo ran successfully
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo run failed.
    pause
    exit /b %ERRORLEVEL%
)

echo [SUCCESS] Program executed successfully!
exit /b 0
