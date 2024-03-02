maturin build --release --target x86_64-apple-darwin -i python3.8 -i python3.9 -i python3.10 -i python3.11 -i python3.12 &&
maturin build --release -i python3.8 -i python3.9 -i python3.10 -i python3.11 -i python3.12 &&
twine upload --repository pypi --config-file ~/.pypirc ./target/wheels/*