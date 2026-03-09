---
phase: 3
slug: batch-processing
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-09
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | File verification + frontmatter checks (content processing, no automated test framework) |
| **Config file** | none — structural checks only |
| **Quick run command** | `find CityFarm/ -name "*.md" -not -path "*/.obsidian/*" -not -path "*/_templates/*" -not -name "*MOC*" \| wc -l` |
| **Full suite command** | `for d in CityFarm/0*/; do echo "$d: $(ls "$d"/*.md 2>/dev/null \| grep -v MOC \| wc -l) notes"; done` |
| **Estimated runtime** | ~2 seconds |

---

## Sampling Rate

- **After every task commit:** Quick note count in target folder + frontmatter spot-check on 1 random note
- **After every plan wave:** Full note count per folder, verify `source: module-NN` present in all notes
- **Before `/gsd:verify-work`:** Full suite must be green, total vault note count 100+
- **Max feedback latency:** 2 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 3-01-01 | 01 | 1 | PROC-02 | smoke | `ls CityFarm/01-Основы/*.md CityFarm/02-Системы-выращивания/*.md 2>/dev/null \| grep -v MOC \| wc -l` | N/A — Wave 0 | ⬜ pending |
| 3-02-01 | 02 | 1 | PROC-03 | smoke | `ls CityFarm/04-Культуры/*.md CityFarm/09-Устойчивое-развитие/*.md 2>/dev/null \| grep -v MOC \| wc -l` | N/A — Wave 0 | ⬜ pending |
| 3-03-01 | 03 | 2 | PROC-04 | smoke | `ls CityFarm/07-Проектирование/*.md CityFarm/05-Микроклимат/*.md CityFarm/06-Освещение/*.md 2>/dev/null \| grep -v MOC \| wc -l` | N/A — Wave 0 | ⬜ pending |
| 3-04-01 | 04 | 2 | PROC-05 | smoke | `ls CityFarm/08-Помещения/*.md 2>/dev/null \| grep -v MOC \| wc -l` | N/A — Wave 0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Processing logs for each module group (created during execution)
- [ ] Glossary categories for new domains (lighting, microclimate, crops, premises)

*Existing infrastructure covers core requirements — Wave 0 only adds tracking artifacts.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Content quality of extracted notes | PROC-02..05 | Semantic content cannot be verified by automation | Spot-check 2-3 random notes per module for accuracy |
| Frontmatter schema consistency | PROC-02..05 | YAML field values need human judgment | Verify tags, source, status fields present and sensible |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 2s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
