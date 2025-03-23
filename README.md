# CoinFantasy Grid Rust Challenge

## Useful commands

The below command generates the target folder with all things necessary for the deployments. This generates a program id which can be found in `target/deploy/proposal_syste-keypair.json` to get the program id run the below command and update the program id in the `declare_id` macro and `Anchor.toml`.

```bash
anchor build
```

Run the below command to get the program id.

```bash
solana address -k target/deploy/proposal_system-keypair.json
```

To run the tests use the below command. Use `--skip-local-validator` flag to avoid spinnig up a local test validator.

```bash
anchor test --skip-local-validator
```

