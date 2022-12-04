#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --accountId proxy.andrii_yer.testnet --wasmFile ./target/wasm32-unknown-unknown/release/deposit_proxy.wasm 
#--initFunction new --initArgs '{"depositary_account": "depository.andrii_yer.testne"}'
