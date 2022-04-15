# Solana account balance - Server application

This is the server application that connects with solana cluster to fetch account balance from Mainnet/Testnet/Devnet clusters.
Requires [Rust](https://www.rust-lang.org/tools/install) installed in system.

It uses [`solana-account-balance`](https://crates.io/crates/solana-account-balance) crate as dependency to connect with solana cluster and fetch account balance. `solana-account-balance` crate code is available on [GitHub](https://github.com/RijulGulati/solana-account-balance).

## Build instructions

**1) Build**

```bash
$ cargo build
```

**2) Run server**

```bash
$ cargo run
```

The server starts at `http://localhost:8100`. The host and port can be configured in `.env` file.

### Endpoints and parameters

- Endpoint: `/balance`
- Query parameters: `cluster=<val>&pubkey=<val>`
- HTTP Method: `GET`

- Possible cluster values:
  - `cluster=1` (Mainnet)
  - `cluster=2` (Testnet)
  - `cluster=3` (Devnet)

### Example

- Sample Request

```bash
$ curl 'http://localhost:8100/balance?cluster=3&pubkey=9aavjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9Ho5'
```

- Sample Response

```json
{
  "data": {
    "lamports": 599985000,
    "sol": 0.599985
  },
  "status": 0
}
```

## License

[GPL v3](https://github.com/RijulGulati/solana-balance/blob/main/LICENSE)
