@echo off

@REM Starts a local web-server that serves the contents of the `doc/` folder,
@REM which is the folder to where the web version is compiled.

cargo install basic-http-server

echo "open http://localhost:8081"

(cd docs && basic-http-server --addr 0.0.0.0:8081 .)
@REM (cd docs && python3 -m http.server 8081 --bind 0.0.0.0)
