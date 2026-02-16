# SOLGuard AI - Devnet Deployment Instructions

## Quick Deploy (Automated Script)

The easiest way to deploy SOLGuard AI to Devnet:

```bash
cd ~/solguard-ai
git pull origin master
./deploy_devnet.sh
```

That's it! The script handles everything automatically:
- ✅ Environment checks
- ✅ Devnet configuration
- ✅ Balance verification
- ✅ Building all 4 programs
- ✅ Deploying to Devnet
- ✅ Displaying program IDs

**Time**: 10-15 minutes  
**Cost**: ~10-15 SOL (Devnet - free from faucet)

---

## Manual Deployment (Step-by-Step)

If you prefer manual control:

### 1. Pull Latest Code

```bash
cd ~/solguard-ai
git pull origin master
```

### 2. Configure for Devnet

```bash
solana config set --url devnet
solana config get
```

### 3. Check/Fund Wallet

```bash
solana address
solana balance

# If balance < 10 SOL, get airdrops
solana airdrop 2
# Repeat 4-5 times or use web faucets
```

### 4. Build Programs

```bash
# Use nightly Rust (handles Cargo.lock v4)
rustup override set nightly

# Clean and build
anchor clean
rm -f Cargo.lock
anchor build
```

### 5. Verify Build

```bash
ls -lh target/deploy/*.so
```

You should see 4 .so files:
- security_oracle.so
- validator_registry.so
- governance.so
- transparency_vault.so

### 6. Deploy

```bash
anchor deploy
```

### 7. Get Program IDs

```bash
solana-keygen pubkey target/deploy/security_oracle-keypair.json
solana-keygen pubkey target/deploy/validator_registry-keypair.json
solana-keygen pubkey target/deploy/governance-keypair.json
solana-keygen pubkey target/deploy/transparency_vault-keypair.json
```

---

## Troubleshooting

### Issue: Cargo.lock version error

**Solution**: Use nightly Rust
```bash
rustup override set nightly
rm -f Cargo.lock
anchor build
```

### Issue: Insufficient balance

**Solution**: Get Devnet SOL from web faucets
- https://faucet.quicknode.com/solana/devnet
- https://faucet.solana.com/

Paste your wallet address (from `solana address`)

### Issue: Build takes too long

**Normal**: Building 4 programs takes 5-10 minutes on first build

### Issue: Platform tools error

**Solution**: Reinstall Solana
```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

---

## After Deployment

### 1. Run Tests on Live Network

```bash
anchor test
```

### 2. Update README

Add program IDs to README.md

### 3. Push to GitHub

```bash
git add README.md Anchor.toml
git commit -m "docs: Add Devnet deployment program IDs"
git push origin master
```

### 4. Verify on Explorer

Visit:
```
https://explorer.solana.com/address/YOUR_PROGRAM_ID?cluster=devnet
```

---

## Expected Costs

| Program | Approximate Cost |
|---------|-----------------|
| Security Oracle | ~3-4 SOL |
| Validator Registry | ~2-3 SOL |
| Governance | ~2-3 SOL |
| Transparency Vault | ~2-3 SOL |
| **Total** | **~10-13 SOL** |

All Devnet SOL is free from faucets!

---

## Success Indicators

✅ All 4 .so files created  
✅ Deployment completes without errors  
✅ 4 program IDs generated  
✅ Programs visible on Solana Explorer  
✅ Tests pass on live network  

---

## Need Help?

If you encounter issues:

1. Check the error message carefully
2. Ensure you have latest code: `git pull origin master`
3. Try the automated script: `./deploy_devnet.sh`
4. Check Solana status: https://status.solana.com/

---

**Ready to deploy?** Run `./deploy_devnet.sh` and you'll be live in 15 minutes! 🚀
