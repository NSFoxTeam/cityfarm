# Processing Prompt Template -- Transcript to Atomic Notes

> Reusable prompt structure for processing Russian STT transcripts from the city farming course into atomic Obsidian notes.

## System Context

You are processing a Russian STT (speech-to-text) transcript from a hydroponics/city farming course. The transcript contains heavy STT artifacts: word truncation, wrong endings, merged/split words, and garbled passages. Your task is to extract clean, structured atomic knowledge notes for an Obsidian vault.

## Inputs

1. **Transcript file:** Raw STT text (5-14KB per lesson)
2. **Glossary.md:** STT correction reference (~100+ term corrections)
3. **Module summary (СВОДКА):** Clean reference for numeric value verification
4. **Target folder:** Based on topic, not source module

## Processing Pipeline

### Step 1: STT Cleanup

- Apply all corrections from Glossary.md (e.g., "азод" -> "азот", "кали" -> "калий")
- Fix obvious garbled passages using surrounding context
- Mark truly unintelligible passages as `[неразборчиво]`
- Do NOT guess numeric values -- mark unclear numbers as `[значение неразборчиво]`

### Step 2: Content Analysis

- Identify distinct knowledge topics within the lesson
- Each topic = one atomic note (target 2-5 notes per lesson)
- A concept is something that could be a glossary entry, a parameter reference, or a standalone explanation
- Map each topic to the appropriate vault folder (topic-based, not module-based)

### Step 3: Atomic Note Extraction

For each identified topic, create one markdown file:

**Filename:** Topic-based Cyrillic with hyphens (`Макроэлементы-растений.md`, NOT `урок-1-макро.md`)

**YAML Frontmatter (required):**
```yaml
---
title: "Human-readable title"
type: knowledge | parameter | reference
topic: <folder-name-without-number>  # e.g., "питательные-растворы"
tags:
  - relevant-tag-1
  - relevant-tag-2
source: module-NN/урок-NN
created: YYYY-MM-DD
status: draft
---
```

**Body structure for knowledge notes:**
```markdown
## Суть
1-2 paragraphs: core concept in clean, reference-style language.

## Детали
Specifics, lists, examples. Preserve exact numeric values with units.

## Связанные заметки
- [[Related Note Name]]
```

**Body structure for parameter cards:**
```markdown
## Описание
What this parameter is and why it matters.

## Оптимальные значения
| Культура | Мин | Макс | Единица | Примечание |

## IoT-маппинг
| Параметр | Сенсор | Порог | Алерт | Действие |

## Источники
- Module N, Lesson M
```

**Body structure for reference tables:**
```markdown
## Параметры
| Параметр | Диапазон | Единица | Примечание |

## Источники
- Module N, Lesson M
```

### Step 4: Parameter Extraction

- Extract exact numeric values with units (pH 5.5-6.5, EC 1.5-2.5 mS/cm)
- Cross-reference with МОДУЛЬ_СВОДКА.txt for verification
- Create parameter-card or reference-table notes for quantitative data

### Step 5: Glossary Update

- Document all new STT errors discovered during processing
- Add to appropriate category tables in Glossary.md
- Pattern: `| Правильно | STT-варианты | English | Категория |`

## Rules

1. **One concept per file.** Never combine unrelated topics.
2. **Topic-based filenames.** Use the concept name, not the lesson number.
3. **Extract knowledge, not lectures.** Remove all lecture artifacts: "сегодня мы поговорим", "на следующем занятии", "давайте рассмотрим", greetings, goodbyes.
4. **Preserve numeric precision.** Exact values with units. Never round or paraphrase numbers.
5. **Mark unknowns.** Use `[неразборчиво]` for garbled text, `[значение неразборчиво]` for unclear numbers.
6. **Wiki-links.** Use `[[Note Name]]` for cross-references in "Связанные заметки".
7. **Source attribution.** Every note must have `source: module-NN/урок-NN` in frontmatter.
8. **Folder placement by topic.** Root physiology -> `01-Основы/`, nutrient solutions -> `03-Питательные-растворы/`, etc.

## Output

- 2-5 atomic note files per lesson
- Updated Glossary.md entries (if new STT errors found)
- Processing log entry for the lesson
