build:
    cargo build

fmt:
    pre-commit run --all-files

release:
    cargo build --release
    cp -p ./target/release/urlx ~/bin/urlx

install target="debug":
    cp ./target/{{target}}/urlx ~/bin/urlx

run *args:
    cargo run {{args}}

# gh

pr:
    gh pr create --assignee=@me
