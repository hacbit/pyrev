@echo off
setlocal enabledelayedexpansion

echo Updating documentation...
set RUNNER=cargo doc --no-deps -p pyrev

for /D %%d in (src/pyrev*) do (
    set FOLDER=%%~nd
    if "!FOLDER:~0,5!"=="pyrev" (
        echo Building documentation for !FOLDER!
        set RUNNER=!RUNNER! -p !FOLDER!
    )
)

echo Try run: %RUNNER%
!RUNNER!

echo Delete old docs
echo Try run: rd /S /Q .\doc\
rd /S /Q .\doc\

echo Copy docs from target/ to crate root
echo Try run: xcopy /E /I .\target\doc\ .\doc\
xcopy /E /I .\target\doc\ .\doc\

endlocal