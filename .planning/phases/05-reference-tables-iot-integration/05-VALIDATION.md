---
phase: 5
slug: reference-tables-iot-integration
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-09
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Manual validation (Obsidian vault content) |
| **Config file** | N/A |
| **Quick run command** | `grep -r "type: parameter" CityFarm/ --include="*.md" \| wc -l` |
| **Full suite command** | Manual checklist review |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Verify updated/created notes have correct frontmatter and required sections
- **After every plan wave:** Count parameter cards with IoT sections, verify all parameters covered
- **Before `/gsd:verify-work`:** All 6 required parameters (pH, EC, temp, humidity, light, CO2) have reference tables + IoT mapping
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 05-01-01 | 01 | 1 | REF-01 | manual | `grep -c "Оптимальные значения" CityFarm/**/*.md` | N/A | ⬜ pending |
| 05-01-02 | 01 | 1 | REF-01 | manual | `grep -c "Источники" CityFarm/**/*.md` | N/A | ⬜ pending |
| 05-02-01 | 02 | 2 | REF-02 | manual | `grep -c "IoT-маппинг" CityFarm/**/*.md` | N/A | ⬜ pending |
| 05-02-02 | 02 | 2 | REF-02 | manual | `test -f CityFarm/IoT-маппинг-CityFarm.md` | Wave 0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `CityFarm/05-Микроклимат/Температура-питательного-раствора.md` — new parameter card for DS18B20
- [ ] `CityFarm/06-Освещение/Интенсивность-освещения.md` — consolidated light parameter card
- [ ] `CityFarm/IoT-маппинг-CityFarm.md` — developer-facing IoT summary note

*Wave 0 creates missing notes before main work begins.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Parameter tables have crop-specific ranges | REF-01 | Content quality check | Review each parameter card for crop breakdown section |
| Source attribution on numeric values | REF-01 | Content accuracy | Spot-check 3+ entries for module/lesson references |
| IoT mapping completeness | REF-02 | Cross-reference with hardware | Compare vault IoT sections to CLAUDE.md sensor list |
| Developer can find threshold values | REF-02 | UX validation | Simulate developer lookup for pH threshold |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
