---
phase: 2
slug: pilot-processing
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-09
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual review + file verification (content quality — no automated test framework) |
| **Config file** | N/A — content processing, validated by structure checks |
| **Quick run command** | `ls CityFarm/03-Питательные-растворы/*.md | wc -l` |
| **Full suite command** | `find CityFarm/ -name "*.md" -not -path "*/.obsidian/*" -not -path "*/_templates/*" | wc -l && grep -rl "source: module-03" CityFarm/ | wc -l` |
| **Estimated runtime** | ~2 seconds |

---

## Sampling Rate

- **After every task commit:** Run `ls CityFarm/03-Питательные-растворы/*.md | wc -l`
- **After every plan wave:** Run full suite command
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | PROC-01a | smoke | `ls CityFarm/03-Питательные-растворы/*.md | wc -l` (expect 25+) | ❌ W0 | ⬜ pending |
| 02-01-02 | 01 | 1 | PROC-01b | smoke | `grep -l "^type: knowledge\|^type: parameter\|^type: reference" CityFarm/03-Питательные-растворы/*.md | wc -l` | ❌ W0 | ⬜ pending |
| 02-01-03 | 01 | 1 | PROC-01c | smoke | `grep -l "type: parameter\|type: reference" CityFarm/03-Питательные-растворы/*.md` | ❌ W0 | ⬜ pending |
| 02-01-04 | 01 | 1 | PROC-01d | smoke | `test -f .planning/phases/02-pilot-processing/processing-prompt-template.md && echo OK` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Processing prompt template — must be created and validated during first task
- [ ] Processing log — track which lessons produced which notes

*Existing infrastructure covers vault templates and glossary from Phase 1.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Content quality of atomic notes | PROC-01 | STT cleanup quality requires human judgement | Review 3 random notes: check grammar, completeness, accurate parameters |
| Cross-topic placement correctness | PROC-01 | Topic classification needs domain knowledge | Verify notes in 01-Основы/ and 07-Проектирование/ match their topic |
| Numeric parameter accuracy | PROC-01c | Values must match source transcripts | Cross-check 5 random parameters against МОДУЛЬ_3_СВОДКА.txt |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
