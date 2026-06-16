# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

The Stellar Launchpad Core team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to [security@stellar-launchpad.org](mailto:security@stellar-launchpad.org).

Please include the following information in your report:

- Type of issue (e.g. smart contract vulnerability, dependency vulnerability, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

### Response Timeline

- **Initial Response**: We will acknowledge receipt of your vulnerability report within 48 hours.
- **Status Updates**: We will send you regular updates about our progress in resolving the vulnerability.
- **Resolution**: We aim to resolve critical vulnerabilities within 7 days, high severity within 30 days.

### Safe Harbor

We support safe harbor for security researchers who:

- Make a good faith effort to avoid privacy violations, destruction of data, and interruption or degradation of our services
- Only interact with accounts you own or with explicit permission of the account holder
- Do not access a system or account beyond what is necessary to demonstrate a security vulnerability
- Report vulnerability information to us as soon as possible
- Do not violate any other applicable laws or regulations

## Security Best Practices

### For Smart Contract Auditors

When auditing our smart contracts, please pay special attention to:

1. **Authentication and Authorization**
   - All admin functions require proper `require_auth()` calls
   - Check for privilege escalation vulnerabilities
   - Verify that only authorized accounts can perform sensitive operations

2. **Input Validation**
   - All user inputs are properly validated
   - Check for integer overflow/underflow vulnerabilities
   - Verify proper bounds checking on arrays and maps

3. **Business Logic**
   - Vesting calculations are mathematically correct
   - Airdrop distribution logic prevents double-claiming
   - Token minting respects supply caps and permissions

4. **Reentrancy**
   - Check for potential reentrancy attacks in cross-contract calls
   - Verify state changes occur before external calls

5. **Gas Optimization**
   - Look for potential denial-of-service via gas exhaustion
   - Check for inefficient loops that could be exploited

### For Integration Partners

When integrating with our contracts:

1. **Contract Verification**
   - Always verify contract addresses against official documentation
   - Use official contract ABIs from our repository

2. **Error Handling** 
   - Implement proper error handling for all contract calls
   - Never assume contract calls will always succeed

3. **Access Control**
   - Implement proper access controls in your integration
   - Don't rely solely on our contract's access controls

4. **Testing**
   - Test thoroughly on testnets before mainnet deployment
   - Include edge cases and error conditions in your tests

## Known Security Considerations

### Smart Contract Limitations

1. **Oracle Dependencies**: If using external price oracles, be aware of oracle manipulation risks
2. **Upgrade Mechanisms**: Contracts are immutable after deployment - plan migrations carefully
3. **Cross-Contract Calls**: Be cautious about composability risks when calling external contracts

### Operational Security

1. **Key Management**: Use hardware wallets or secure key management for admin operations
2. **Multi-Signature**: Consider using multi-signature wallets for high-value operations
3. **Time Delays**: Important operations may benefit from time delays or governance processes

## Security Audits

This project undergoes regular security audits. Audit reports are published at:
- [Audit Reports Repository](https://github.com/stellar-launchpad/audit-reports)

## Bug Bounty Program

We may run bug bounty programs for critical security issues. Details will be announced on:
- [Official Website](https://stellar-launchpad.org)
- [Twitter](https://twitter.com/stellar_launchpad)

## Contact

For security-related questions that don't constitute vulnerabilities, you can reach us at:
- Email: [security@stellar-launchpad.org](mailto:security@stellar-launchpad.org)
- Discord: [Stellar Developer Community](https://discord.gg/stellar-dev)

---

Thank you for helping keep Stellar Launchpad Core and our users safe!