# Maintainers

This document lists the maintainers of the Mobile AI Orchestrator project, organized by TPCF (Tri-Perimeter Contribution Framework) perimeter.

## Perimeter 1: Core Team

**Responsibilities**:
- Architectural decisions
- Security vulnerability handling
- Release management
- Governance and TPCF administration
- Final review authority

| Name | GitHub | Email | Focus Areas |
|------|--------|-------|-------------|
| Jonathan Bowman | [@Hyperpolymath](https://github.com/Hyperpolymath) | hyperpolymath@protonmail.com | Architecture, Rust implementation, RSR compliance, formal verification |

## Perimeter 2: Curated Contributors

**Responsibilities**:
- Code review for Perimeter 3 PRs
- Issue triage and labeling
- Feature development
- Documentation maintenance

| Name | GitHub | Focus Areas |
|------|--------|-------------|
| *Open for applications* | - | Looking for experts in: mobile optimization, reservoir computing, LLM inference |

**How to join**: See [CONTRIBUTING.md](CONTRIBUTING.md) section on "Tri-Perimeter Contribution Framework"

## Perimeter 3: Community Sandbox

**Open to all contributors!**

Current active contributors (5+ merged PRs):

| Name | GitHub | Contributions |
|------|--------|---------------|
| *Be the first!* | - | - |

## Emeritus Maintainers

Former maintainers who have stepped down but retain honorary status:

*None yet*

## Area Experts

Specialists for specific components (may be from any perimeter):

| Area | Expert | Contact |
|------|--------|---------|
| Reservoir Computing | *Seeking expert* | - |
| Mobile Optimization | *Seeking expert* | - |
| LLM Inference | *Seeking expert* | - |
| Formal Verification | Jonathan Bowman | hyperpolymath@protonmail.com |
| Security | *Seeking expert* | - |
| Android Development | *Seeking expert* | - |

## Communication Channels

### Public

- **GitHub Issues**: https://github.com/Hyperpolymath/heterogenous-mobile-computing/issues
- **GitHub Discussions**: https://github.com/Hyperpolymath/heterogenous-mobile-computing/discussions
- **Matrix (future)**: TBD

### Private (Perimeter 1-2 only)

- **Email**: hyperpolymath@protonmail.com
- **Security**: See [SECURITY.md](SECURITY.md)

## Decision-Making Process

### Routine Decisions (Documentation, Bug Fixes, Tests)

- **Who decides**: Any Perimeter 2+ maintainer
- **Process**: Approve PR after review
- **Timeline**: Within 48 hours

### Feature Additions

- **Who decides**: Perimeter 1, with Perimeter 2 input
- **Process**:
  1. Proposal filed as issue
  2. Discussion period (≥7 days)
  3. Perimeter 1 decision
  4. Implementation by anyone
- **Timeline**: 2-4 weeks

### Breaking Changes

- **Who decides**: Perimeter 1 only
- **Process**:
  1. RFC (Request for Comments) filed
  2. Public comment period (≥14 days)
  3. Perimeter 1 consensus decision
  4. Documented in CHANGELOG with migration guide
- **Timeline**: 4-8 weeks

### Security Fixes

- **Who decides**: Perimeter 1, security expert (if available)
- **Process**:
  1. Private disclosure to Perimeter 1
  2. Patch developed in private
  3. Coordinated public disclosure
- **Timeline**: 30 days (standard), expedited for Critical

## Conflict Resolution

### Minor Conflicts (Code Style, Implementation Details)

1. Discuss in PR comments
2. If no consensus → Perimeter 2 vote
3. If tie → Perimeter 1 breaks tie

### Major Conflicts (Architecture, Governance)

1. File issue tagged `[conflict]`
2. Allow 7-day discussion period
3. Perimeter 1 makes final call
4. If irreconcilable → Fork is encouraged (open source!)

### Code of Conduct Violations

1. Report to hyperpolymath@protonmail.com
2. Perimeter 1 investigates (within 72 hours)
3. Decision per CODE_OF_CONDUCT.md enforcement guidelines
4. Appeals: Email with new information

## Succession Planning

### Perimeter 1 Transition

If sole Perimeter 1 member (Jonathan Bowman) is unavailable for >30 days:

1. **Emergency contact**: (TBD - will add when Perimeter 2 exists)
2. **Temporary authority**: Most senior Perimeter 2 member (by contribution count)
3. **Long-term (>90 days)**: Community vote for new Perimeter 1 members

### Adding New Perimeter 1 Members

Requirements:
- ≥2 years of consistent contributions
- Deep domain expertise (Rust, AI, mobile, or formal methods)
- Demonstrated judgment in architectural decisions
- Availability for security response (72-hour SLA)
- Unanimous vote by existing Perimeter 1

## Recognition

### Contribution Metrics

Tracked automatically via GitHub:
- PRs merged
- Issues filed/resolved
- Code review comments
- Documentation additions

View stats: https://github.com/Hyperpolymath/heterogenous-mobile-computing/graphs/contributors

### Awards (Informal)

- 🥉 **Bronze Contributor**: 1-4 merged PRs
- 🥈 **Silver Contributor**: 5-9 merged PRs
- 🥇 **Gold Contributor**: 10+ merged PRs
- 🌟 **Core Contributor**: Perimeter 2 membership
- 🏆 **Founding Member**: Perimeter 1 membership

### Academic Credit

Contributors to research aspects (reservoir computing, hybrid architecture) will be:
- Acknowledged in academic papers
- Invited as co-authors if contribution is substantial (per academic norms)

## Meeting Schedule

### Perimeter 1 Meetings

**Frequency**: As needed (currently no standing meetings)
**Format**: Async (GitHub issues/discussions) preferred
**Sync meetings**: Only for urgent/complex decisions

### Community Meetings (Future)

When project grows beyond ~10 active contributors:
- **Frequency**: Monthly
- **Format**: Video call (recorded for async access)
- **Agenda**: Open to all, published 1 week prior

## Maintainer Responsibilities

### Perimeter 1 (Core Team)

- [ ] Review and merge PRs (target: within 7 days)
- [ ] Triage new issues (within 48 hours)
- [ ] Respond to security reports (within 72 hours)
- [ ] Cut releases (when ready, following semver)
- [ ] Update roadmap (quarterly)
- [ ] Recruit Perimeter 2 members (when appropriate)
- [ ] Enforce Code of Conduct

### Perimeter 2 (Curated Contributors)

- [ ] Review PRs from Perimeter 3 (within 7 days)
- [ ] Help with issue triage
- [ ] Mentor new contributors
- [ ] Maintain documentation
- [ ] Test pre-release builds

## Stepping Down

If a maintainer needs to step down:

1. Notify other maintainers (at least 2 weeks notice if possible)
2. Transfer any in-progress work
3. Move to "Emeritus" status (retains honor, loses commit access)
4. Can return to Perimeter 3 anytime without re-proving

**No explanation needed** - life happens, priorities shift!

## Perimeter Promotion Process

### Perimeter 3 → Perimeter 2

**Criteria**:
- 5+ merged PRs
- Demonstrated expertise in ≥1 area
- Alignment with project values (RSR, safety, privacy)
- Active for ≥3 months

**Process**:
1. Any Perimeter 1-2 member nominates in private channel
2. Perimeter 1 approves (usually quick)
3. Invitation sent with expectations and responsibilities
4. Acceptance grants Perimeter 2 access

### Perimeter 2 → Perimeter 1

**Criteria**:
- ≥2 years active contribution
- Major architectural contributions
- Security clearance (for vulnerability handling)
- Availability commitment

**Process**:
1. Unanimous vote by existing Perimeter 1
2. Formal invitation with governance responsibilities
3. Mentorship period (3 months)
4. Full Perimeter 1 access granted

## Contact

**General inquiries**: File a GitHub issue
**Private matters**: hyperpolymath@protonmail.com
**Security**: See [SECURITY.md](SECURITY.md)

---

*This document is itself governed by Perimeter 1. Changes require PR approval from Perimeter 1 member.*

*Last updated: 2025-11-22*
