# Branch Protection Rules

This document describes the recommended branch protection rules for the Vantis Media Player repository.

## Recommended Branch Protection Configuration

Since this is a public repository, the following branch protection rules are recommended:

### Main Branch Protection

1. **Require a pull request before merging**
   - Require approvals: 1 (minimum)
   - Dismiss stale pull request approvals when new commits are pushed
   - Require review from Code Owners (if applicable)

2. **Require status checks to pass before merging**
   - Require branches to be up to date before merging
   - Status checks that are required:
     - `build` (from CI workflow)
     - `test` (from Testing workflow)
     - `security` (from Security workflow)

3. **Require conversation resolution before merging**
   - All conversations must be resolved

4. **Require signed commits** (optional but recommended)
   - Commits must have verified signatures

5. **Require linear history**
   - Prevent merge commits

6. **Include administrators**
   - Apply these rules to administrators as well

7. **Restrict force pushes**
   - Block force pushes to the branch

8. **Allow deletions**
   - Do not allow branch deletions

### How to Configure

1. Go to repository Settings → Branches
2. Click "Add rule"
3. Branch name pattern: `main`
4. Check the following options:
   - ✅ Require a pull request before merging
   - ✅ Require approvals (1)
   - ✅ Dismiss stale pull request approvals when new commits are pushed
   - ✅ Require status checks to pass before merging
   - ✅ Require branches to be up to date before merging
   - ✅ Require conversation resolution before merging
   - ✅ Restrict force pushes
   - ✅ Do not allow deletions

### Additional Recommendations

For public repositories:

1. **Enable Dependabot alerts** (Settings → Security & analysis)
2. **Enable Dependabot security updates**
3. **Enable secret scanning**
4. **Enable push protection**
5. **Enable private vulnerability reporting**

## GitHub CLI Commands

To configure branch protection via CLI (requires admin permissions):

```bash
gh api -X PUT repos/vantisCorp/Vantis-Media-Player/branches/main/protection \
  -f required_pull_request_reviews='{"dismiss_stale_reviews":true,"require_code_owner_reviews":true,"required_approving_review_count":1}' \
  -f required_status_checks='{"strict":true,"contexts":["build","test","security"]}' \
  -f enforce_admins=true \
  -f restrictions=null \
  -f required_linear_history=true \
  -f allow_force_pushes=false \
  -f allow_deletions=false
```

## Security Best Practices

1. Never commit secrets or credentials
2. Always use environment variables or GitHub Secrets
3. Review all pull requests carefully
4. Use signed commits when possible
5. Keep dependencies up to date
6. Monitor security alerts