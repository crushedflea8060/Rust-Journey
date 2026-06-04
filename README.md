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
Gives you a binary that can process any of the html files in the current directory - although it cannot process subdirectories to my knowledge.. yet.
