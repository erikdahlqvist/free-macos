# free-macos
Free command but for MacOS

# Prerequisites
Have MacOS installed and access to Terminal <br><br>
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
cargo b -r
```
Move binary to local binary directory
```
sudo mv target/release/free /usr/local/bin/
```
