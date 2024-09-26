curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
--
sudo apt-get update
--
sudo apt-get install gcc-mingw-w64-x86-64
--
rustup target add x86_64-pc-windows-gnu
--
cargo build --target x86_64-pc-windows-gnu --release
--
