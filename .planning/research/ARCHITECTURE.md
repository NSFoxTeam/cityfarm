# Architecture Patterns

**Domain:** Obsidian Knowledge Base for Hydroponics/City Farming
**Researched:** 2026-03-09

## Recommended Architecture

A **topic-based vault with MOCs (Maps of Content)** combining light folder hierarchy with rich internal linking. This avoids the two extremes: deep folder nesting (rigid, hard to cross-reference) and flat chaos (unnavigable without search).

The vault name is `CityFarm/` (per PROJECT.md decision), lives at the repo root, syncs via Git.

### Folder Hierarchy

```
CityFarm/
  _templates/           # Note templates (Templater/core templates)
  _assets/              # Images, diagrams, attachments
  MOC/                  # Maps of Content (topic indexes)
    MOC-Гидропоника.md
    MOC-Питание.md
    MOC-Микроклимат.md
    MOC-Освещение.md
    MOC-Культуры.md
    MOC-Оборудование.md
    MOC-Проект-CityFarm.md
  Знания/               # Atomic knowledge notes (bulk of the vault)
    Гидропоника/        # Light sub-grouping by domain
    Питание/
    Микроклимат/
    Освещение/
    Культуры/
    Оборудование/
  Справочники/          # Reference tables (parameters, thresholds, formulas)
  Решения/              # Project decisions log
  Журнал/               # Grow logs, experiment notes (future)
  Home.md               # Vault entry point / dashboard
```

**Why this structure:**

- **`_templates/`** and **`_assets/`** prefixed with underscore to sort to top, standard Obsidian convention.
- **`MOC/`** is the navigation layer. Each MOC links to atomic notes across `Знания/` subfolders. One note can appear in multiple MOCs.
- **`Знания/`** holds atomic notes with light subfolder grouping. Subfolders mirror the domain, not the course modules (per PROJECT.md: "Структура по темам, не по модулям").
- **`Справочники/`** for reference tables (pH ranges, EC targets, temperature thresholds) -- these are the most directly useful for firmware/backend development.
- **`Решения/`** for project-specific decisions (what sensors chosen, why, calibration notes).
- **`Журнал/`** for future grow logs and experiment tracking.

### Design Rationale: Why Not Flat?

A fully flat structure with only MOCs and tags works for personal Zettelkasten but breaks down for domain knowledge bases where a collaborator (or AI agent) needs to browse. Light folders (6-8 subfolders under `Знания/`) give enough structure for navigation without locking notes into rigid hierarchies. The MOCs remain the primary navigation; folders are a convenience, not a constraint.

## Note Types

### 1. Home Dashboard (`Home.md`)

The vault entry point. Links to all MOCs, shows vault stats via Dataview, and provides quick-reference links.

**Properties:**
```yaml
---
type: dashboard
---
```

**Structure:**
- Welcome / purpose
- Links to all MOCs
- Quick reference (key parameters table)
- Dataview query: recently modified notes

### 2. Map of Content (MOC)

One per major domain topic. Curated list of links with brief context, organized by subtopic headers. MOCs link to atomic notes and to each other for cross-domain connections.

**Properties:**
```yaml
---
type: moc
domain: гидропоника
---
```

**Structure:**
- Topic overview (2-3 sentences)
- Sections with `##` headers grouping related links
- Each link has a one-line annotation
- Cross-references to related MOCs at bottom

**Example:** `MOC-Питание.md` links to notes on pH, EC, макроэлементы, микроэлементы, приготовление растворов, and cross-links to `MOC-Гидропоника.md` and `MOC-Культуры.md`.

### 3. Atomic Knowledge Note

The core unit. One concept per note. Extracted and distilled from course transcripts. Should be self-contained enough to be useful without reading the parent MOC.

**Properties:**
```yaml
---
type: knowledge
domain: питание
source: module-03
tags:
  - pH
  - раствор
---
```

**Structure:**
- `# Title` -- concept name
- Brief definition / explanation
- Key facts, numbers, ranges
- Practical application (how this relates to growing)
- `## Связь с CityFarm` -- how this applies to the project (optional)
- Wiki-links to related notes inline

**Naming convention:** Descriptive title in Russian, e.g., `pH-контроль-раствора.md`, `NFT-система.md`.

### 4. Reference Table

Structured data optimized for lookup. Parameter ranges, crop requirements, equipment specs. Heavy use of Markdown tables. These are the notes most directly consumed by firmware/backend development.

**Properties:**
```yaml
---
type: reference
domain: параметры
tags:
  - пороги
  - датчики
---
```

**Structure:**
- `# Title`
- One or more Markdown tables with parameter names, ranges, units
- Source attribution
- Notes on measurement conditions

**Examples:**
- `Пороги-датчиков.md` -- pH 5.5-6.5, EC 1.5-2.5 mS/cm, temp 20-30C, humidity 60-80%, light 10000-30000 lux
- `Требования-культур.md` -- per-crop parameter matrix
- `Спецификации-оборудования.md` -- sensor specs, calibration data

### 5. Decision Note

Records a project decision with context and rationale. Useful for understanding why CityFarm is built the way it is.

**Properties:**
```yaml
---
type: decision
date: 2026-03-09
status: accepted
tags:
  - датчики
---
```

**Structure:**
- `# Decision: [title]`
- Context / problem
- Options considered
- Decision and rationale
- Consequences / follow-up

### 6. Grow Log Entry (future)

Timestamped observation or experiment record.

**Properties:**
```yaml
---
type: log
date: 2026-03-09
crop: салат
tags:
  - эксперимент
---
```

## Linking Strategy

### Wiki-Links as Primary Navigation

Use Obsidian wiki-links `[[note-name]]` everywhere. These create the knowledge graph and enable backlinks.

**Rules:**
1. **Every atomic note** must be linked from at least one MOC.
2. **Inline linking** -- when mentioning a concept that has its own note, link it: "Раствор должен иметь [[pH-контроль-раствора|pH 5.5-6.5]]".
3. **Cross-MOC links** -- each MOC has a "Related" section linking to other MOCs.
4. **Backlinks panel** serves as automatic "what links here" -- no manual maintenance needed.

### Tags as Secondary Classification

Tags (`#tag`) complement links for filtering and Dataview queries. Use sparingly and consistently.

**Tag taxonomy:**
- Domain: `#гидропоника`, `#питание`, `#микроклимат`, `#освещение`, `#культуры`, `#оборудование`
- Type markers: `#справочник`, `#практика`, `#теория`
- Project: `#cityfarm` (for notes directly relevant to the IoT platform)

### Properties (YAML Frontmatter)

Every note gets frontmatter with at minimum `type` and `domain`. This enables Dataview queries and Bases views.

**Standard properties across all notes:**
| Property | Type | Values | Purpose |
|----------|------|--------|---------|
| `type` | text | dashboard, moc, knowledge, reference, decision, log | Note classification |
| `domain` | text | гидропоника, питание, микроклимат, освещение, культуры, оборудование, проект | Topic area |
| `source` | text | module-01 through module-11, experience, external | Origin of knowledge |
| `tags` | list | free-form | Searchable classification |

**Keep frontmatter flat** -- no nesting. Unique property names are sufficient.

## Patterns to Follow

### Pattern 1: Extract, Don't Copy

**What:** Distill course transcripts into atomic notes. Do not paste transcript chunks.
**Why:** Raw transcripts are verbose and unstructured. The value is in extraction and synthesis.
**Process:**
1. Read transcript section
2. Identify distinct concepts
3. Create one atomic note per concept
4. Extract specific numbers/ranges into reference tables
5. Link from appropriate MOC

### Pattern 2: MOC as Living Index

**What:** MOCs are actively maintained, not generated. They reflect how you think about the domain, not how the source material was organized.
**Why:** Course modules mix topics. MOC-Питание should pull from modules 3, 4, 5, 7 wherever nutrient info appears.

### Pattern 3: Reference Tables for IoT Integration

**What:** Every measurable parameter gets a reference table entry with: parameter name, unit, optimal range, danger thresholds, sensor type, measurement frequency.
**Why:** These tables directly feed firmware threshold configs and backend alert rules. This is the bridge between domain knowledge and code.

```markdown
| Параметр | Единица | Оптимум | Внимание | Критично | Датчик |
|----------|---------|---------|----------|----------|--------|
| pH | - | 5.5-6.5 | <5.0 / >7.0 | <4.5 / >7.5 | DFRobot pH V2 |
| EC | mS/cm | 1.5-2.5 | <1.0 / >3.0 | <0.5 / >3.5 | DFRobot TDS |
| Температура воздуха | C | 20-28 | <18 / >30 | <15 / >35 | DHT22 |
```

### Pattern 4: Bidirectional Project Links

**What:** Knowledge notes link to CityFarm project decisions. Decision notes link back to knowledge.
**Why:** Creates a traceable path: "Why is pH alert at 5.0?" -> Decision note -> Knowledge note on pH ranges -> Course module source.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Module-Based Organization

**What:** Organizing by course module numbers (Module 1, Module 2...).
**Why bad:** The existing CFT knowledge base does this. It mirrors how the course was taught, not how you look up information. Nobody thinks "I need Module 3" -- they think "What pH range for lettuce?"
**Instead:** Organize by domain topic. One concept may span multiple modules.

### Anti-Pattern 2: Mega-Notes

**What:** Putting all of Module 3 into one big file (the current `03-nutrient-management.md` pattern).
**Why bad:** Can't link to specific concepts. Can't reuse parts in different MOCs. Hard to maintain.
**Instead:** Atomic notes. One concept = one file. MOCs reassemble them into coherent views.

### Anti-Pattern 3: Deep Folder Nesting

**What:** `Знания/Питание/Макроэлементы/Азот/Формы-азота.md`
**Why bad:** Path becomes unwieldy. Moving notes breaks links (unless using wiki-links with unique names). Obsidian's strength is linking, not hierarchy.
**Instead:** Maximum 2 levels under `Знания/`. Use MOCs for deeper organization.

### Anti-Pattern 4: Tag Explosion

**What:** Dozens of tags per note, inconsistent naming, tags replacing links.
**Why bad:** Tags without discipline become noise. No one maintains tag taxonomy.
**Instead:** 2-5 tags per note. Use links for relationships, tags for classification.

### Anti-Pattern 5: Copy-Paste Transcripts

**What:** Pasting raw transcript text into notes.
**Why bad:** Transcripts have filler, repetition, off-topic tangents. Noise drowns signal.
**Instead:** Extract and synthesize. Reference the source module for attribution.

## Essential Plugins

| Plugin | Purpose | Priority |
|--------|---------|----------|
| **Templater** | Note templates with auto-populated fields | Must have |
| **Dataview** | Dynamic queries, tables, dashboards | Must have |
| **Calendar** | Navigate journal entries by date | Nice to have (for grow logs) |
| **Obsidian Git** | Sync vault via Git | Already planned |

Dataview enables dynamic reference tables like "all notes tagged #cityfarm sorted by domain" or "all reference tables with pH data." This is essential for a knowledge base this size.

## Suggested Build Order

The architecture implies this build sequence:

1. **Vault skeleton** -- Create folder structure, Home.md, templates, `.obsidian` config
2. **Reference tables first** -- Extract numerical parameters (pH, EC, temp, humidity, light ranges) from course materials. These have the highest immediate value for CityFarm firmware/backend.
3. **MOC stubs** -- Create all 6-7 MOCs with section headers but empty link lists. This defines the target structure.
4. **Atomic notes from processed modules** -- Modules 1-2 already have summaries. Break these into atomic notes, link from MOCs.
5. **Process remaining modules (3-11)** -- Extract knowledge from raw transcripts into atomic notes. Process in priority order: Module 3 (nutrients/pH/EC) -> Module 8 (microclimate) -> Module 9 (lighting) -> Module 7 (system design) -> rest.
6. **Decision notes** -- Document existing CityFarm project decisions (sensor choices, calibration values).
7. **Grow logs** -- Start when actual growing begins.

**Priority rationale:** Reference tables and nutrition/climate modules directly feed the IoT platform (firmware thresholds, backend alerts, frontend dashboard ranges). Business and sustainability modules are lower priority for a home project.

## Scalability Considerations

| Concern | At 50 notes | At 200 notes | At 500+ notes |
|---------|-------------|--------------|---------------|
| Navigation | Home + MOCs sufficient | MOCs become essential | Consider MOC-of-MOCs or nested MOCs |
| Search | Filename search works | Tags + properties + Dataview needed | Full Dataview integration required |
| Maintenance | Manual linking fine | Need regular MOC review | Consider automated orphan detection |
| Performance | No issues | No issues | Dataview queries may slow -- use folder scoping |

For this project, 100-200 notes is the realistic ceiling (11 course modules + project decisions + grow logs). The recommended structure handles this comfortably.

## Sources

- [Science Research Vault (Obsidian Forum)](https://forum.obsidian.md/t/science-research-vault-a-structured-workflow-for-academics/95589) - MEDIUM confidence
- [Scientific Research Vault (GitHub)](https://github.com/LalieA/obsidian-scientific-research-vault) - MEDIUM confidence, SCTO method
- [Maps of Content: Complete Guide](https://www.dsebastien.net/2022-05-15-maps-of-content/) - HIGH confidence, Nick Milo's MOC methodology
- [MOCs vs Folder Hierarchy (GitHub Discussion)](https://github.com/BryanJenksCommunity/FAQ/discussions/14) - MEDIUM confidence
- [Obsidian Properties (Official Docs)](https://help.obsidian.md/properties) - HIGH confidence
- [Obsidian Properties Best Practices (Forum)](https://forum.obsidian.md/t/obsidian-properties-best-practices-and-why/63891) - MEDIUM confidence
- [Dataview Plugin](https://blacksmithgu.github.io/obsidian-dataview/) - HIGH confidence
- [Building a Second Brain with Obsidian (Oxford)](https://www.blopig.com/blog/2026/02/building-a-second-brain-a-functional-knowledge-stack-with-obsidian/) - MEDIUM confidence
- [From Chaos to Clarity: Multi-Vault Approach](https://medium.com/obsidian-observer/from-chaos-to-clarity-my-multi-vault-approach-to-obsidian-knowledge-management-6868e5597a8c) - MEDIUM confidence
