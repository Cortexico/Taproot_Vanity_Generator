@echo off
echo 🦀 Building Ultra-Fast Taproot Vanity Generator...
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust is not installed!
    echo Please install Rust from: https://rustup.rs/
    echo.
    echo After installation, restart your terminal and run this script again.
    pause
    exit /b 1
)

echo ✅ Rust found, building optimized release...
cargo build --release

if %ERRORLEVEL% EQ 0 (
    echo.
    echo 🎉 Build successful!
    echo.
    echo 📁 Executable location: target\release\taproot-vanity.exe
    echo.
    echo 🚀 Usage examples:
    echo   target\release\taproot-vanity.exe
    echo   target\release\taproot-vanity.exe --min-kek-count 3
    echo   target\release\taproot-vanity.exe --workers 16
    echo   target\release\taproot-vanity.exe --output-file my_kek_addresses.txt
    echo   target\release\taproot-vanity.exe --case-sensitive
    echo.
    echo 💡 For maximum performance, use all CPU cores:
    echo   target\release\taproot-vanity.exe --workers %NUMBER_OF_PROCESSORS%
    echo.
) else (
    echo ❌ Build failed!
    echo Check the error messages above.
)

pause
