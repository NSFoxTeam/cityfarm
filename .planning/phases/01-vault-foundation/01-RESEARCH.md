# Phase 1: Vault Foundation - Research

**Researched:** 2026-03-09
**Domain:** Obsidian vault setup, knowledge base structure, hydroponic domain glossary
**Confidence:** HIGH

## Summary

Phase 1 creates an Obsidian vault `CityFarm/` in the project root with topic-based folder structure, Templater templates for three note types, configured plugins (Dataview, Templater, Obsidian Git), and a domain glossary for STT error correction. This is primarily a file-creation phase -- no code compilation, no external dependencies beyond downloading three Obsidian plugin releases from GitHub.

The key challenge is designing a topic-based (not module-based) folder structure that maps well to the 11 course modules while remaining navigable by topic. The existing `CFT/docs/city-farm-knowledge/00-INDEX.md` provides a complete content map across all modules. The STT glossary must handle Russian speech-to-text errors observed in transcripts (e.g., "дациент" instead of "доцент", "кали" instead of "калий", "азод" instead of "азот").

**Primary recommendation:** Create vault structure, templates, and plugin configs as plain files committed to git. Plugins are installed by downloading `main.js`, `manifest.json`, and `styles.css` from GitHub releases into `.obsidian/plugins/<plugin-id>/`. User opens vault in Obsidian to activate.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
No locked decisions -- all implementation details are left to Claude's discretion.

### Claude's Discretion
All implementation decisions are open:
- **Folder structure** -- topic sections based on 11 course modules analysis, nesting depth, folder naming
- **Note templates** -- YAML frontmatter fields, body structure for each type (knowledge note, reference table, parameter card)
- **Home.md** -- entry point format, section grouping, navigation style
- **Domain glossary** -- file format, term categories, Russian/English mapping, STT error coverage
- **Plugin configuration** -- Dataview, Templater, Obsidian Git settings

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| VAULT-01 | Obsidian vault `CityFarm/` with topic-based folder structure | Topic structure derived from 00-INDEX.md module analysis; .obsidian config for vault recognition |
| VAULT-02 | Home.md as entry point with section navigation | Wiki-link navigation pattern; Obsidian standard Home note approach |
| VAULT-03 | Templater templates for knowledge note, reference table, parameter card with YAML frontmatter | Templater tp.file/tp.date functions in frontmatter; Dataview-queryable fields |
| VAULT-04 | Plugins configured: Dataview, Templater, Obsidian Git | Plugin download from GitHub releases; community-plugins.json + data.json configs |
| VAULT-05 | Domain glossary of hydroponic terms for STT error correction | Real STT errors observed in module_03 transcripts; ~880KB of transcripts across 11 modules |
</phase_requirements>

## Standard Stack

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Obsidian | 1.7+ | Knowledge base application | Already used in project (CLAUDE.md references ObsidianDB/) |
| Dataview | latest | Query and display vault metadata | De facto standard for structured queries in Obsidian |
| Templater | 2.18+ | Template engine with dynamic fields | Standard for YAML frontmatter generation, tp.file/tp.date functions |
| Obsidian Git | 2.38+ | Git sync for vault | Already pattern in user's other projects (Kohan) |

### Plugin IDs and GitHub Repos
| Plugin | ID (community-plugins.json) | GitHub Repo |
|--------|----------------------------|-------------|
| Dataview | `dataview` | `blacksmithgu/obsidian-dataview` |
| Templater | `templater-obsidian` | `SilentVoid13/Templater` |
| Obsidian Git | `obsidian-git` | `Vinzent03/obsidian-git` |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Templater | Core Templates plugin | Core templates lack dynamic fields (tp.date, tp.file), no folder templates |
| Dataview | Manual links | No dynamic queries, manual maintenance of lists |

## Architecture Patterns

### Recommended Vault Structure
```
CityFarm/
├── .obsidian/
│   ├── app.json
│   ├── appearance.json
│   ├── community-plugins.json
│   ├── core-plugins.json
│   └── plugins/
│       ├── dataview/
│       │   ├── main.js
│       │   ├── manifest.json
│       │   └── data.json
│       ├── templater-obsidian/
│       │   ├── main.js
│       │   ├── manifest.json
│       │   ├── styles.css
│       │   └── data.json
│       └── obsidian-git/
│           ├── main.js
│           ├── manifest.json
│           ├── styles.css
│           └── data.json
├── _templates/
│   ├── knowledge-note.md
│   ├── reference-table.md
│   └── parameter-card.md
├── Home.md
├── Glossary.md
├── 01-Основы/                    # CEA, vertical farming, economics
├── 02-Системы-выращивания/       # NFT, DWC, aeroponics, drip, ebb-flow
├── 03-Питательные-растворы/      # Nutrients, pH, EC, mixing
├── 04-Культуры/                  # Lettuce, herbs, microgreens
├── 05-Микроклимат/               # Temperature, humidity, CO2, ventilation
├── 06-Освещение/                 # Spectrum, LED, PAR, photoperiod
├── 07-Проектирование/            # System design, components, mounting
├── 08-Помещения/                 # Facility selection, zoning, utilities
└── 09-Устойчивое-развитие/       # Sustainability, water recycling, ESG
```

**Design rationale:**
- 9 topic folders (consolidated from 11 modules: modules 4+5 merge into "Культуры", modules 10+11 business content is minimal for home project)
- Cyrillic folder names match vault language (Russian)
- Numbered prefixes for stable ordering in file explorer
- `_templates/` with underscore prefix to sort before content folders
- Flat structure (no sub-folders yet) -- can nest later as content grows

### Pattern 1: Topic-Based Organization
**What:** Group notes by topic domain, not by source module
**When to use:** Always in this vault -- per PROJECT.md and REQUIREMENTS.md constraint
**Why:** Cross-module topics (e.g., pH appears in modules 2, 3, 7, 8) should live together, not be scattered across module folders

### Pattern 2: YAML Frontmatter for Dataview
**What:** Every note has structured YAML frontmatter queryable by Dataview
**When to use:** All three note types
**Example:**
```yaml
---
title: "Азот в питании растений"
type: knowledge          # knowledge | reference | parameter
topic: питательные-растворы
tags:
  - макроэлементы
  - азот
  - питание-растений
source: module-03/урок-01
created: 2026-03-09
status: draft            # draft | review | final
---
```

**Key fields by note type:**

Knowledge note:
```yaml
type: knowledge
topic: <folder-topic>
tags: [...]
source: <module/lesson>
status: draft | review | final
```

Reference table:
```yaml
type: reference
topic: <folder-topic>
parameters: [pH, EC, температура, ...]
crops: [салат, базилик, ...]
```

Parameter card:
```yaml
type: parameter
parameter: pH
unit: "-"
optimal_range: "5.5-6.5"
sensor: "DFRobot pH V2"
alert_low: 5.0
alert_high: 7.0
```

### Pattern 3: Templater in Frontmatter
**What:** Use Templater functions for auto-populated fields
**Example template:**
```yaml
---
title: "<% tp.file.title %>"
type: knowledge
topic: ""
tags: []
source: ""
created: <% tp.date.now("YYYY-MM-DD") %>
status: draft
---

## Суть

## Детали

## Связанные заметки

```

**Important:** Templater processes functions bottom-to-top in files. Place simple fields (title, date) in frontmatter; complex logic in body.

### Anti-Patterns to Avoid
- **Module-based folders:** Don't create `module-01/`, `module-02/` -- use topics. pH content from modules 2, 3, 7 should be in one place.
- **English folder names in Russian vault:** Inconsistent. Use Cyrillic since all content is Russian.
- **Deep nesting:** Don't create `CityFarm/Растворы/Макроэлементы/Азот/` -- flat topic folders, use tags for sub-categorization.
- **Inline Dataview fields:** Use YAML frontmatter exclusively. Obsidian Properties natively supports YAML; inline `[key:: value]` syntax is legacy.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Note metadata queries | Custom index files | Dataview queries | Dynamic, auto-updating, filterable |
| Date/title insertion | Manual typing | Templater tp.date/tp.file | Consistent, error-free |
| Git sync | Manual git commands | Obsidian Git plugin | Auto-commit, auto-pull, conflict handling |
| Note type enforcement | Documentation conventions | Templater folder templates | Automatic template on note creation in folder |

## Common Pitfalls

### Pitfall 1: Plugin Files Not Committed to Git
**What goes wrong:** `.obsidian/plugins/` added to `.gitignore`, plugins not available after clone
**Why it happens:** Default Obsidian gitignore patterns exclude plugins
**How to avoid:** Commit full plugin folders (main.js, manifest.json, styles.css, data.json) to git. The files are small (< 1MB each). This ensures vault is functional immediately after clone.
**Warning signs:** Vault opens with "plugin not found" errors

### Pitfall 2: Templater Not Configured for Template Folder
**What goes wrong:** Templater doesn't recognize templates, no auto-insertion
**Why it happens:** `data.json` missing `templates_folder` setting
**How to avoid:** Set `"templates_folder": "_templates"` in Templater's `data.json`. Also set `"trigger_on_file_creation": true` for folder templates.

### Pitfall 3: Dataview Not Enabled
**What goes wrong:** Dataview codeblocks render as plain text
**Why it happens:** Plugin installed but not listed in `community-plugins.json` or data.json missing
**How to avoid:** Ensure `community-plugins.json` lists all three plugin IDs AND each plugin has a `data.json` with minimal config.

### Pitfall 4: Cyrillic Filenames in Git
**What goes wrong:** Git shows escaped Unicode in filenames (`\320\220\320\267\320\276\321\202`)
**Why it happens:** Git default `core.quotePath=true` escapes non-ASCII
**How to avoid:** Not a blocker -- Obsidian handles this fine. Can set `git config core.quotePath false` for readability in git log.

### Pitfall 5: STT Glossary Missing Common Errors
**What goes wrong:** Phase 2-3 processing can't correct STT errors because glossary is incomplete
**Why it happens:** Glossary created without analyzing actual transcript errors
**How to avoid:** Build glossary from observed errors in real transcripts. Sample analyzed from module_03/урок_1.txt shows patterns: word truncation ("кали" -> "калий"), wrong endings ("азод" -> "азот"), merged words ("дациент" -> "доцент").

## Code Examples

### Plugin Installation via CLI
```bash
# Download plugin from GitHub releases
# Pattern: https://github.com/{owner}/{repo}/releases/latest/download/{file}
VAULT=CityFarm/.obsidian/plugins

# Dataview
mkdir -p "$VAULT/dataview"
curl -sL "https://github.com/blacksmithgu/obsidian-dataview/releases/latest/download/main.js" -o "$VAULT/dataview/main.js"
curl -sL "https://github.com/blacksmithgu/obsidian-dataview/releases/latest/download/manifest.json" -o "$VAULT/dataview/manifest.json"
curl -sL "https://github.com/blacksmithgu/obsidian-dataview/releases/latest/download/styles.css" -o "$VAULT/dataview/styles.css"

# Templater
mkdir -p "$VAULT/templater-obsidian"
curl -sL "https://github.com/SilentVoid13/Templater/releases/latest/download/main.js" -o "$VAULT/templater-obsidian/main.js"
curl -sL "https://github.com/SilentVoid13/Templater/releases/latest/download/manifest.json" -o "$VAULT/templater-obsidian/manifest.json"
curl -sL "https://github.com/SilentVoid13/Templater/releases/latest/download/styles.css" -o "$VAULT/templater-obsidian/styles.css"

# Obsidian Git
mkdir -p "$VAULT/obsidian-git"
curl -sL "https://github.com/Vinzent03/obsidian-git/releases/latest/download/main.js" -o "$VAULT/obsidian-git/main.js"
curl -sL "https://github.com/Vinzent03/obsidian-git/releases/latest/download/manifest.json" -o "$VAULT/obsidian-git/manifest.json"
curl -sL "https://github.com/Vinzent03/obsidian-git/releases/latest/download/styles.css" -o "$VAULT/obsidian-git/styles.css"
```

### community-plugins.json
```json
[
  "dataview",
  "templater-obsidian",
  "obsidian-git"
]
```

### Templater data.json (minimal)
```json
{
  "templates_folder": "_templates",
  "trigger_on_file_creation": true,
  "auto_jump_to_cursor": true,
  "command_timeout": 5,
  "empty_file_template": "",
  "folder_templates": [],
  "syntax_highlighting": true,
  "enabled_templates_hotkeys": []
}
```

### Dataview data.json (minimal)
```json
{
  "renderNullAs": "—",
  "taskCompletionTracking": false,
  "warnOnEmptyResult": true,
  "recursiveSubTaskCompletion": false,
  "enableDataviewJs": false,
  "enableInlineDataview": false
}
```

### Obsidian Git data.json (minimal)
```json
{
  "autoSaveInterval": 5,
  "autoPullInterval": 10,
  "commitMessage": "vault: auto-backup {{date}}",
  "autoCommitMessage": "vault: auto-backup {{date}}",
  "autoPullOnBoot": true,
  "disablePush": false,
  "pullBeforePush": true
}
```

### Home.md Example
```markdown
---
title: Home
type: navigation
---

# CityFarm Knowledge Base

Структурированная база знаний по сити-фермерству.
Источник: курс "Сити-фермер: от идеи до PRO"

## Разделы

- [[01-Основы/|Основы сити-фермерства]] — CEA, вертикальные фермы, экономика
- [[02-Системы-выращивания/|Системы выращивания]] — NFT, DWC, аэропоника, капельный полив
- [[03-Питательные-растворы/|Питательные растворы]] — элементы питания, pH, EC
- [[04-Культуры/|Культуры]] — салаты, травы, микрозелень
- [[05-Микроклимат/|Микроклимат]] — температура, влажность, CO2
- [[06-Освещение/|Освещение]] — спектры, LED, расчёт освещённости
- [[07-Проектирование/|Проектирование систем]] — компоненты, монтаж, тестирование
- [[08-Помещения/|Выбор помещений]] — критерии, зонирование, инженерия
- [[09-Устойчивое-развитие/|Устойчивое развитие]] — рециркуляция воды, экология

## Справочные материалы

- [[Glossary|Глоссарий терминов]]

## О vault

- Язык: русский (технические термины на английском)
- Формат: wiki-ссылки `[[ссылка]]`
- Шаблоны: `_templates/` (knowledge note, reference table, parameter card)
```

### STT Glossary Format
```markdown
---
title: Глоссарий гидропонных терминов
type: reference
purpose: stt-correction
---

# Глоссарий

Домен-специфичные термины для коррекции ошибок распознавания речи (STT).

## Элементы питания

| Правильно | STT-варианты | English | Категория |
|-----------|-------------|---------|-----------|
| азот | азод, азод, азода | nitrogen | макроэлемент |
| фосфор | фосфар, фосфора | phosphorus | макроэлемент |
| калий | кали, кали й | potassium | макроэлемент |
| кальций | кальцы, кальци | calcium | макроэлемент |
| магний | магни, магни й | magnesium | макроэлемент |
| железо | железа, желез | iron | макроэлемент |
| марганец | марганец | manganese | микроэлемент |
| молибден | молевден, молебден | molybdenum | микроэлемент |
| цинк | цинг, цинк | zinc | микроэлемент |
| бор | бор | boron | микроэлемент |
| медь | медь | copper | микроэлемент |

## Гидропонные системы

| Правильно | STT-варианты | English | Категория |
|-----------|-------------|---------|-----------|
| гидропоника | гидропоник, гидрапоника | hydroponics | система |
| аэропоника | аеропоника, аэропоник | aeroponics | система |
| субстрат | субстрад, субстрат | substrate | материал |
| рециркуляция | рецеркуляция, рициркуляция | recirculation | процесс |

## Параметры среды

| Правильно | STT-варианты | English | Категория |
|-----------|-------------|---------|-----------|
| электропроводность | электропровадность, электро-проводность | electrical conductivity (EC) | параметр |
| кислотность | кислотность | acidity (pH) | параметр |
| фотосинтез | фотосинтэз, фота синтез | photosynthesis | процесс |
| транспирация | транспирацы, транспирация | transpiration | процесс |

## Общие термины курса

| Правильно | STT-варианты | English | Категория |
|-----------|-------------|---------|-----------|
| доцент | дациент, дацент | associate professor | должность |
| агрохимия | агрохими, агра химия | agrochemistry | наука |
| органические | органическе, органически | organic | тип |
| минеральные | минеральны, менеральные | mineral | тип |
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Inline Dataview `[key:: value]` | YAML frontmatter Properties | Obsidian 1.4+ (2023) | Use Properties panel, native support |
| Core Templates plugin | Templater for dynamic content | Stable since 2022 | tp.date, tp.file, folder templates |
| Manual git | Obsidian Git plugin | Mature since 2023 | Auto-commit, auto-pull, no CLI needed |

**Deprecated/outdated:**
- Inline Dataview fields: Still work but YAML frontmatter is the modern approach
- `tp.frontmatter` module: Does not work on file creation (known issue #1290), use raw YAML in templates instead

## Open Questions

1. **Exact STT error patterns across all modules**
   - What we know: Module 3 lesson 1 shows clear patterns (word truncation, wrong endings, merged words)
   - What's unclear: Whether other modules have different error patterns (different lecturers, different recording quality)
   - Recommendation: Start with glossary based on module 3 analysis, expand during Phase 2 processing

2. **Plugin version pinning**
   - What we know: Latest versions work; downloading from `releases/latest` is simplest
   - What's unclear: Whether future updates could break vault config
   - Recommendation: Download latest at creation time, commit to git. Versions are frozen until user manually updates.

3. **Module-to-topic mapping granularity**
   - What we know: Modules 4+5 (salads+microgreens) naturally merge into "Культуры"; Modules 10+11 (facilities+business) are less relevant for home project
   - What's unclear: Whether user wants business content (module 11) at all
   - Recommendation: Include all 9 topic folders but note module 11 content is low priority. Business folder can remain sparse.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual Obsidian validation (no automated test framework applicable) |
| Config file | N/A -- file-based vault, validated by opening in Obsidian |
| Quick run command | `ls -la CityFarm/.obsidian/plugins/*/main.js && echo "plugins present"` |
| Full suite command | `find CityFarm/ -name "*.md" \| wc -l && cat CityFarm/.obsidian/community-plugins.json` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| VAULT-01 | Topic-based folder structure exists | smoke | `ls -d CityFarm/0[1-9]-*/` | N/A -- Wave 0 |
| VAULT-02 | Home.md exists with wiki-links | smoke | `head -20 CityFarm/Home.md` | N/A -- Wave 0 |
| VAULT-03 | Templates have YAML frontmatter | smoke | `head -5 CityFarm/_templates/knowledge-note.md` | N/A -- Wave 0 |
| VAULT-04 | Plugin files downloaded and configured | smoke | `ls CityFarm/.obsidian/plugins/*/main.js` | N/A -- Wave 0 |
| VAULT-05 | Glossary file exists with term tables | smoke | `grep -c "STT-варианты" CityFarm/Glossary.md` | N/A -- Wave 0 |

### Sampling Rate
- **Per task commit:** Quick file existence checks
- **Per wave merge:** Full structure verification
- **Phase gate:** Open vault in Obsidian, verify plugins load, create note from template

### Wave 0 Gaps
None -- this phase creates files from scratch; no test infrastructure needed beyond shell commands for file verification.

## Sources

### Primary (HIGH confidence)
- Project files: `CFT/docs/city-farm-knowledge/00-INDEX.md` -- full module content map
- Project files: `CFT/docs/transcripts_by_modules/module_03/урок_1.txt` -- real STT error examples
- Project files: Kohan vault `.obsidian/` structure -- existing plugin pattern reference
- [Templater settings docs](https://silentvoid13.github.io/Templater/settings.html) -- configuration options
- [Dataview metadata docs](https://blacksmithgu.github.io/obsidian-dataview/annotation/add-metadata/) -- YAML frontmatter approach
- [Obsidian Git GitHub](https://github.com/Vinzent03/obsidian-git) -- plugin repo and releases

### Secondary (MEDIUM confidence)
- [Obsidian community-plugins.json](https://github.com/obsidianmd/obsidian-releases/blob/master/community-plugins.json) -- plugin registry
- [Obsidian help: Templates](https://help.obsidian.md/plugins/templates) -- core vs Templater comparison

### Tertiary (LOW confidence)
- STT error patterns extrapolated from single transcript file -- needs validation across more modules in Phase 2

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- plugins are well-established, user already uses Obsidian
- Architecture: HIGH -- topic structure derived from concrete content map (00-INDEX.md)
- Pitfalls: HIGH -- based on real project patterns (Kohan vault) and official docs
- Glossary: MEDIUM -- STT errors verified from one transcript, need broader validation

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable domain, no fast-moving dependencies)
