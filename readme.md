curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get update
sudo apt-get install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release

Crea un archivo de configuración para Cargo en tu proyecto. En la raíz de tu proyecto, crea un directorio .cargo y dentro de él, un archivo config.toml:
mkdir -p .cargo
nano .cargo/config.toml

Añade las siguientes líneas al archivo config.toml:
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"