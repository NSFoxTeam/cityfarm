# Phase 5: Reference Tables & IoT Integration - Research

**Researched:** 2026-03-09
**Domain:** Obsidian vault content aggregation, IoT sensor-parameter mapping
**Confidence:** HIGH

## Summary

Phase 5 is the capstone of the CityFarm Knowledge Base: aggregating parameter data scattered across 95+ notes into consolidated reference tables, and mapping each parameter to a concrete CityFarm sensor with thresholds and alert actions. The vault already has significant groundwork -- 5 parameter cards (pH, EC, temperature, humidity, CO2) with IoT-маппинг sections populated, plus crop-specific parameter notes. The backend already has `DefaultThresholds` in `thresholds.go` that should be synchronized with vault reference data.

The main work is: (1) creating consolidated cross-crop reference tables that aggregate per-parameter data from all vault sources with source attribution, (2) filling gaps where parameter cards lack IoT mapping or crop-specific breakdowns, (3) ensuring solution temperature (DS18B20) and light intensity (BH1750) have proper reference entries, and (4) adding a developer-facing IoT integration summary that maps the complete sensor inventory to vault knowledge.

**Primary recommendation:** Upgrade existing parameter cards (type: parameter) with crop-specific breakdowns and source attribution, create a consolidated IoT reference note linking all sensors to their vault-backed thresholds, and synchronize with backend `DefaultThresholds`.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| REF-01 | Справочные таблицы параметров -- pH, EC, температура, влажность, освещённость, CO2 с оптимальными диапазонами по культурам | Existing parameter cards provide base data; need crop-specific breakdowns and source attribution. Content analysis below identifies all sources per parameter. |
| REF-02 | Привязка параметров к CityFarm IoT -- маппинг: параметр -> датчик -> порог -> алерт -> действие | 5 of 7 parameter cards already have IoT-маппинг sections; need light/DLI and solution temp cards; need consolidated developer reference; synchronize with backend thresholds.go |
</phase_requirements>

## Standard Stack

This phase is pure Obsidian Markdown content work -- no external libraries needed.

### Core Tools
| Tool | Purpose | Why |
|------|---------|-----|
| Obsidian | Vault editor, wiki-links, templates | Project standard |
| parameter-card template | Structured parameter notes with IoT-маппинг | Already exists in `_templates/parameter-card.md` |
| reference-table template | Aggregated parameter tables | Already exists in `_templates/reference-table.md` |

### Existing Infrastructure
| Asset | Location | Status |
|-------|----------|--------|
| parameter-card template | `_templates/parameter-card.md` | Complete -- has IoT-маппинг section |
| reference-table template | `_templates/reference-table.md` | Complete -- basic table structure |
| Tag taxonomy | 37 tags across 95 notes | Complete (Phase 4) |
| Wiki-links | Cross-folder links | Complete (Phase 4) |

## Architecture Patterns

### Existing Parameter Card Pattern (PROVEN)

The vault already has 5 parameter cards with a consistent structure. This pattern MUST be followed:

```markdown
---
title: "Parameter Name"
type: parameter
topic: topic-name
tags:
  - домен/topic
  - тип/параметр
  - iot
source: module-XX/урок-YY
---

## Описание
[What the parameter is, why it matters]

## Оптимальные значения
| Культура/Условие | Мин | Макс | Единица | Примечание |
|------------------|-----|------|---------|------------|

## IoT-маппинг
| Параметр | Сенсор | Порог | Алерт | Действие |
|----------|--------|-------|-------|----------|

## Источники
- Module X, Lesson Y

## Связанные заметки
- [[wiki-links]]
```

### Existing Parameter Cards Inventory

| Parameter | Note | Has IoT? | Has Crop Breakdown? | Gaps |
|-----------|------|----------|---------------------|------|
| pH | `pH-питательного-раствора.md` | YES | NO (general 5.5-6.5 only) | Need per-crop pH ranges |
| EC | `EC-по-фазам-развития-растений.md` | YES | By growth phase, not crop | Need per-crop EC |
| Temperature (air) | `Температура-воздуха-на-сити-ферме.md` | YES | NO (general 20-30) | Need salad/microgreen specific |
| Humidity (air) | `Влажность-воздуха-на-сити-ферме.md` | YES | NO (general 60-80) | Need crop-specific |
| CO2 | `Концентрация-CO2-на-сити-ферме.md` | YES | NO (general 1000-1500) | No crop-specific data in vault |
| Light (PPFD/lux) | No dedicated parameter card | Partial in DLI note | Salat only (200 PPFD) | Need parameter card |
| Solution temp | No dedicated parameter card | NO | NO | Need parameter card, DS18B20 mapping |

### Crop-Specific Data Already in Vault

| Crop Type | Source Note | Parameters Covered |
|-----------|------------|-------------------|
| Салаты/травы (проращивание) | `Параметры-выращивания-салатов-и-трав.md` | Temp 24-25C, Humidity 70-80%, EC <450ppm, pH 5.5-6.5 |
| Салаты/травы (рост) | Same note | Temp 21-22C, Humidity 50-60%, EC <800ppm, pH 5.5-6.5, 12h light |
| Салат (освещение) | `Оптимальная-интенсивность-освещения-салата.md` | PPFD 200 umol/m2/s, 16h photoperiod |
| Микрозелень | `Особенности-выращивания-микрозелени.md` | 5000 lux, 16-18h, EC 300-400ppm |
| Огурец/томат/перец (DLI) | `Суточная-доза-освещения-DLI.md` | DLI 22-30 mol/m2/day |

### Recommended Structure for New Content

```
CityFarm/
├── 03-Питательные-растворы/
│   ├── pH-питательного-раствора.md          # UPDATE: add crop breakdown
│   └── EC-по-фазам-развития-растений.md     # UPDATE: add crop breakdown
├── 05-Микроклимат/
│   ├── Температура-воздуха-на-сити-ферме.md  # UPDATE: add crop breakdown
│   ├── Влажность-воздуха-на-сити-ферме.md    # UPDATE: add crop breakdown
│   ├── Концентрация-CO2-на-сити-ферме.md     # UPDATE: stays general (no per-crop data)
│   └── Температура-питательного-раствора.md  # NEW: DS18B20 parameter card
├── 06-Освещение/
│   └── Интенсивность-освещения.md            # NEW: consolidated light parameter card
└── Home.md                                    # UPDATE: add reference section
```

Plus one new consolidated note:
- `CityFarm/IoT-маппинг-CityFarm.md` -- developer-facing reference that aggregates ALL sensor-parameter mappings in one place

### Anti-Patterns to Avoid

- **Duplicating data across notes:** Crop-specific values should be added to EXISTING parameter cards, not creating new parallel notes. The parameter card is the single source of truth.
- **Inventing threshold values:** Only use values explicitly stated in vault sources (course material). If the vault doesn't have crop-specific CO2 ranges, don't make them up.
- **Disconnecting from backend code:** The IoT mapping notes should reference the actual sensor names/pins from `thresholds.go` and firmware config, not abstract sensor names.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cross-crop parameter matrix | Complex Dataview queries | Static Markdown tables | Dataview JS disabled (security); static tables are sufficient for 3-4 crop types |
| IoT threshold config | Auto-generated from vault | Manual synchronization + comments | Vault is reference, code is runtime; human review needed for threshold changes |
| Lux-to-PPFD conversion | Custom formula | Document conversion coefficients per light type | Coefficient varies by spectrum; no universal formula |

## Common Pitfalls

### Pitfall 1: Threshold Mismatch Between Vault and Backend
**What goes wrong:** Vault says pH warning at 5.5, backend code says different value, developer doesn't know which is authoritative.
**Why it happens:** Two sources of truth diverge over time.
**How to avoid:** IoT mapping note explicitly states "canonical values in backend thresholds.go" and vault documents the agronomic basis. Include a comparison table showing vault recommendations vs current backend defaults.
**Warning signs:** Different numbers for the same parameter in vault vs code.

### Pitfall 2: Missing Source Attribution
**What goes wrong:** Reference table has numeric ranges but nobody knows which lesson/module they came from, making validation impossible.
**Why it happens:** Values aggregated without tracking origin.
**How to avoid:** Every numeric value in reference tables MUST have a source citation (module/lesson or note name).

### Pitfall 3: Lux vs PPFD Confusion
**What goes wrong:** BH1750 outputs lux, but plant science uses PPFD (umol/m2/s). Mixing units leads to wrong threshold comparisons.
**Why it happens:** BH1750 measures visible light weighted for human eye, not photosynthetically active radiation.
**How to avoid:** IoT mapping for light explicitly states: "BH1750 measures lux. Conversion to PPFD requires coefficient dependent on light source spectrum. For white LED: ~1 lux = 0.015 umol/m2/s. For sunlight: ~1 lux = 0.0185 umol/m2/s." Reference tables should provide both lux and PPFD where available.
**Warning signs:** Comparing PPFD research values directly to lux sensor readings.

### Pitfall 4: Solution Temperature vs Air Temperature
**What goes wrong:** Confusing air temperature thresholds with nutrient solution temperature thresholds.
**Why it happens:** Both measured in Celsius, different sensors (DHT22 vs DS18B20), different optimal ranges.
**How to avoid:** Separate parameter cards, different sensor mappings. DS18B20 (GPIO22, 1-Wire) for solution; DHT22 (GPIO4) for air.

### Pitfall 5: CO2 Sensor Not Yet Installed
**What goes wrong:** IoT mapping references MH-Z19B/SCD40 but neither is currently connected to RPi.
**Why it happens:** CO2 sensor proposed but not yet purchased/installed.
**How to avoid:** Mark CO2 IoT mapping as "planned" status, not "active." IoT note should distinguish between operational and planned sensors.

## Code Examples

Not applicable -- this phase is pure Markdown content. See Architecture Patterns for templates.

## Key Data: Backend DefaultThresholds vs Vault Values

This comparison is critical for REF-02 (IoT integration):

| Parameter | Backend Key | Backend Warn | Backend Crit | Vault Optimal | Source |
|-----------|-------------|-------------|-------------|---------------|--------|
| pH | `ph` | 5.5 / 6.5 | 5.0 / 7.0 | 5.5-6.5 | pH-питательного-раствора.md |
| TDS | `tds` | 400 / 800 | 300 / 1000 | Varies by phase | EC-по-фазам-развития-растений.md |
| EC | `ec` | 0.8 / 1.6 | 0.5 / 2.0 | 0.8-2.5 by phase | EC-по-фазам-развития-растений.md |
| Solution Temp | `solution_temp` | 18 / 26 | 15 / 30 | Not in vault yet | DS18B20 |
| Air Temp | `temperature` | 15 / 35 | 10 / 40 | 20-30 day, 15-20 night | Температура-воздуха |
| Humidity | `humidity` | 30 / 80 | 20 / 90 | 60-80 general | Влажность-воздуха |
| Moisture | `moisture` | 20 / 80 | 10 / 90 | Not in vault | YL-69 soil moisture |
| Light (lux) | Not in backend | N/A | N/A | 5000 lux (microgreen) | Особенности-выращивания |
| CO2 | Not in backend | N/A | N/A | 1000-1500 ppm | Концентрация-CO2 |

**Key findings:**
- Backend has `moisture` threshold but vault has no soil moisture parameter card
- Backend lacks `light` and `co2` thresholds -- IoT note should flag these as needed additions
- Backend `ec` warn range (0.8-1.6) maps to vault's seedling-vegetation phases but misses later phases (up to 2.5)
- Backend `temperature` thresholds (warn 15/35) are very wide compared to vault optimal (20-30)

## Complete CityFarm Sensor Inventory

For IoT integration reference:

| Sensor | Measures | Interface | Address/Pin | Status |
|--------|----------|-----------|-------------|--------|
| DHT22 | Air temp + humidity | GPIO4 (single-wire) | -- | Active |
| BH1750 x3 | Light (lux) | I2C via TCA9548A ch0-2 | 0x23 | Active |
| ADS1115 | ADC (4 ch) | I2C via TCA9548A ch7 | 0x48 | Active |
| YL-69 | Soil moisture | Analog via ADS1115 A0 | -- | Active |
| DFRobot pH V2 | pH | Analog via ADS1115 A1 | -- | Active (pending calibration) |
| DFRobot TDS | TDS/EC | Analog via ADS1115 A2 | -- | Active (pending calibration) |
| DS18B20 | Solution temp | GPIO22 (1-Wire) | -- | Active |
| Relay | Water pump | GPIO17 (active LOW) | -- | Active |
| MH-Z19B / SCD40 | CO2 | -- | -- | Planned (not installed) |
| Camera | Plant health | RTSP | -- | Active (YOLO pipeline) |

## State of the Art

| Old Approach | Current Approach | Impact |
|--------------|------------------|--------|
| Scattered values across 95 notes | Consolidated parameter cards | Phase 5 aggregates into reference tables |
| Generic thresholds in code | Vault-backed agronomic thresholds | Backend thresholds should cite vault rationale |
| No source attribution | Module/lesson citations | Enables validation against course PDFs |

## Open Questions

1. **Soil moisture (YL-69) reference values**
   - What we know: Backend has thresholds (warn 20-80%, crit 10-90%), sensor is connected
   - What's unclear: Vault has NO soil moisture parameter content from the course
   - Recommendation: Create a minimal parameter card with backend defaults, mark as "project-derived" not course-sourced. Or skip if out of scope for course-based vault.

2. **Solution temperature optimal range**
   - What we know: Backend defaults 18-26C (warn), DS18B20 connected on GPIO22
   - What's unclear: Course material doesn't explicitly cover solution temperature ranges in dedicated notes
   - Recommendation: Create parameter card based on general hydroponic knowledge (18-22C optimal), mark confidence as LOW, note that value needs validation. The `Коррекция-питательного-раствора.md` note mentions temperature affecting concentration needs but gives no specific range.

3. **Light threshold for backend**
   - What we know: BH1750 active, vault has PPFD/lux data, backend has no light threshold
   - What's unclear: Whether light thresholds should be added to backend thresholds.go as part of this phase
   - Recommendation: Document the vault-recommended thresholds in IoT note, flag as "backend TODO" -- actual code changes are out of scope for vault phase.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual validation (Obsidian vault content) |
| Config file | N/A |
| Quick run command | `grep -r "type: parameter" CityFarm/ --include="*.md" \| wc -l` |
| Full suite command | Manual checklist review |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| REF-01 | Reference tables exist for all 6 parameters with crop breakdowns | manual | `grep -c "Оптимальные значения" CityFarm/**/*.md` | N/A |
| REF-01 | Source attribution on every numeric value | manual | `grep -c "Источники" CityFarm/**/*.md` | N/A |
| REF-02 | IoT mapping for each parameter | manual | `grep -c "IoT-маппинг" CityFarm/**/*.md` | N/A |
| REF-02 | Consolidated developer reference note exists | manual | `test -f CityFarm/IoT-маппинг-CityFarm.md` | Wave 0 |

### Sampling Rate
- **Per task commit:** Verify updated/created notes have correct frontmatter and required sections
- **Per wave merge:** Count parameter cards with IoT sections, verify all 7 parameters covered
- **Phase gate:** All 6 required parameters (pH, EC, temp, humidity, light, CO2) have reference tables + IoT mapping

### Wave 0 Gaps
- [ ] `CityFarm/05-Микроклимат/Температура-питательного-раствора.md` -- new parameter card for DS18B20
- [ ] `CityFarm/06-Освещение/Интенсивность-освещения.md` -- consolidated light parameter card
- [ ] `CityFarm/IoT-маппинг-CityFarm.md` -- developer-facing IoT summary note

## Sources

### Primary (HIGH confidence)
- Existing vault notes (read directly): pH, EC, temperature, humidity, CO2, DLI, light intensity, crop parameters
- Backend source code: `backend/internal/alerting/thresholds.go` -- actual threshold values
- Firmware source code: `firmware/src/config.rs` -- sensor configuration
- Templates: `_templates/parameter-card.md`, `_templates/reference-table.md`
- CLAUDE.md -- hardware pin assignments and sensor inventory

### Secondary (MEDIUM confidence)
- STATE.md decisions about BH1750 lux-not-PPFD, MH-Z19B/SCD40 for CO2
- Solution temperature ranges (18-22C) from general hydroponic practice (not explicitly in vault)

### Tertiary (LOW confidence)
- Soil moisture reference values -- backend defaults only, no course content
- Lux-to-PPFD conversion coefficients -- varies by light source, approximate values cited

## Metadata

**Confidence breakdown:**
- Existing parameter cards: HIGH -- read directly from vault, well-structured
- Crop-specific data availability: HIGH -- identified all sources in vault
- Backend threshold synchronization: HIGH -- read thresholds.go directly
- Solution temperature ranges: MEDIUM -- backend has defaults, vault lacks dedicated content
- Soil moisture references: LOW -- no course content, backend defaults only

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable -- vault content and hardware don't change rapidly)
