# 1: Simple HTTP Server
## Display Simple Http Pages
Working on a way to display all files in the current directory.

## Usage:

### Basic Usage:
```bash
rustc main.rs
chmod +x main
./main
```
- Gives you a binary that can process any of the html files in the current directory - although it cannot process subdirectories to my knowledge.. yet.
### Easiest Usage:
```bash
./run.sh
'''
 - Specific to POSIX, but it's the fastest
### Docker Usage:

- Refer to the build.sh that has already been included.
- If on windows, read the file and copy over the commands, it should just "work".
```bash
docker build -t web-app .
docker run -i -p 8000:8000 --rm --name web web-app 
```
