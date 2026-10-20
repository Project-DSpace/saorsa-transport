# Security policy

## Reporting a vulnerability

Please don't report security vulnerabilities in public issues, pull requests or
discussions.

Report them privately through GitHub instead: open this repository's
[Security tab](https://github.com/Project-DSpace/saorsa-transport/security) and choose
**Report a vulnerability**, or go straight to
<https://github.com/Project-DSpace/saorsa-transport/security/advisories/new>.
Only you and the maintainers can see the report and the discussion on it.

Please include:

- what is affected: the component, and the version or commit you tested
- how to reproduce it
- what an attacker could do with it
- a fix, if you have one in mind

## Known security considerations

### NAT Traversal

The NAT traversal functionality introduces some security considerations:

- **Hole Punching**: Can potentially be abused for port scanning
- **Address Discovery**: Reveals network topology information
- **Relay Services**: Trust boundaries must be carefully managed

Mitigations are implemented but users should be aware of these aspects.

### Raw Public Keys

When using Raw Public Keys (RFC 7250):
- Proper key management is critical
- No certificate chain validation
- Application must verify key authenticity
