# Gouttelettes

*Ocean of solutions*

## Users

Try Gouttelettes at **gouttelettes.com**.

## Developers

```sh
mkdir -p $HOME/github.com/gouttelettes/gouttelettes
git clone git@github.com:gouttelettes/gouttelettes.git $HOME/github.com/gouttelettes/gouttelettes
alias gcl="PYTHONPATH=$HOME/github.com/gouttelettes python3 -m gouttelettes"
gcl start
```

### Setup

```sh
# rust
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack
# https://rustwasm.github.io/docs/book/game-of-life/setup.html#wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# npm
brew install npm
npm install npm@latest -g

# mkcert
# https://github.com/FiloSottile/mkcert#macos
brew install mkcert
brew install nss # if you use Firefox
```
