maturin build --release --target x86_64-apple-darwin --interpreter python3.8 --interpreter python3.9 --interpreter python3.10 --interpreter p
ython3.11 --interpreter python3.12 &&

maturin build --release --interpreter python3.8 --interpreter python3.9 --interpreter python3.10 --interpreter python3.11 --interpreter python3.12

twine upload ./target/wheels/*
