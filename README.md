# free-macos
Free command but for MacOS

# Prerequisites
Rust and Cargo must be installed

If not run
```
curl https://sh.rustup.rs -sSf | sh
```


# Install

Clone repo
```
git clone https://github.com/erikdahlqvist/free-macos
```
Enter project directory
```
cd free-macos
```
Compile release build
```
cargo b --release
```
Move binary to local binary directory
```
sudo mv free-macos /usr/local/bin/
```
