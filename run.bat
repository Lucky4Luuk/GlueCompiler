@echo off
cargo run
ld result.o -o output.exe
output.exe
echo Program output: %errorlevel%
