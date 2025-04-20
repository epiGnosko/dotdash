cargo build --release
chmod +x target/release/dotdash
sudo cp target/release/dotdash /usr/bin 
rm -rf target
