# Phase 2: Pilot Processing - Research

**Researched:** 2026-03-09
**Domain:** Transcript cleanup, knowledge extraction, Obsidian atomic note creation
**Confidence:** HIGH

## Summary

Phase 2 processes 12 Module 3 transcripts (~120KB total, Russian STT output) into clean, structured atomic notes in the CityFarm Obsidian vault. The transcripts cover nutrient solutions -- macro/microelements, pH, EC, water quality, measuring instruments, and solution management. Each transcript is a raw speech-to-text dump with heavy STT artifacts: word truncation ("кали" -> "калий"), wrong endings ("азод" -> "азот"), merged/split words ("электро проводность" -> "электропроводность"), and garbled passages where entire phrases are unintelligible.

The core challenge is designing a repeatable processing pipeline: (1) STT cleanup using the Glossary, (2) content structuring into logical sections, (3) knowledge extraction into atomic notes (one concept per file), and (4) numeric parameter extraction into reference format. The pipeline must be validated and reusable for Phase 3 batch processing of remaining modules. The existing vault has templates (knowledge-note, reference-table, parameter-card), a glossary of ~90 STT correction terms, and a topic folder `03-Питательные-растворы/` ready to receive content.

**Primary recommendation:** Process transcripts in lesson order, one at a time. For each: clean STT errors, extract 2-5 atomic notes per lesson, place in appropriate topic folder with proper YAML frontmatter. Create reusable processing prompts that can be templated for Phase 3. Extract numeric parameters (pH, EC ranges) into parameter cards and reference tables.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PROC-01 | Module 3 transcripts (12 lessons) cleaned, structured, knowledge extracted into atomic notes | Full pipeline: STT cleanup via Glossary -> content structuring -> atomic note extraction -> parameter extraction. Templates exist from Phase 1. Source: 12 .txt files in CFT/docs/transcripts_by_modules/module_03/ |
</phase_requirements>

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Obsidian vault (CityFarm/) | Existing | Target for atomic notes | Created in Phase 1, templates ready |
| Glossary.md | Existing | STT error correction reference | ~90 term corrections, created in Phase 1 |
| Claude Code | Current | Text processing and extraction | Processing agent for cleanup and extraction |

### Supporting
| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| knowledge-note.md template | Existing | Structure for concept notes | Every atomic knowledge note |
| reference-table.md template | Existing | Structure for parameter tables | pH/EC/TDS ranges, water quality specs |
| parameter-card.md template | Existing | Structure for single-parameter reference | Individual parameter deep-dives (pH, EC) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual per-lesson processing | Batch all 12 at once | Per-lesson is slower but catches quality issues early; batch risks propagating errors |
| Claude Code processing | External NLP pipeline | Overkill for 12 files; Claude understands Russian domain context natively |

## Architecture Patterns

### Recommended Processing Pipeline

```
Source transcript (.txt)
    |
    v
[Step 1: STT Cleanup]
    - Apply Glossary.md corrections
    - Fix obvious garbled passages using context
    - Mark unintelligible passages as [неразборчиво]
    |
    v
[Step 2: Content Analysis]
    - Identify distinct knowledge topics in lesson
    - Map topics to vault folders (most -> 03-Питательные-растворы/)
    - Some topics cross-cut to other folders (e.g., root physiology -> 01-Основы/)
    |
    v
[Step 3: Atomic Note Extraction]
    - One note per concept (2-5 notes per lesson typically)
    - Use knowledge-note.md template
    - Fill YAML frontmatter: type, topic, tags, source, status
    - Write content: Суть (core idea), Детали (details), Связанные заметки (links)
    |
    v
[Step 4: Parameter Extraction]
    - Extract numeric values (pH ranges, EC values, concentrations)
    - Create reference-table or parameter-card notes
    - Include source attribution (module-03/урок-N)
    |
    v
[Step 5: Glossary Update]
    - Add newly discovered STT errors to Glossary.md
    - Document patterns for Phase 3 reuse
```

### Pattern 1: Atomic Note Granularity
**What:** Each note covers exactly one concept, method, or parameter set
**When to use:** Always -- this is the core principle of the vault
**Example topic splits from Module 3:**
- Lesson 1 (Elements of plant nutrition) -> Notes: "Классификация элементов питания", "Макроэлементы растений", "Микроэлементы растений"
- Lesson 5 (pH of nutrient solution) -> Notes: "pH питательного раствора", "Влияние pH на доступность элементов"
- Lesson 12 (Water quality) -> Notes: "Качество воды для гидропоники", "Подготовка воды для раствора", "Жёсткость воды и её коррекция"

### Pattern 2: Source Attribution
**What:** Every note tracks its source lesson for traceability
**When to use:** All notes derived from transcripts
**Format in frontmatter:**
```yaml
source: module-03/урок-05
```
**In body, for specific numeric values:**
```markdown
> pH оптимальный: 5.5-6.5 *(Модуль 3, Урок 5)*
```

### Pattern 3: Cross-Folder Placement
**What:** Notes go to the topic folder that best matches their subject, even if source is Module 3
**When to use:** When a lesson covers a cross-cutting topic
**Example:** Lesson 3 covers root physiology (корневая система) -- this belongs in `01-Основы/` not `03-Питательные-растворы/`

### Pattern 4: Processing Prompt Template
**What:** Reusable prompt structure for Claude Code to process each transcript
**When to use:** Every lesson processing -- same prompt structure, different input
**Template structure:**
1. System context: "You are processing a Russian STT transcript about hydroponics..."
2. Glossary reference: include correction table
3. Instructions: clean, structure, extract atomic notes, extract parameters
4. Output format: markdown files with YAML frontmatter per template

### Anti-Patterns to Avoid
- **One giant note per lesson:** Defeats atomic note purpose. Split by concept, not by lesson.
- **Copying transcript verbatim after cleanup:** Extract knowledge, don't preserve lecture style ("Сегодня мы поговорим...", "На следующем занятии...").
- **Ignoring unintelligible passages:** Mark them `[неразборчиво]` rather than guessing. Better to have a gap than wrong information.
- **Skipping frontmatter:** Every note MUST have complete YAML frontmatter for Dataview queries.
- **Module-based filenames:** Use topic-based names ("pH-питательного-раствора.md" not "урок-5-pH.md").

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| STT correction | Custom regex script | Glossary.md + Claude's contextual understanding | Glossary covers known errors; Claude handles novel errors via context |
| Note templating | Manual copy-paste of frontmatter | Existing _templates/ with standard fields | Consistency across all notes |
| Topic classification | Manual guessing | 00-INDEX.md from CFT + vault folder structure | INDEX maps topics to modules; vault folders are the target taxonomy |
| Duplicate detection | Post-hoc review | Track extracted topics in a processing log | Prevent creating overlapping notes from different lessons |

## Common Pitfalls

### Pitfall 1: Overly Fine or Overly Coarse Note Granularity
**What goes wrong:** Either 1 note per lesson (too coarse, not atomic) or 10+ notes per lesson (too fine, fragments lose context)
**Why it happens:** No clear definition of "one concept"
**How to avoid:** Target 2-5 notes per lesson. A concept = something that could be a glossary entry, a parameter reference, or a standalone explanation. If you need more than 3 paragraphs, it's one note. If you can summarize in one sentence, it might be too small.
**Warning signs:** Notes with single sentences; notes that duplicate content across files

### Pitfall 2: Lost Numeric Precision
**What goes wrong:** Specific values (pH 5.5-6.5, EC 1.5-2.5 mS/cm) get paraphrased or rounded
**Why it happens:** Cleaning up garbled text, the numbers get simplified
**How to avoid:** Treat numeric values as sacred. Extract exact values with units. When transcript is unclear, mark as `[значение неразборчиво]`. Cross-reference with МОДУЛЬ_3_СВОДКА.txt which has clean summary values.
**Warning signs:** Parameters without units; ranges that don't match the summary

### Pitfall 3: STT Errors Surviving Cleanup
**What goes wrong:** New STT errors not in Glossary.md pass through uncorrected
**Why it happens:** Glossary covers ~90 terms but transcripts have many more domain words
**How to avoid:** During each lesson processing, actively look for new STT patterns. After processing 2-3 lessons, update Glossary.md with newly discovered errors before continuing.
**Warning signs:** Remaining typos/wrong words in final notes; inconsistent terminology across notes

### Pitfall 4: Unprocessable Passages
**What goes wrong:** Some transcript sections are so garbled they're meaningless (observed in lessons 7, 12)
**Why it happens:** Poor audio quality, background noise, speaker talking fast
**How to avoid:** Mark as `[неразборчиво]` or `[STT: неразборчивый фрагмент]`. Do NOT fabricate content. The МОДУЛЬ_3_СВОДКА.txt summary and 00-INDEX.md may provide context to fill gaps, but must be attributed.
**Warning signs:** Sentences that don't make grammatical sense even in Russian

### Pitfall 5: Processing Prompts Not Validated for Reuse
**What goes wrong:** Phase 3 can't reuse the pipeline because prompts were ad-hoc per lesson
**Why it happens:** Iterating on prompt quality per-lesson without extracting the final template
**How to avoid:** After processing first 3 lessons, stabilize the prompt template. Process remaining 9 lessons with the stable template. Document the final template as an artifact.
**Warning signs:** Each lesson processed differently; no documented prompt template at phase end

### Pitfall 6: Cross-Topic Content Misplaced
**What goes wrong:** All notes land in `03-Питательные-растворы/` when some belong in other folders
**Why it happens:** Module 3 is the source, so default placement is folder 03
**How to avoid:** Lesson 3 (root physiology) -> `01-Основы/`. Lesson 7 (instruments) could partially go to `07-Проектирование/`. Review each note's topic independently of its source module.
**Warning signs:** Folder 03 has 30+ notes while related folders have none

## Code Examples

### Atomic Note Example: pH Parameter
```markdown
---
title: "pH питательного раствора"
type: parameter
topic: питательные-растворы
tags:
  - pH
  - контроль-параметров
  - питательный-раствор
source: module-03/урок-05
created: 2026-03-09
status: draft
---

## Описание

pH -- показатель кислотности питательного раствора. Определяет активность ионов водорода (H+) в растворе. Шкала от 0 до 14: кислая среда (pH < 7), нейтральная (pH = 7), щелочная (pH > 7).

## Оптимальные значения

| Культура | Мин | Макс | Единица | Примечание |
|----------|-----|------|---------|------------|
| Общий диапазон | 5.5 | 6.5 | pH | Оптимальное усвоение элементов |

При pH > 6.5 микроэлементы (Fe, Mn) выпадают в осадок. При pH = 7.3 железо теряется на 50%. При pH = 8.0 железо практически отсутствует в растворе.

## IoT-маппинг

| Параметр | Сенсор | Порог | Алерт | Действие |
|----------|--------|-------|-------|----------|
| pH | DFRobot pH V2 | < 5.0 / > 7.0 | критический | Корректировка pH-буфером |

## Источники

- Модуль 3, Урок 5 -- реакция питательного раствора
```

### Atomic Note Example: Knowledge Note
```markdown
---
title: "Классификация элементов питания растений"
type: knowledge
topic: питательные-растворы
tags:
  - элементы-питания
  - макроэлементы
  - микроэлементы
  - классификация
source: module-03/урок-01
created: 2026-03-09
status: draft
---

## Суть

Элементы питания растений делятся на три группы по количеству потребления: макроэлементы (сотые доли -- проценты), микроэлементы (тысячные доли процента), ультрамикроэлементы (миллионные доли процента). Каждый элемент выполняет специфическую функцию и не может быть заменён другим.

## Детали

**Органогены** (основа органических соединений): углерод (C), водород (H), кислород (O).

**Макроэлементы:** азот (N), фосфор (P), калий (K), кальций (Ca), магний (Mg), сера (S), железо (Fe).

**Микроэлементы:** медь (Cu), бор (B), цинк (Zn), марганец (Mn), кобальт (Co), молибден (Mo).

**Ультрамикроэлементы:** серебро (Ag), радий (Ra), ртуть (Hg), кадмий (Cd).

Недостаток любого макроэлемента приводит к нарушению обмена веществ, ухудшению роста, снижению урожая и его качества.

## Связанные заметки

- [[Макроэлементы растений]]
- [[Микроэлементы растений]]
- [[pH питательного раствора]]
```

### Reference Table Example: EC Values by Growth Phase
```markdown
---
title: "EC по фазам развития растений"
type: reference
topic: питательные-растворы
tags:
  - EC
  - электропроводность
  - фазы-роста
parameters:
  - EC
source: module-03/урок-12
created: 2026-03-09
status: draft
---

## Параметры

| Параметр | Диапазон | Единица | Примечание |
|----------|----------|---------|------------|
| Черенки | 0.2-0.4 | мС/см | Минимальная концентрация |
| Рассада | 0.8-1.2 | мС/см | Начальное питание |
| Вегетация | 1.6-1.8 | мС/см | Активный рост |
| Цветение/плодоношение | 1.8-2.2 | мС/см | Повышенная потребность |
| Финальная стадия | 2.4-2.6 | мС/см | Максимальная концентрация |
| Полноценный раствор | 2.0-2.5 | мС/см | Общий целевой диапазон |

## Источники

- Модуль 3, Урок 12 -- показатели качества воды
```

### Processing Log Format
```markdown
# Processing Log -- Module 3

| Урок | Файл | Размер | Темы | Заметки | Параметры | Статус |
|------|------|--------|------|---------|-----------|--------|
| 1 | урок_1.txt | 6.4KB | Элементы питания, классификация | 3 | 0 | Done |
| 2 | урок_2.txt | 10.2KB | Роль минеральных веществ | 3 | 1 | Done |
| ... | | | | | | |
| 12 | урок_12.txt | 13KB | Качество воды, подготовка | 3 | 2 | Done |
```

## Lesson-to-Topic Mapping (Module 3)

Based on transcript analysis, here is the expected topic distribution:

| Lesson | Content Summary | Expected Notes | Target Folder |
|--------|----------------|----------------|---------------|
| 1 | Элементы питания, классификация (макро/микро) | 2-3 | 03-Питательные-растворы/ |
| 2 | Роль минеральных веществ в питании | 2-3 | 03-Питательные-растворы/ |
| 3 | Поступление минеральных солей через корни | 2-3 | 01-Основы/ (root physiology) |
| 4 | Концентрация раствора, взаимодействие элементов | 2-3 | 03-Питательные-растворы/ |
| 5 | pH раствора, кислотность, доступность элементов | 2-3 | 03-Питательные-растворы/ |
| 6 | Рецепты питательных растворов | 3-4 | 03-Питательные-растворы/ |
| 7 | Приборы (TDS/EC/pH метры), калибровка | 3-4 | 03-Питательные-растворы/ + 07-Проектирование/ |
| 8 | Диагностика питания, визуальные признаки | 2-3 | 03-Питательные-растворы/ |
| 9 | Управление составом, коррекция раствора | 2-3 | 03-Питательные-растворы/ |
| 10 | (TBD -- need to read) | 2-3 | 03-Питательные-растворы/ |
| 11 | (TBD -- need to read) | 2-3 | 03-Питательные-растворы/ |
| 12 | Качество воды, подготовка, фильтрация | 3-4 | 03-Питательные-растворы/ |

**Estimated total:** 30-40 atomic notes from 12 lessons.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Module-based summaries (МОДУЛЬ_3_СВОДКА.txt) | Atomic topic-based notes | This phase | Knowledge is searchable and linkable |
| Raw STT transcripts | Cleaned + structured content | This phase | Usable reference material |
| Flat text parameters | YAML frontmatter + tables | This phase | Dataview-queryable parameters |

## Open Questions

1. **Optimal batch size for processing**
   - What we know: 12 lessons, ~120KB total. Each lesson is 5-14KB.
   - What's unclear: Whether to process 1 lesson per plan task, or batch 3-4 lessons together
   - Recommendation: Process in batches of 3-4 lessons per plan task (4 tasks total). This balances quality review with throughput.

2. **Glossary expansion scope**
   - What we know: Current glossary has ~90 terms. Module 3 transcripts contain additional STT errors not yet catalogued.
   - What's unclear: How many new errors will be found
   - Recommendation: Update glossary after each batch. Expect 20-30 new corrections from Module 3.

3. **Cross-reference format between notes**
   - What we know: Wiki-links `[[note name]]` per vault convention
   - What's unclear: Whether to use aliases, section links, or simple note links
   - Recommendation: Simple `[[note name]]` for now. Aliases and section links can be added in Phase 4 (linking).

4. **Handling repeated content across lessons**
   - What we know: pH ranges (5.5-6.5) appear in lessons 5, 7, 9, 12. EC values appear in 7, 12.
   - What's unclear: Whether to create one note per parameter or allow overlap
   - Recommendation: One authoritative note per parameter. Later lessons that repeat/refine values should update the existing note, not create duplicates. Track which lessons contributed.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual review + file verification (no automated test framework applicable for content quality) |
| Config file | N/A -- content processing, validated by structure checks and manual review |
| Quick run command | `ls CityFarm/03-Питательные-растворы/*.md \| wc -l` |
| Full suite command | `find CityFarm/ -name "*.md" -not -path "*/.obsidian/*" -not -path "*/_templates/*" \| wc -l && grep -rl "source: module-03" CityFarm/ \| wc -l` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROC-01a | All 12 transcripts cleaned and structured | smoke | `ls CityFarm/03-Питательные-растворы/*.md \| wc -l` (expect 25+) | N/A -- Wave 0 |
| PROC-01b | Atomic notes have proper YAML frontmatter | smoke | `grep -l "^type: knowledge\|^type: parameter\|^type: reference" CityFarm/03-Питательные-растворы/*.md \| wc -l` | N/A -- Wave 0 |
| PROC-01c | pH/EC parameters extracted in reference format | smoke | `grep -l "type: parameter\|type: reference" CityFarm/03-Питательные-растворы/*.md` | N/A -- Wave 0 |
| PROC-01d | Processing prompts documented and reusable | smoke | `test -f .planning/phases/02-pilot-processing/processing-prompt-template.md && echo OK` | N/A -- Wave 0 |

### Sampling Rate
- **Per task commit:** Quick note count + frontmatter check
- **Per wave merge:** Full note count, frontmatter validation, cross-reference check
- **Phase gate:** Manual review of 3 random notes for content quality. Verify processing prompt template exists.

### Wave 0 Gaps
- [ ] Processing prompt template -- must be created and validated during first task
- [ ] Processing log -- track which lessons produced which notes

## Sources

### Primary (HIGH confidence)
- Project files: `CFT/docs/transcripts_by_modules/module_03/` -- all 12 source transcripts analyzed
- Project files: `CFT/docs/transcripts_by_modules/module_03/МОДУЛЬ_3_СВОДКА.txt` -- summary with clean parameter values
- Project files: `CFT/docs/city-farm-knowledge/00-INDEX.md` -- topic-to-module mapping
- Project files: `CityFarm/Glossary.md` -- existing STT correction glossary (~90 terms)
- Project files: `CityFarm/_templates/` -- existing note templates (knowledge, reference, parameter)

### Secondary (MEDIUM confidence)
- Phase 1 research: `.planning/phases/01-vault-foundation/01-RESEARCH.md` -- vault architecture decisions
- Phase 1 summaries: established patterns for frontmatter, folder structure

### Tertiary (LOW confidence)
- Lesson-to-topic mapping for lessons 10-11 (not yet read, estimated from module summary)
- Estimated note count (30-40) based on analysis of 5/12 lessons

## Metadata

**Confidence breakdown:**
- Processing pipeline: HIGH -- based on direct analysis of 5 transcripts, clear STT patterns observed
- Note granularity: HIGH -- templates exist, example notes validate the approach
- Parameter extraction: HIGH -- specific values confirmed in transcripts (pH 5.5-6.5, EC ranges)
- Reusable prompt: MEDIUM -- needs validation during processing, will stabilize after first batch
- Total note count: MEDIUM -- 30-40 estimated from partial analysis, may vary

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (content is static -- transcripts don't change)
