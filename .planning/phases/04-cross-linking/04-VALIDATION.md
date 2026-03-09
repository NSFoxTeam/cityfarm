---
phase: 4
slug: cross-linking
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-09
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Shell scripts + grep-based validation |
| **Config file** | none — Wave 0 creates validation scripts |
| **Quick run command** | `grep -r '\[\[' CityFarm --include="*.md" \| wc -l` |
| **Full suite command** | `bash .planning/phases/04-cross-linking/validate.sh` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `grep -r '\[\[' CityFarm --include="*.md" | wc -l`
- **After every plan wave:** Run `bash .planning/phases/04-cross-linking/validate.sh`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 4-01-01 | 01 | 1 | LINK-01 | smoke | `grep -rL "Связанные заметки" CityFarm --include="*.md"` | ❌ W0 | ⬜ pending |
| 4-01-02 | 01 | 1 | LINK-01 | smoke | Cross-folder link count > 80 | ❌ W0 | ⬜ pending |
| 4-01-03 | 01 | 1 | LINK-01 | smoke | Broken wiki-link checker | ❌ W0 | ⬜ pending |
| 4-02-01 | 02 | 2 | LINK-02 | smoke | All tags match canonical list | ❌ W0 | ⬜ pending |
| 4-02-02 | 02 | 2 | LINK-02 | smoke | Every note has `домен/*` tag | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `validate.sh` — full validation script (cross-folder link count, broken links, tag compliance)
- [ ] Canonical tag list defined in plan (domain + type + cross-cutting)
- [ ] Broken wiki-link checker (extract `[[targets]]`, verify .md exists)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Links are meaningful (not link-for-linking-sake) | LINK-01 | Semantic judgment | Spot-check 10 random notes: each cross-link should reference a genuinely related concept |
| Navigation depth ≤ 3 clicks | LINK-01 | Path analysis | From Home.md, try reaching 5 random notes in different folders — should take ≤ 3 clicks |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
