# solana-counter

Anchor program implementing a simple on-chain counter with two instructions: `initialize` and `increment`.

## Program

- **Program ID (devnet):** `4XbSb17wfoEDD6RKhKu9V3rjwvvpqJqeDwBRvxjJyziP`
- **Network:** Solana Devnet

## Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Creates the counter PDA and sets `count = 0` |
| `increment`  | Increments `count` by 1 |

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
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

## Interact with the deployed program

```bash
cargo run --bin interact
```

The binary calls `initialize` then `increment` on devnet and prints the Solana Explorer links for both transactions.
