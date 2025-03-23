# CoinFantasy Grid Rust Challenge

## Useful commands

### Build

The below command generates the target folder with all things necessary for the deployments. This generates a program id which can be found in `target/deploy/proposal_syste-keypair.json` to get the program id run the below command and update the program id in the `declare_id` macro and `Anchor.toml`.

```bash
anchor build
```

### Generate Deterministic Program ID

To use a deterministic program id that never changes generate a new keypair and use it to deploy your program.

```bash
solana-keygen new --outfile ./keypairs/local/proposal-system-keypair.json
```

Run the below command to get the public key that will be used as the program id.

```bash
solana address -k ./keypairs/local/proposal-system-keypair.json
```

Link keypair generate by `anchor build` to use the generated keypair using the following command.

```bash
ln -sf $(pwd)/keypairs/local/proposal-system-keypair.json        target/deploy/proposal_system-keypair.json
```

### Deploy

Deploy your program to the generated keypair using the following command.

```bash
solana program deploy --program-id ./keypairs/local/proposal-system-keypair.json     --upgrade-authority /home/<system_name>/.config/solana/id.json     target/deploy/proposal_system.so
```

The `solana-keygen new` command without file out path will generate the default keypair in the systems config folder for solana and I am using that accoutn as the upgrade authority for my program.

### Test

To run the tests use the below command. Use `--skip-local-validator` flag to avoid spinnig up a local test validator.

```bash
anchor test --skip-local-validator
```
