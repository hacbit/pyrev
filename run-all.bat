@echo off
setlocal enabledelayedexpansion

set IS_RELEASE=0
set FILES=

for %%a in (%*) do (
    if "%%a" == "--release" (
        set IS_RELEASE=1
    )
)

for %%f in (test\*.txt) do (
    set FILES=!FILES! --file %%f
)

if %IS_RELEASE% equ 1 (
    echo Running in release mode...
    cargo run --release -- !FILES!
) else (
    echo Running in debug mode...
    cargo run -- !FILES!
)

echo Press any key to continue...

pause > nul

endlocal