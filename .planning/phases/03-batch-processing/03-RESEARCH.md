# Phase 3: Batch Processing - Research

**Researched:** 2026-03-09
**Domain:** Batch transcript processing, knowledge extraction, vault population (modules 1-11 excluding pilot module 3)
**Confidence:** HIGH

## Summary

Phase 3 scales the validated pilot processing pipeline from Module 3 (Phase 2) to all remaining course modules. The work divides into two distinct sub-tasks: (1) restructuring existing clean summaries for Modules 1-2 (already processed into structured summaries of 672 and 967 words), and (2) processing raw STT transcripts for Modules 4-11 (71 lessons, ~540KB total raw text). The processing prompt template, Glossary (~158 STT corrections), and note format are all validated from Phase 2.

The key challenge is volume management and correct topic-folder placement. Module 3 produced 24 notes from 12 lessons -- extrapolating, Modules 4-11 produce an estimated 80-120 notes from 54 lessons, plus 8-12 notes from restructuring Modules 1-2 summaries. Total vault will grow from 24 content notes to approximately 110-155 notes across 9 topic folders. Module 9 (Освещение, 14 lessons, 154KB) is the largest and will require special attention for batching. Modules 1-2 are a different workflow: they already have clean summaries and need restructuring into atomic notes, not STT cleanup.

**Primary recommendation:** Process in 4 plan waves matching the 4 requirements (PROC-02 through PROC-05). Use the established processing pipeline with minimal modifications. Modules 1-2 (PROC-02) use summaries as primary source with transcripts for supplementary detail. Modules 4-11 (PROC-03 through PROC-05) use the standard STT cleanup pipeline. Batch 3-5 lessons per plan task within each wave.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PROC-02 | Modules 1-2 existing summaries restructured into topic-based atomic notes | Summaries are clean (8.8KB + 13KB), well-structured markdown. No STT cleanup needed. Restructure into atomic notes using knowledge-note/reference-table templates. Target folders: 01-Основы/ (Module 1 content) and 02-Системы-выращивания/ (Module 2 content) |
| PROC-03 | Modules 4-6 processed from transcripts into atomic notes | Module 4 (4 lessons, 34KB) -> 04-Культуры/; Module 5 (5 lessons, 31KB) -> 04-Культуры/ (microgreens are crops); Module 6 (3 lessons, 68KB) -> 09-Устойчивое-развитие/. All have raw STT transcripts + brief СВОДКА files for reference |
| PROC-04 | Modules 7-9 processed from transcripts into atomic notes | Module 7 (4 lessons, 56KB) -> 07-Проектирование/; Module 8 (5 lessons, 37KB) -> 05-Микроклимат/; Module 9 (14 lessons, 154KB) -> 06-Освещение/. Module 9 is the largest module -- needs 3-4 task batches |
| PROC-05 | Modules 10-11 processed from transcripts into atomic notes | Module 10 (7 lessons, 61KB) -> 08-Помещения/; Module 11 (4 lessons, 32KB) -> 01-Основы/ or 09-Устойчивое-развитие/ (business basics overlap). СВОДКА files available for cross-reference |
</phase_requirements>

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Processing prompt template | v1 (from Phase 2) | Reusable prompt for transcript cleanup and extraction | Validated on 12 lessons, produced 24 quality notes |
| Glossary.md | ~158 entries | STT error correction reference | Expanded during Phase 2, covers core hydroponics terminology |
| CityFarm vault | 9 topic folders | Target for atomic notes | Created in Phase 1, populated with Module 3 notes in Phase 2 |
| _templates/ | 3 templates | Note structure (knowledge-note, reference-table, parameter-card) | Validated in Phase 2, consistent frontmatter format |

### Supporting
| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| МОДУЛЬ_N_СВОДКА.txt | Per module | Clean summaries for numeric value verification | Cross-reference every extracted numeric value |
| Processing log | Per module | Track lesson-to-note mapping | Create one per plan task, merge into module log |
| 00-INDEX.md (CFT) | Existing | Topic-to-module mapping reference | When deciding folder placement for cross-cutting topics |

## Architecture Patterns

### Module-to-Folder Mapping
```
Module 1 (Что такое сити-фермерство)    -> 01-Основы/
Module 2 (Системы без почв)              -> 02-Системы-выращивания/
Module 4 (Салаты и травы)                -> 04-Культуры/
Module 5 (Микрозелень)                   -> 04-Культуры/
Module 6 (Устойчивое развитие)           -> 09-Устойчивое-развитие/
Module 7 (Проектирование систем)         -> 07-Проектирование/
Module 8 (Микроклимат)                   -> 05-Микроклимат/
Module 9 (Освещение)                     -> 06-Освещение/
Module 10 (Выбор помещений)              -> 08-Помещения/
Module 11 (Основы бизнеса)              -> 01-Основы/ (foundational) or 09-Устойчивое-развитие/
```

### Pattern 1: Summary Restructuring (Modules 1-2 only)
**What:** Modules 1-2 have clean, structured summaries (not raw STT). The task is restructuring, not cleanup.
**When to use:** PROC-02 only
**Process:**
1. Read МОДУЛЬ_N_СВОДКА.txt as primary source (clean text, no STT errors)
2. Optionally read raw transcripts for supplementary detail missed in summary
3. Identify atomic topics within the summary sections
4. Create notes using same templates/frontmatter as Phase 2
5. No Glossary expansion needed (text is already clean)

**Module 1 expected topics (from summary analysis):**
- Определение сити-фермерства / CEA (01-Основы/)
- Контролируемые параметры среды (01-Основы/)
- Вертикальные фермы (01-Основы/)
- Преимущества сити-фермерства (01-Основы/)
- Экономическая эффективность сити-ферм (01-Основы/)
- Снижение затрат на электроэнергию (01-Основы/ or 06-Освещение/)

**Module 2 expected topics (from summary analysis):**
- Гидропоника -- определение и методы (02-Системы-выращивания/)
- Фитильная система (02-Системы-выращивания/)
- Глубоководная система DWC (02-Системы-выращивания/)
- Периодическое затопление Ebb & Flow (02-Системы-выращивания/)
- Капельный полив (02-Системы-выращивания/)
- NFT -- техника питательного слоя (02-Системы-выращивания/)
- Аэропоника (02-Системы-выращивания/)

### Pattern 2: Standard STT Processing (Modules 4-11)
**What:** Same pipeline as Phase 2 -- clean STT, extract atomic notes, extract parameters
**When to use:** PROC-03, PROC-04, PROC-05
**Process:** Identical to Phase 2 processing-prompt-template.md

### Pattern 3: Large Module Batching (Module 9 specifically)
**What:** Module 9 has 14 lessons (154KB) -- the largest module. Must be batched into 3-4 task chunks.
**When to use:** Module 9 processing within PROC-04
**Recommended splits:**
- Lessons 1-4: Light fundamentals, spectra
- Lessons 5-8: LED technology, efficiency
- Lessons 9-11: Light calculations, parameters
- Lessons 12-14: Practical lighting design

### Pattern 4: Cross-Topic Placement Decisions
**What:** Some module content belongs in multiple folders
**Examples from analysis:**
- Module 1 section on LED efficiency -> could go to 06-Освещение/ or stay in 01-Основы/
- Module 6 on water recycling -> could go to 03-Питательные-растворы/ (already has water quality notes)
- Module 8 on automation sensors -> could go to 07-Проектирование/ (system design)
- Module 11 business content -> may be out of scope per PROJECT.md ("бизнес-план и финмодель -- не релевантно для домашнего проекта")
**Rule:** Default to the topic folder matching the module. Cross-place only when content is clearly a better fit elsewhere. Never duplicate.

### Anti-Patterns to Avoid
- **Processing all modules sequentially without parallelization:** Modules are independent. Plans for PROC-03, PROC-04, PROC-05 can be structured with independent tasks per module.
- **Applying STT cleanup to Modules 1-2 summaries:** These are already clean. Don't waste effort on STT correction -- just restructure.
- **Creating business-plan notes from Module 11:** PROJECT.md explicitly excludes "бизнес-план и финмодель." Extract only technical/practical content (premises evaluation criteria, etc.).
- **Ignoring the Glossary for new modules:** Modules 4-11 will have domain-specific STT errors not yet in the Glossary. Continue expanding it.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Note template structure | Manual frontmatter each time | _templates/ knowledge-note.md, reference-table.md, parameter-card.md | Consistency enforced by template |
| STT correction | Module-specific correction lists | Glossary.md (cumulative, expanding) | Each module's corrections feed the next |
| Processing pipeline | Per-module custom prompts | processing-prompt-template.md from Phase 2 | Validated on 12 lessons, stable |
| Folder placement | Guessing per-note | Module-to-folder mapping table above | Predictable, documented |
| Duplicate detection | Post-hoc review | Processing log per module | Track all created notes, check against existing vault |

## Common Pitfalls

### Pitfall 1: Module 11 Business Content Scope Creep
**What goes wrong:** Creating detailed business plan notes that are explicitly out of scope
**Why it happens:** Module 11 is "Основы бизнеса" -- sounds like all business content should be extracted
**How to avoid:** Filter through PROJECT.md constraint: "бизнес-план и финмодель -- не релевантно для домашнего проекта." Extract only: operational knowledge (staffing, logistics), technical setup tips, practical evaluation criteria. Skip: revenue projections, franchise models, marketing strategies.
**Warning signs:** Notes about ROI calculations, pricing strategies, marketing channels

### Pitfall 2: Module 9 Processing Overwhelm
**What goes wrong:** Trying to process 14 lessons (154KB) in one task leads to quality degradation
**Why it happens:** Module 9 is 3-4x larger than average modules
**How to avoid:** Split Module 9 into 3-4 task batches of 3-5 lessons each. Review quality after each batch.
**Warning signs:** Notes becoming shallow, missing numeric parameters, skipping details

### Pitfall 3: Duplicate Content with Existing Module 3 Notes
**What goes wrong:** New modules create notes that overlap with existing Module 3 content (pH, EC, water quality)
**Why it happens:** Multiple modules reference the same parameters
**How to avoid:** Before creating a new note, check if topic is already covered in existing vault notes. If yes, add new information to existing note or create a more specific note that links to the general one. Never duplicate parameter cards.
**Warning signs:** Two notes about pH from different modules, duplicate EC reference tables

### Pitfall 4: Inconsistent Quality Across Modules
**What goes wrong:** Later modules get processed with less care than earlier ones
**Why it happens:** Processing fatigue, time pressure
**How to avoid:** Use the same processing prompt template for every module. Spot-check 1 note per module against source transcript. Maintain processing log.
**Warning signs:** Notes with remaining STT errors, missing frontmatter fields, missing source attribution

### Pitfall 5: Module 1-2 Over-Processing
**What goes wrong:** Re-reading all Module 1-2 raw transcripts and doing full STT cleanup when summaries are sufficient
**Why it happens:** Applying the same pipeline as Modules 4-11
**How to avoid:** Use summaries as primary source. Only check raw transcripts for details not captured in summaries.
**Warning signs:** Spending more time on Modules 1-2 than on larger Modules 4-11

### Pitfall 6: New Domain STT Errors Not in Glossary
**What goes wrong:** Modules 4-11 cover different domains (cultures, lighting, premises) with new vocabulary not in Glossary
**Why it happens:** Glossary was built from Module 3 (nutrient solutions domain)
**How to avoid:** Each module will introduce domain-specific STT errors. Actively expand Glossary with new categories: lighting terms (PPFD, спектр, фотосинтез), microclimate terms (CO2, вентиляция), crop terms (микрозелень, базилик). Batch Glossary updates per module.
**Warning signs:** Unfamiliar terms left uncorrected, inconsistent spelling of new domain terms

## Workload Estimation

### Volume Summary
| Group | Modules | Total Lessons | Total Size | Source Type | Estimated Notes |
|-------|---------|---------------|------------|-------------|-----------------|
| PROC-02 | 1-2 | 13 (but using summaries) | 22KB summaries | Clean summaries | 10-15 |
| PROC-03 | 4-6 | 12 | 132KB transcripts | Raw STT | 20-30 |
| PROC-04 | 7-9 | 23 | 248KB transcripts | Raw STT | 35-55 |
| PROC-05 | 10-11 | 11 | 93KB transcripts | Raw STT | 15-25 |
| **Total** | **10 modules** | **59 lessons** | **495KB** | **Mixed** | **80-125** |

### Processing Velocity (from Phase 2)
- Phase 2 processed 12 lessons into 24 notes across 3 plans (~14 min total)
- Average: ~4 lessons per plan, ~4.7 min per plan
- Estimated Phase 3: 10-14 plans, ~47-66 min total execution

## Code Examples

### Module 1 Restructured Note Example
```markdown
---
title: "Что такое сити-фермерство"
type: knowledge
topic: основы
tags:
  - сити-фермерство
  - CEA
  - определение
  - вертикальные-фермы
source: module-01/сводка
created: 2026-03-09
status: draft
---

## Суть

Сити-фермерство (Controlled Environment Agriculture, CEA) -- перспективное направление сельского хозяйства, выращивание растительной продукции в закрытых контролируемых помещениях в городской среде. Главное отличие от традиционного земледелия -- полный контроль всех параметров среды: температура, влажность, освещение, состав воздуха, питательный раствор.

## Детали

Контролируемые параметры: температура, влажность, освещение, состав воздуха, потоки воздуха, состав питательного раствора, pH, электропроводность. Управление через кондиционеры, увлажнители, вентиляцию, LED-светильники, автоматические системы подачи питания.

Сити-фермерство = вертикальное фермерство. Многоярусные конструкции увеличивают урожайность на единицу площади.

## Связанные заметки

- [[Преимущества сити-фермерства]]
- [[Экономическая эффективность сити-ферм]]
```

### Module 4 Processed Note Example (from raw STT)
```markdown
---
title: "Салатные культуры для гидропоники"
type: knowledge
topic: культуры
tags:
  - салаты
  - листовые-культуры
  - выращивание
source: module-04/урок-01
created: 2026-03-09
status: draft
---

## Суть

Листовые салаты -- одна из основных групп овощных культур для гидропоники. Отличаются зеленой листовой пластиной, укороченным стеблем, нейтральным освежающим вкусом и хрустящей текстурой. Простые в выращивании, высокая всхожесть (до 98%), полный цикл роста 35-45 дней.

## Детали

**Основные виды:** латук, лолло росса, айсберг, романо, фризе. Сорта для гидропоники: Максимаст, Эксайд, Экзакт, Дюнан.

**Родственные культуры:** кейл (листовая капуста), мангольд (свекла без корнеплода), щавель, шпинат.

**Показатели:**
- Всхожесть: ~98%
- Прорастание: 2-3 дня после посева
- Цикл роста: 35-45 дней
- Выход биомассы: 100-150 г с куста

**Формы поставки:** васова милистия, бэби-лив, салатмиксы, салат в горшочках с субстратом.

## Связанные заметки

- [[Пряные травы для гидропоники]]
- [[Выращивание микрозелени]]
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Module-based summaries per module | Topic-based atomic notes across folders | Phase 2 (established) | Knowledge is searchable by topic, not locked to course structure |
| Modules 1-2 as standalone summaries | Restructured into same atomic format as others | This phase | Consistent format across all modules |
| Glossary with ~158 hydroponics terms | Expanded with lighting, climate, crop terms | This phase (expected) | Better STT correction for new domains |

## Open Questions

1. **Module 11 scope filtering**
   - What we know: PROJECT.md excludes "бизнес-план и финмодель" as out of scope for домашний проект
   - What's unclear: How much of Module 11 content is purely business vs. practically useful
   - Recommendation: Process Module 11 selectively. Extract: premises evaluation, operational setup, practical tips. Skip: financial models, marketing, franchise models.

2. **Module 5 (Микрозелень) folder placement**
   - What we know: Vault has 04-Культуры/ but microgreens could be argued as a separate topic
   - What's unclear: Whether microgreens deserve a separate folder or belong with other crops
   - Recommendation: Place in 04-Культуры/ -- it is a crop type. The folder name "Культуры" is broad enough.

3. **Cross-module parameter deduplication**
   - What we know: Module 8 (microclimate) will mention temperature/humidity parameters that may overlap with Module 3 notes on solution temperature
   - What's unclear: Exact overlap scope
   - Recommendation: Create new parameter cards for microclimate-specific parameters (air temp, humidity, CO2). Link to existing solution-temperature notes. No duplication of identical values.

4. **Glossary growth rate for new domains**
   - What we know: Phase 2 added ~70 STT corrections from Module 3 (hydroponic chemistry domain)
   - What's unclear: How many new corrections will modules 4-11 introduce across lighting, climate, crops, business domains
   - Recommendation: Estimate 5-15 new corrections per module. Batch Glossary updates per plan task. Expected total: 200-300 entries by end of Phase 3.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | File verification + frontmatter checks (same as Phase 2, no automated test framework for content quality) |
| Config file | N/A -- content processing, validated by structure checks |
| Quick run command | `find CityFarm/ -name "*.md" -not -path "*/.obsidian/*" -not -path "*/_templates/*" -not -name "*MOC*" \| wc -l` |
| Full suite command | `for d in CityFarm/0*/; do echo "$d: $(ls "$d"/*.md 2>/dev/null \| grep -v MOC \| wc -l) notes"; done` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROC-02 | Modules 1-2 restructured into atomic notes in 01-Основы/ and 02-Системы-выращивания/ | smoke | `ls CityFarm/01-Основы/*.md CityFarm/02-Системы-выращивания/*.md 2>/dev/null \| grep -v MOC \| wc -l` (expect 10+) | N/A -- Wave 0 |
| PROC-03 | Modules 4-6 processed into notes in 04-Культуры/ and 09-Устойчивое-развитие/ | smoke | `ls CityFarm/04-Культуры/*.md CityFarm/09-Устойчивое-развитие/*.md 2>/dev/null \| grep -v MOC \| wc -l` (expect 15+) | N/A -- Wave 0 |
| PROC-04 | Modules 7-9 processed into notes in 07-Проектирование/, 05-Микроклимат/, 06-Освещение/ | smoke | `ls CityFarm/07-Проектирование/*.md CityFarm/05-Микроклимат/*.md CityFarm/06-Освещение/*.md 2>/dev/null \| grep -v MOC \| wc -l` (expect 25+) | N/A -- Wave 0 |
| PROC-05 | Modules 10-11 processed into notes in 08-Помещения/ | smoke | `ls CityFarm/08-Помещения/*.md 2>/dev/null \| grep -v MOC \| wc -l` (expect 10+) | N/A -- Wave 0 |

### Sampling Rate
- **Per task commit:** Quick note count in target folder + frontmatter spot-check on 1 random note
- **Per wave merge:** Full note count per folder, verify source: module-NN present in all notes
- **Phase gate:** Total vault note count 100+. All 9 topic folders populated. Glossary expanded. Processing logs for all modules.

### Wave 0 Gaps
- [ ] Processing logs for each module group (create during each plan task)
- [ ] Glossary categories for new domains (lighting, microclimate, crops, premises)

## Sources

### Primary (HIGH confidence)
- Project files: All module СВОДКА files analyzed for content scope and topic mapping
- Project files: Module 4 and Module 9 raw transcripts sampled for STT quality assessment
- Project files: Phase 2 processing-prompt-template.md, processing-log.md -- validated pipeline
- Project files: Phase 2 RESEARCH.md, 02-03-SUMMARY.md -- pipeline validation results
- Project files: CityFarm vault -- 24 existing notes examined for format consistency
- Project files: README_MODULES.md -- module structure and status reference

### Secondary (MEDIUM confidence)
- Note count estimates (80-125 total) -- extrapolated from Phase 2 ratio of 2 notes/lesson average

### Tertiary (LOW confidence)
- Module 11 scope filtering -- needs user confirmation on how much business content to include
- Processing time estimates -- based on Phase 2 velocity which may not hold for diverse content domains

## Metadata

**Confidence breakdown:**
- Processing pipeline: HIGH -- identical to validated Phase 2 pipeline, no changes needed
- Module-to-folder mapping: HIGH -- clear topic-folder correspondence from vault structure
- Workload estimation: MEDIUM -- extrapolated from Phase 2, but module sizes vary significantly (31KB-154KB)
- Module 11 scope: MEDIUM -- PROJECT.md gives guidance but exact filtering requires judgment
- Glossary expansion: MEDIUM -- new domains will have new STT patterns, exact count unknown

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (source transcripts are static)
