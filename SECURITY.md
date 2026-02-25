# Security Guidelines

⚠️ **Trading involves financial risk. Please read this document carefully.**

## 🔒 Protecting Your Funds

### Private Keys

- **NEVER** commit private keys to git
- **NEVER** share private keys with anyone
- **ALWAYS** use environment variables or secure vaults
- **CONSIDER** using hardware wallets for significant amounts

```bash
# Good: Use environment variables
export PRIVATE_KEY="your_key_here"

# Bad: Hardcoded in code
const PRIVATE_KEY: &str = "0x...";
```

### API Keys

- Rotate API keys regularly
- Use read-only keys where possible
- Monitor API key usage
- Revoke compromised keys immediately

### Environment Files

```bash
# .env - NEVER COMMIT THIS FILE
echo ".env" >> .gitignore
echo ".env.local" >> .gitignore
echo "*.key" >> .gitignore
```

## 🛡️ Safety Features

Always enable these in production:

### 1. Kill Switch

```rust
use risk_management::KillSwitch;

let kill_switch = KillSwitch::new()
    .balance_floor(100.0)           // Stop if balance < $100
    .max_positions(10);              // Max 10 open positions

if !kill_switch.check().passed {
    panic!("Kill switch triggered!");
}
```

### 2. Circuit Breaker

```rust
use risk_management::CircuitBreaker;

let mut cb = CircuitBreaker::new()
    .max_consecutive_losses(3)       // Stop after 3 losses
    .cooldown_duration(Duration::minutes(30));
```

### 3. Position Limits

```rust
use risk_management::{PositionSizer, FixedFractionalSizing};

let sizer = PositionSizer::new(
    FixedFractionalSizing::conservative()  // 1% risk per trade
)
.with_max_size(1000.0);  // Max $1000 per trade
```

## 🧪 Testing Before Live Trading

### 1. Paper Trading

Always test with paper trading first:

```rust
// Use testnet endpoints
let client = PolymarketClient::new(Chain::PolygonTestnet);

// Or dry-run mode
let config = TradingConfig {
    dry_run: true,
    ..Default::default()
};
```

### 2. Small Amounts

When going live, start with minimal amounts:

- Initial test: $10-50
- Gradual increase only after proven performance
- Never exceed your risk tolerance

### 3. Monitoring

Set up comprehensive monitoring:

```rust
// Alert on significant events
if drawdown > 5.0 {
    bot.send_alert("High drawdown detected!").await?;
}
```

## 🔐 Secure Deployment

### VPS/Cloud

- Use firewalls (allow only necessary ports)
- Enable 2FA on all accounts
- Regular security updates
- Monitor access logs

### Docker

```dockerfile
# Run as non-root user
USER 1000:1000

# Don't embed secrets
ENV PRIVATE_KEY=""
```

### Secrets Management

Consider using:
- [HashiCorp Vault](https://www.vaultproject.io/)
- [AWS Secrets Manager](https://aws.amazon.com/secrets-manager/)
- [1Password Secrets Automation](https://1password.com/secrets/)

## ⚠️ Common Pitfalls

### 1. Race Conditions

```rust
// Bad: Multiple bots competing
// Good: Use file locks or distributed locks
let _lock = FileLock::new("trading.lock")?;
```

### 2. API Rate Limits

```rust
// Always handle rate limits
match client.place_order(order).await {
    Ok(result) => { /* ... */ }
    Err(Error::RateLimit) => {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

### 3. Slippage Protection

```rust
// Set maximum slippage
let order = OrderRequest::buy(token, size, price)
    .with_max_slippage(0.01);  // 1% max slippage
```

## 🚨 Emergency Procedures

### If You Suspect Compromise

1. **Immediately revoke all API keys**
2. **Transfer funds to a new wallet**
3. **Review all recent transactions**
4. **Change all passwords**
5. **Report to exchanges if needed**

### Kill Switch Activation

```rust
// Manual kill switch
kill_switch.manual_trigger("Emergency stop");
```

## 📞 Security Contacts

- Report vulnerabilities: security@example.com
- Do NOT open public issues for security bugs

## 📚 Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Blockchain Security Best Practices](https://consensys.github.io/smart-contract-best-practices/)
- [Exchange API Security](https://docs.kraken.com/rest/#section/Security)

---

**Remember**: No trading bot is perfect. Always monitor your funds and be prepared to intervene manually.
