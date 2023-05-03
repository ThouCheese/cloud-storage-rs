set -e
echo && echo '--------------------------------' && echo 'Runing async tests'
cargo test --features dotenv,sync,global-client -- --test-threads=1
echo && echo '--------------------------------' && echo 'Runing async tests with rustls'
cargo test --no-default-features --features dotenv,sync,rustls-tls,global-client -- --test-threads=1
echo && echo '--------------------------------' && echo 'Runing sync tests with all features'
/cargo test --all-features -- --test-threads=1
