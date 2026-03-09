# Phase 4: Cross-Linking - Research

**Researched:** 2026-03-09
**Domain:** Obsidian wiki-links, tag taxonomy, knowledge graph connectivity
**Confidence:** HIGH

## Summary

The CityFarm vault contains 95 content notes + 9 MOCs + Home.md + Glossary.md across 9 topic folders. Current state: most notes (84/95) already have a `## Связанные заметки` section with wiki-links, but links are overwhelmingly intra-folder (238 intra-folder vs only 23 cross-folder links). 11 notes lack the section entirely. Tags exist on all 95 content notes (267 unique tags), but are ad-hoc topic keywords without a consistent taxonomy -- no domain prefixes, no content-type tags, no status tags in the tag list.

The phase needs two distinct workstreams: (1) add meaningful cross-folder wiki-links where topics genuinely reference each other, and (2) normalize the tag system into a hierarchical taxonomy with domain, content-type, and status dimensions. MOCs already provide good Home -> section navigation; cross-folder links will close the "reach any note in 2-3 clicks" gap.

**Primary recommendation:** Process folder-by-folder, adding cross-folder `[[wiki-links]]` in the `## Связанные заметки` section and inline where natural, then apply a unified tag taxonomy as a separate pass across all notes.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| LINK-01 | Wiki-ссылки `[[ссылка]]` между связанными заметками по всему vault | Current state: 238 intra-folder links, only 23 cross-folder. 11 notes missing `## Связанные заметки` section. Research identifies cross-folder link candidates by topic overlap. |
| LINK-02 | Система тегов для категоризации заметок (домен, тип контента, статус) | Current state: 267 unique ad-hoc tags, no taxonomy. Research defines 3-tier tag system: domain, content-type, status. |
</phase_requirements>

## Standard Stack

Not applicable -- this phase is pure Obsidian Markdown editing (no code libraries). Tools used:

| Tool | Purpose | Why |
|------|---------|-----|
| Claude Code + Read/Edit | Batch-edit .md files | Fast, consistent, can process all 95+ notes |
| Obsidian wiki-link syntax | `[[Note-Name]]` or `[[Note-Name|Display Text]]` | Project standard per CLAUDE.md |
| YAML frontmatter tags | `tags:` array in frontmatter | Already used in all notes |

## Architecture Patterns

### Current Vault Structure
```
CityFarm/
  Home.md                    # navigation hub -> MOCs
  Glossary.md                # terminology reference
  _templates/                # knowledge-note, parameter-card, reference-table
  01-Основы/         (6 notes + MOC)    # CEA basics
  02-Системы-выращивания/ (7 notes + MOC) # hydroponic systems
  03-Питательные-растворы/ (22 notes + MOC) # nutrients, pH, EC
  04-Культуры/       (13 notes + MOC)   # crops, microgreens
  05-Микроклимат/    (7 notes + MOC)    # temp, humidity, CO2
  06-Освещение/      (19 notes + MOC)   # light, spectrum, LED
  07-Проектирование/ (6 notes + MOC)    # system design, assembly
  08-Помещения/      (9 notes + MOC)    # premises, infrastructure
  09-Устойчивое-развитие/ (6 notes + MOC) # sustainability
```

### Pattern 1: Cross-Folder Linking by Topic Overlap

**What:** Notes in different folders that share concepts should link to each other.

**Key cross-folder relationship clusters identified:**

| Cluster | Folders Involved | Example Links |
|---------|-----------------|---------------|
| pH/EC parameters <-> cultures | 03 <-> 04 | `EC-по-фазам` -> `Параметры-выращивания-салатов-и-трав` |
| Growing systems <-> system design | 02 <-> 07 | `NFT-техника` -> `Компоненты-проточной-гидропонной-системы` |
| Light parameters <-> cultures | 06 <-> 04 | `Оптимальная-интенсивность-освещения-салата` -> `Салатные-культуры-для-гидропоники` |
| Microclimate <-> premises | 05 <-> 08 | `Температура-воздуха` -> `Вентиляция-сити-фермы` |
| Water quality <-> sustainability | 03 <-> 09 | `Качество-воды-для-гидропоники` -> `Водосбережение-на-предприятии` |
| Basics <-> all domains | 01 <-> all | `Контролируемые-параметры-среды` -> parameter cards in 03,05,06 |
| Substrates <-> sustainability | 04 <-> 09 | `Субстраты-для-салатов-и-трав` -> `Экологичные-субстраты-и-упаковка` |
| LED/energy <-> sustainability | 06 <-> 09 | `Преимущества-LED-перед-НЛВД` -> `Энергосбережение-на-предприятии` |
| Microclimate <-> lighting | 05 <-> 06 | `Температура-воздуха` -> `Преимущества-LED-перед-НЛВД` (LED heat) |
| Diagnostics <-> cultures | 03 <-> 04 | `Визуальные-признаки-дефицита` -> `Проблемы-выращивания-салатов-и-трав` |

**Where to place links:**
1. In `## Связанные заметки` section at bottom (add if missing)
2. Inline in body text where natural reference exists (like existing `[[Коррекция-питательного-раствора|температуры питательного раствора]]` pattern in Температура-воздуха note)

### Pattern 2: Tag Taxonomy

**What:** Replace ad-hoc tags with a structured system.

**Proposed 3-tier taxonomy:**

**Tier 1 -- Domain tags (map to vault folders):**
| Tag | Folder | Notes |
|-----|--------|-------|
| `домен/основы` | 01 | CEA, vertical farms, economics |
| `домен/системы` | 02 | Hydroponic system types |
| `домен/растворы` | 03 | Nutrients, pH, EC, water |
| `домен/культуры` | 04 | Crops, microgreens |
| `домен/микроклимат` | 05 | Temperature, humidity, CO2 |
| `домен/освещение` | 06 | Light, spectrum, photosynthesis |
| `домен/проектирование` | 07 | System design, components |
| `домен/помещения` | 08 | Premises, infrastructure |
| `домен/устойчивость` | 09 | Sustainability, resources |

**Tier 2 -- Content type tags:**
| Tag | Meaning | Maps to `type:` |
|-----|---------|-----------------|
| `тип/знание` | Conceptual knowledge | knowledge |
| `тип/параметр` | Measurable parameter with IoT mapping | parameter |
| `тип/справочник` | Reference table, lookup data | reference |
| `тип/практика` | Step-by-step procedure, algorithm | knowledge (workflow) |

**Tier 3 -- Cross-cutting topic tags (keep best existing ones):**
| Tag | Notes |
|-----|-------|
| `iot` | Notes with IoT-маппинг section |
| `pH` | pH-related notes |
| `EC` | EC-related notes |
| `LED` | LED lighting notes |
| `фотосинтез` | Photosynthesis notes |
| `салаты` | Lettuce/salad notes |
| `микрозелень` | Microgreens notes |
| `DWC`, `NFT`, `аэропоника` | System-specific notes |

**What to drop:** Redundant/low-value tags that duplicate the note title or folder name without adding findability. Example: `капельный-полив` on `Капельный-полив.md` -- the title already says it.

### Pattern 3: Navigation Depth Check

**Current navigation paths:**
- Home.md -> MOCs (1 click) -> individual notes (2 clicks) = 2 clicks for any note within its domain
- Cross-domain navigation: Home -> MOC-A -> note-A -> cross-link -> note-B (3 clicks, sometimes 4)

**After cross-linking:** Any note reachable in max 3 clicks:
- Same domain: Home -> MOC -> note (2 clicks)
- Cross-domain: Home -> MOC -> note -> cross-link (3 clicks)

### Anti-Patterns to Avoid
- **Link-for-linking-sake:** Do NOT add a `[[link]]` just because two notes are in the same vault. Links must represent genuine conceptual relationships.
- **Circular link-only sections:** Avoid patterns where A links to B and B links to A with no other reason. Links should aid navigation, not create busywork.
- **Tag explosion:** 267 unique tags for 95 notes is already high (~2.8 per note). Taxonomy should REDUCE unique tags, not add more. Target: ~50-70 unique tags after normalization.
- **Nested tag abuse:** `домен/растворы/pH/калибровка` -- too deep. Maximum 2 levels of nesting.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Finding link candidates | Manual reading of all 95 notes | Systematic folder-by-folder analysis by Claude | Consistent coverage, no missed connections |
| Tag normalization | Manual one-by-one editing | Batch-process with pattern: read frontmatter -> apply taxonomy -> write | 95 notes with tags, manual would take hours |
| Broken link detection | Nothing | After linking, run `grep -r '\[\[' | extract targets | check file exists` | Catch typos in link targets |

## Common Pitfalls

### Pitfall 1: Broken Wiki-Links from Cyrillic Filenames
**What goes wrong:** Wiki-links must match exact filename (without .md). Cyrillic filenames are sensitive to encoding and hyphenation.
**Why it happens:** Typing `[[Температура воздуха]]` instead of `[[Температура-воздуха-на-сити-ферме]]`.
**How to avoid:** Always use exact filename from the filesystem. Build a lookup map of all note filenames before linking.
**Warning signs:** Obsidian shows unresolved links in a different color.

### Pitfall 2: Over-Linking Body Text
**What goes wrong:** Turning every mention of a concept into a wiki-link makes text unreadable.
**Why it happens:** Enthusiasm for connectivity.
**How to avoid:** Link inline only on first meaningful mention. Keep `## Связанные заметки` section for systematic cross-references. 3-6 cross-links per note is healthy; >10 suggests over-linking.

### Pitfall 3: Tag Taxonomy Inconsistency
**What goes wrong:** Using `домен/растворы` on some notes and `растворы` on others.
**Why it happens:** Partial migration, forgetting the prefix.
**How to avoid:** Process all notes in a single batch, use a defined tag list, validate after.

### Pitfall 4: Losing Existing Useful Tags
**What goes wrong:** Replacing all tags with taxonomy removes useful search terms.
**Why it happens:** Overly strict normalization.
**How to avoid:** Keep useful cross-cutting topic tags (Tier 3) alongside the taxonomy. Only remove truly redundant ones.

## Current State Audit

### Linking Status by Folder

| Folder | Notes | With Links | With Cross-Folder Links | Missing `## Связанные заметки` |
|--------|-------|-----------|------------------------|-------------------------------|
| 01-Основы | 6 | 6 | ~3 | 0 |
| 02-Системы | 7 | 7 | 0 | 0 |
| 03-Растворы | 22 | 21 | ~5 | 1 |
| 04-Культуры | 13 | 13 | 0 | 0 |
| 05-Микроклимат | 7 | 5 | ~2 | 2 |
| 06-Освещение | 19 | 13 | ~3 | 6 |
| 07-Проектирование | 6 | 6 | 0 | 0 |
| 08-Помещения | 9 | 8 | ~5 | 1 |
| 09-Устойчивость | 6 | 6 | ~1 | 1 |
| **Total** | **95** | **85** | **~17** | **11** |

### Key Numbers
- Total intra-folder links: 238
- Total cross-folder links: 23
- Cross-folder ratio: 8.8% (target: 25-35%)
- Notes missing `## Связанные заметки`: 11
- Unique tags: 267 (target after normalization: 50-70)
- All notes have `tags:` in frontmatter: YES (95/95)
- All notes have `type:` in frontmatter: YES
- All notes have `status: draft`: YES

### 11 Notes Missing `## Связанные заметки`
1. `EC-по-фазам-развития-растений.md` (03)
2. `Влажность-воздуха-на-сити-ферме.md` (05)
3. `Влияние-спектральных-диапазонов-на-растения.md` (06)
4. `Концентрация-CO2-на-сити-ферме.md` (05)
5. `Преимущества-LED-перед-НЛВД.md` (06)
6. `Суточная-доза-освещения-DLI.md` (06)
7. `Температура-воздуха-на-сити-ферме.md` (05)
8. `Требования-к-помещению-сити-фермы.md` (08)
9. `Фотометрические-величины.md` (06)
10. `Фоторецепторы-растений.md` (06)
11. `Фотосинтетически-активная-радиация.md` (06)

## Execution Strategy

### Recommended Batch Order

**Wave 1: Add cross-folder wiki-links (LINK-01)**
Process folder-by-folder. For each note:
1. Read content, identify cross-domain concepts mentioned
2. Check if `## Связанные заметки` section exists; add if missing
3. Add cross-folder links to the section
4. Add 1-2 inline links in body text where natural

Suggested folder processing order (by cross-link potential):
1. 03-Питательные-растворы (22 notes, many cross-domain concepts: pH->cultures, water->sustainability)
2. 04-Культуры (13 notes, links to 03/parameters, 06/lighting, 02/systems)
3. 06-Освещение (19 notes, 6 missing sections, links to 04/cultures, 05/microclimate)
4. 05-Микроклимат (7 notes, 2 missing sections, links to 08/premises, 06/lighting)
5. 01-Основы (6 notes, foundational links to all domains)
6. 02-Системы-выращивания (7 notes, links to 07/design, 03/nutrients)
7. 07-Проектирование (6 notes, links to 02/systems, 03/nutrients)
8. 08-Помещения (9 notes, links to 05/microclimate, 09/sustainability)
9. 09-Устойчивое-развитие (6 notes, links to 03/water, 06/LED)

**Wave 2: Normalize tag taxonomy (LINK-02)**
Single batch pass across all 95 notes:
1. Define canonical tag list (domain + type + cross-cutting)
2. For each note: replace tags with taxonomy-compliant set
3. Validate: no orphan tags, consistent application

### Estimated Scope
- Wave 1: ~95 notes to review, ~60-70 needing new cross-links, ~11 needing section added
- Wave 2: ~95 notes to re-tag
- Can be split into 2-4 plans for manageable batches

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Shell scripts + grep-based validation |
| Config file | none -- ad-hoc validation commands |
| Quick run command | `grep -r '\[\[' CityFarm --include="*.md" \| wc -l` |
| Full suite command | See validation script below |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| LINK-01 | Cross-folder links exist | smoke | `python3 cross-link-audit.py` (count cross-folder links > 80) | Wave 0 |
| LINK-01 | No broken wiki-links | smoke | Extract all `[[targets]]`, verify each maps to a .md file | Wave 0 |
| LINK-01 | All notes have `## Связанные заметки` | smoke | `grep -rL "Связанные заметки" CityFarm --include="*.md"` (excluding MOC/Home/Glossary) | Wave 0 |
| LINK-02 | Tag taxonomy applied | smoke | Verify all tags match canonical list | Wave 0 |
| LINK-02 | Domain tag present on every content note | smoke | Check each note has at least one `домен/*` tag | Wave 0 |

### Sampling Rate
- **Per task commit:** Quick grep-based link count
- **Per wave merge:** Full validation (broken links + tag compliance)
- **Phase gate:** All notes have cross-links, all tags normalized, no broken links

### Wave 0 Gaps
- [ ] Validation script for cross-folder link counting
- [ ] Canonical tag list definition (as part of LINK-02 plan)
- [ ] Broken link checker script

## Open Questions

1. **Tag language: Russian or English?**
   - Current: mixed (most Russian, some English like `LED`, `DWC`, `NFT`, `pH`, `EC`)
   - Recommendation: Keep Russian for domain/type prefixes (`домен/`, `тип/`), English for technical terms (`LED`, `DWC`, `pH`)
   - Confidence: MEDIUM -- user preference may differ

2. **Should MOCs get cross-section links?**
   - Currently MOCs only link to notes in their own section
   - Could add "See also: [[other-MOC]]" sections
   - Recommendation: YES, add brief "## Смежные разделы" to MOCs linking to related MOCs
   - Confidence: MEDIUM

3. **How many cross-folder links per note is right?**
   - Recommendation: 1-4 cross-folder links per note, in addition to existing intra-folder links
   - Total links per note target: 3-8 (currently ~2.5 average)
   - Confidence: HIGH

## Sources

### Primary (HIGH confidence)
- Direct vault analysis: 95 content notes audited, 238 intra-folder links counted, 23 cross-folder links counted
- File-level analysis: all 11 notes missing `## Связанные заметки` identified
- Tag analysis: 267 unique tags across 95 notes, distribution mapped

### Secondary (MEDIUM confidence)
- Obsidian wiki-link syntax: standard `[[filename]]` and `[[filename|display text]]` format (verified in existing notes)
- Obsidian YAML tags: array format in frontmatter, nested with `/` separator (standard Obsidian feature)

## Metadata

**Confidence breakdown:**
- Current state audit: HIGH - direct filesystem analysis
- Cross-link candidates: HIGH - based on actual note content review
- Tag taxonomy design: MEDIUM - design choices, user may prefer alternatives
- Execution strategy: HIGH - follows established batch processing pattern from Phase 3

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (vault structure stable, no external dependencies)
