@echo off
REM Change to the native directory and run the BUILD.cmd script there
cd .\src\native
call .\BUILD.cmd

REM Return to the root directory
cd ..\..

REM Run cargo build
cargo build

REM Check if cargo build was successful
if %errorlevel% equ 0 (
    echo Cargo build completed successfully.
) else (
    echo Cargo build failed with error level %errorlevel%.
    exit /b %errorlevel%
)
