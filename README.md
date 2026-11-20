# airlock-cs-program

A Solana program that says hello and does nothing else, written with
[pinocchio](https://github.com/anza-xyz/pinocchio). It exists to be upgraded:
[Airlock](https://github.com/tiago18c/airlock)'s showcase proposes its next build through a Squads
multisig, where each member's vote lands only if the upgrade's buffer holds the build that member
read (Squads clear signing).

- Program (devnet): `eiMeJF72bc52wjXcVFSEZNnYk2R28aZhekSGqztsQ4m`
- IDL: [`idl/airlock_cs_program.json`](idl/airlock_cs_program.json), Codama, published on chain
  with program-metadata.

It holds and moves nothing. Do not send it anything of value.

## Build

The build anyone can repeat, in Docker:

```bash
solana-verify build --library-name airlock_cs_program
solana-verify get-executable-hash target/deploy/airlock_cs_program.so
```

The Solana tools version it builds with is pinned in `Cargo.toml` (`[workspace.metadata.cli]`).
Without Docker, `cargo build-sbf` with the same tools gives the same file.

## Verify what is deployed

```bash
solana-verify get-program-hash -u devnet eiMeJF72bc52wjXcVFSEZNnYk2R28aZhekSGqztsQ4m
solana-verify verify-from-repo -u devnet \
  --program-id eiMeJF72bc52wjXcVFSEZNnYk2R28aZhekSGqztsQ4m \
  --library-name airlock_cs_program \
  https://github.com/tiago18c/airlock-cs-program
```

## License

MIT.
