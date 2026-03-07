# Branch Protection Rules

> **Secure your code with enforced policies**

---

## 📋 Required Settings

### Main Branch Protection

```yaml
# .github/branch-protection.yml (conceptual)
branch_protection:
  main:
    required_status_checks:
      strict: true
      contexts:
        - ci/tests
        - ci/build
        - ci/security-scan
        - ci/lint
    
    required_pull_request_reviews:
      dismiss_stale_reviews: true
      require_code_owner_reviews: true
      required_approving_review_count: 2
    
    enforce_admins: true
    required_linear_history: true
    allow_force_pushes: false
    allow_deletions: false
    
    restrictions:
      users: []
      teams:
        - core-team
```

---

## 🛡️ Protection Levels

### Critical Branches (main, release/*)

| Rule | Setting |
|------|---------|
| Require PR | ✅ Yes |
| Required reviewers | 2 |
| Dismiss stale reviews | ✅ Yes |
| Require CODEOWNERS | ✅ Yes |
| Require status checks | ✅ Yes |
| Require linear history | ✅ Yes |
| Allow force push | ❌ No |
| Allow delete | ❌ No |
| Enforce for admins | ✅ Yes |

### Feature Branches (feature/*)

| Rule | Setting |
|------|---------|
| Require PR | ✅ Yes |
| Required reviewers | 1 |
| Require status checks | ✅ Yes |
| Allow force push | ❌ No |
| Allow delete | ❌ No |

### Development Branches (develop, staging)

| Rule | Setting |
|------|---------|
| Require PR | ✅ Yes |
| Required reviewers | 1 |
| Require status checks | ✅ Yes |
| Allow force push | ❌ No |

---

## 👥 CODEOWNERS

```gitignore
# .github/CODEOWNERS

# Default owners
* @vantisCorp/core-team

# Core
/core/ @vantisCorp/core-team
/ui/ @vantisCorp/ui-team

# Security
/security/ @vantisCorp/security-team
**/security*.rs @vantisCorp/security-team

# Infrastructure
/.github/ @vantisCorp/devops-team
/Dockerfile @vantisCorp/devops-team
/docker-compose.yml @vantisCorp/devops-team

# Documentation
/docs/ @vantisCorp/docs-team
*.md @vantisCorp/docs-team

# CI/CD
/.github/workflows/ @vantisCorp/devops-team
```

---

## 🔐 Signing Requirements

### GPG Signing

```bash
# Configure GPG signing
git config --global commit.gpgsign true
git config --global gpg.program gpg
git config --global user.signingkey YOUR_GPG_KEY

# Create signed commit
git commit -S -m "feat: new feature"
```

### SSH Signing (Alternative)

```bash
# Configure SSH signing
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
```

---

## 📊 Status Checks Required

| Check | Purpose | Timeout |
|-------|---------|---------|
| `ci/tests` | Unit + integration tests | 15 min |
| `ci/build` | Build verification | 10 min |
| `ci/security-scan` | Security analysis | 20 min |
| `ci/lint` | Code style check | 5 min |
| `ci/coverage` | Test coverage | 10 min |

---

## 🚨 Violation Handling

### Automatic Actions

1. **Failed Status Check**
   - Block merge
   - Notify author
   - Require fix before merge

2. **Missing Reviewers**
   - Block merge
   - Request reviews from CODEOWNERS

3. **Force Push Attempt**
   - Block automatically
   - Log incident

4. **Direct Push to Protected**
   - Block automatically
   - Alert security team

---

## 📅 Review Requirements

### Standard Reviews

| Change Type | Reviewers Required |
|-------------|-------------------|
| Documentation | 1 |
| Bug fix | 1 |
| Feature | 2 |
| Security | 2 + security team |
| Breaking change | 3 |

### Fast-Track Reviews

| Condition | Reviewers Required |
|-----------|-------------------|
| Hotfix | 1 (admin override) |
| Typo fix | 1 |
| Dependency update | 1 (automated) |

---

*Protect your branches, protect your code.*