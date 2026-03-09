---
phase: 1
slug: vault-foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-09
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Shell smoke tests (file-based vault, no runtime) |
| **Config file** | none — file existence checks only |
| **Quick run command** | `ls -la CityFarm/.obsidian/plugins/*/main.js && echo "plugins present"` |
| **Full suite command** | `find CityFarm/ -name "*.md" \| wc -l && cat CityFarm/.obsidian/community-plugins.json` |
| **Estimated runtime** | ~1 second |

---

## Sampling Rate

- **After every task commit:** Run `ls -la CityFarm/.obsidian/plugins/*/main.js && echo "plugins present"`
- **After every plan wave:** Run `find CityFarm/ -name "*.md" | wc -l && cat CityFarm/.obsidian/community-plugins.json`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 1 second

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | VAULT-01 | smoke | `ls -d CityFarm/0[1-9]-*/` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | VAULT-02 | smoke | `head -20 CityFarm/Home.md` | ❌ W0 | ⬜ pending |
| 01-01-03 | 01 | 1 | VAULT-03 | smoke | `head -5 CityFarm/_templates/knowledge-note.md` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | VAULT-04 | smoke | `ls CityFarm/.obsidian/plugins/*/main.js` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 1 | VAULT-05 | smoke | `grep -c "STT-варианты" CityFarm/Glossary.md` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] All files created from scratch — no pre-existing test infrastructure needed
- [ ] Shell commands above serve as automated verification

*Existing infrastructure covers all phase requirements via file existence checks.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Obsidian opens vault correctly | VAULT-01 | Requires Obsidian app | Open CityFarm/ in Obsidian, verify folder tree visible |
| Plugins load and function | VAULT-04 | Requires Obsidian runtime | Open vault → Settings → Community plugins → verify 3 enabled |
| Template creates note with frontmatter | VAULT-03 | Requires Templater runtime | Create new note via template → verify YAML fields populated |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 1s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
