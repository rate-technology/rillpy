cargo build

pushd target/debug
mv librillrate.so rillrate.so
ls
popd