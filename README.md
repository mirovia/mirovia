# Mirovia

*Ocean of solutions*

## Users

Try Mirovia at **mirovia.one**.

## Developers

```sh
mkdir -p $HOME/github.com/mirovia/mirovia
git clone git@github.com:mirovia/mirovia.git $HOME/github.com/mirovia/mirovia
alias mcl="PYTHONPATH=$HOME/github.com/mirovia python3 -m mirovia"
mcl start
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
