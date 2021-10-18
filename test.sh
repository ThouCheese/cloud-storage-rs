set -e
echo && echo '--------------------------------' && echo 'Runing sync tests'
cargo test --features sync,global-client -- --test-threads=1
echo && echo '--------------------------------' && echo 'Runing sync tests with rustls'
cargo test --no-default-features --features sync,rustls-tls,global-client -- --test-threads=1
echo && echo '--------------------------------' && echo 'Runing sync tests with all features'
cargo test --all-features -- --test-threads=1
