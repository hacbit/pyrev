@echo off
setlocal

set IS_RELEASE=0

for %%a in (%*) do (
    if "%%a" == "--release" (
        set IS_RELEASE=1
    )
)

if %IS_RELEASE% equ 1 (
    echo Building in release mode
    cargo build --release
) else (
    echo Building in debug mode
    cargo build
)

for %%f in (test\*.txt) do (
    echo Running test %%f
    if %IS_RELEASE% equ 1 (
        cargo run --release -- --file %%f
    ) else (
        cargo run -- --file %%f
    )
)

echo Press any key to continue...

pause > nul

endlocal