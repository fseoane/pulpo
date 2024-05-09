clear 
cargo clean
cargo build --release -j 4
ls -lah target/release/pulpo
sudo cp target/release/pulpo /opt/pulpo
