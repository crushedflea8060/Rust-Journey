# 1: Simple HTTP Server
## Display Simple Http Pages
Working on a way to display all files in the current directory.

## Usage:

### Basic Usage:
```bash
cargo run
```
- cargo will compile and run it for you
### Other Usage:
```bash
mv src/main.rs .
rustc main.rs
```
- Gives you a binary that can process any of the html files in the current directory - although it cannot process subdirectories to my knowledge.. yet.

### Docker Usage:

- Refer to the build.sh that has already been included.
- If on windows, read the file and copy over the commands, it should just "work".
```bash
docker build -t web-app .
docker run -d --rm --name web web-app 
```
