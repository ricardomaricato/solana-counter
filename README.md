# solana-counter

Anchor program implementing a simple on-chain counter with two instructions: `initialize` and `increment`.

## Program

- **Program ID (devnet):** `4XbSb17wfoEDD6RKhKu9V3rjwvvpqJqeDwBRvxjJyziP`
- **Network:** Solana Devnet
- **Explorer:** https://explorer.solana.com/address/4XbSb17wfoEDD6RKhKu9V3rjwvvpqJqeDwBRvxjJyziP?cluster=devnet

## Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Creates the counter PDA and sets `count = 0` |
| `increment`  | Increments `count` by 1 |

## On-chain interactions

| Transaction | Explorer |
|-------------|----------|
| `initialize` | https://explorer.solana.com/tx/YCeXZDb1H4syY77Y9agoxbdVQAMy7Zzaiy7G72qRZxUjgU7zSd2NGoo82Zmxr5CxCSGRZC3toWLUoAPYLF1fgRh?cluster=devnet |
| `increment`  | https://explorer.solana.com/tx/eVtzG2LQh5dWdzEzHmQxfqKp7jq9VQCDrHBhdqPvkSnwgYwwNsbJiJNyEnH9Dv29zWRep9bkdi8driUCGyFMuoA?cluster=devnet |

## Build

```bash
anchor build
```

## Test

```bash
cargo test
```

## Deploy to devnet

```bash
solana program deploy \
  target/deploy/solana_counter.so \
  --url devnet \
  --keypair ~/.config/solana/id.json \
  --program-id target/deploy/solana_counter-keypair.json \
  --use-rpc
```

## Interact with the deployed program

```bash
cd app && npm install
cd ..
NODE_TLS_REJECT_UNAUTHORIZED=0 \
  SOLANA_RPC_URL=https://api.devnet.solana.com \
  ANCHOR_WALLET=~/.config/solana/id.json \
  node app/interact.mjs
```
