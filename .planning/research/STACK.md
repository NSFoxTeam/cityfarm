# Technology Stack

**Project:** CityFarm Knowledge Base (Obsidian vault from course transcriptions)
**Researched:** 2026-03-09

## Recommended Stack

### Transcript Processing Pipeline

| Technology | Purpose | Why |
|------------|---------|-----|
| Claude Code (sub-agents) | Transcript cleanup, summarization, knowledge extraction | Already in the project toolchain; handles Russian text natively; can process structured prompts per-module |
| Python scripts (optional) | Pre-processing: merge per-module, strip filler phrases | Simple text manipulation before LLM pass; `improve_transcription.py` already exists in CFT |

**Confidence: HIGH** -- Claude Code is already the project's toolchain, and transcript processing is a well-understood LLM task.

### Processing Approach

Use a **two-pass pipeline**:

1. **Pass 1 -- Cleanup & Structure** (per lesson): Remove filler words/phrases, fix STT artifacts (see PITFALLS.md Pitfall 1), produce clean structured text
2. **Pass 2 -- Knowledge Extraction** (per module): Synthesize lessons into thematic knowledge notes with frontmatter, wiki-links, and reference tables

**Why two passes, not one:** Single-pass loses nuance. Pass 1 preserves all content cleanly; Pass 2 reorganizes by topic (not by lesson order). This matches the project requirement of "structure by topic, not by module."

**Why NOT Claude API Batch:** The total corpus is ~720KB across 82 files. This is small enough to process interactively via Claude Code sub-agents. API batch processing adds pipeline complexity for marginal cost savings on a corpus this size.

**Confidence: MEDIUM** -- Two-pass is standard for transcript-to-knowledge pipelines, but the exact prompt templates need iteration on a pilot module.

### Obsidian Vault Setup

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Obsidian | Latest | Knowledge base viewer/editor | Already used in the project (ObsidianDB pattern) |
| Obsidian Git plugin | Latest | Sync vault via Git | Already part of the workflow per CLAUDE.md |
| Dataview plugin | 0.5.x+ | Dynamic queries, parameter tables, dashboards | Query notes by frontmatter properties; essential for "find all notes where pH is mentioned" type lookups |
| Templater plugin | Latest | Note templates with frontmatter scaffolding | Consistent structure across all knowledge notes; auto-populate dates and tags |

**Confidence: HIGH** -- These are the standard Obsidian knowledge management plugins, widely used and well-maintained.

### Obsidian Plugins -- Minimal Set

| Plugin | Purpose | Why Include |
|--------|---------|-------------|
| Dataview | Query notes as data, build dashboards | Essential for parameter reference tables and cross-cutting queries |
| Templater | Template engine for consistent note structure | Ensures every note has correct frontmatter, structure |
| Obsidian Git | Git sync | Already in workflow |
| Calendar (optional) | Timeline view for grow logs | Only if tracking experiments over time |

**Do NOT install:**
- **Excalidraw** -- Overkill for this vault; diagrams live in PDF source materials
- **Kanban** -- Not a project management vault
- **Tasks** -- Not a task management vault; tasks live in `.planning/`
- **Slides / Presentation** -- Not a presentation tool
- **Any AI plugins (Smart Connections, Copilot, etc.)** -- Processing happens via Claude Code pipeline, not inside Obsidian

**Confidence: HIGH** -- Plugin minimalism is a well-established best practice. Each plugin adds load time and maintenance burden.

### Frontmatter Schema

Every knowledge note should use this YAML frontmatter:

```yaml
---
type: knowledge
domain: питание
source: module-03
tags:
  - pH
  - EC
  - раствор
status: draft
optimal-range: "pH 5.5-6.5, EC 1.5-2.5"
sensor: "DFRobot pH V2, DFRobot TDS"
created: 2026-03-09
---
```

**Why this schema:**
- `type` classifies note kind (knowledge, reference, moc, decision, log, dashboard) -- enables Dataview filtering
- `domain` enables Dataview grouping by topic area (питание, микроклимат, освещение, культуры, гидропоника, оборудование)
- `source` traces back to original course module for verification
- `tags` enable quick search and filtering
- `status` tracks processing state (draft/review/complete)
- `optimal-range` captures key numeric thresholds as text (Dataview can still filter)
- `sensor` links to CityFarm hardware for IoT integration
- **All properties are flat** (no nesting) -- Obsidian native Properties do not support nested YAML. Flat keys work with both Obsidian Properties UI and Dataview queries.
- Native Obsidian Properties (YAML) preferred over Dataview inline fields `[key:: value]` for better performance and broader compatibility

**Confidence: MEDIUM** -- Schema is sound but will need refinement once actual content is processed during pilot module.

### Vault Folder Structure

```
CityFarm/
  .obsidian/                    # Obsidian config (plugins, themes)
  _templates/                   # Templater templates
    knowledge-note.md
    reference-table.md
    grow-log.md
  _assets/                      # Images, diagrams (if any)
  MOC/                          # Maps of Content (navigation hubs)
    MOC-Питание.md
    MOC-Микроклимат.md
    MOC-Освещение.md
    MOC-Культуры.md
    MOC-Гидропоника.md
    MOC-Оборудование.md
    MOC-Проект-CityFarm.md
  Знания/                       # Atomic knowledge notes
    Гидропоника/
    Питание/
    Микроклимат/
    Освещение/
    Культуры/
    Оборудование/
  Справочники/                  # Reference tables (parameters, thresholds)
    Пороги-датчиков.md
    Требования-культур.md
  Решения/                      # Project decision records
  Журнал/                       # Grow logs (future)
  Home.md                       # Vault entry point / dashboard
  _sources/                     # Source tracking (not content)
    module-index.md
```

**Why this structure:**
- Organized by **domain topic**, not by course module (per project requirement)
- **MOC/** as the navigation layer: each MOC links to atomic notes across `Знания/` subfolders
- **Знания/** holds atomic notes with light subfolder grouping (max 2 levels deep)
- **Справочники/** for the highest-value output: numeric parameter tables for IoT integration
- **Решения/** for CityFarm project decisions with rationale
- `_templates/` and `_assets/` prefixed with underscore to sort to top
- `_sources/` tracks provenance without cluttering knowledge areas
- Flat-ish hierarchy -- Obsidian works best with shallow folder trees + links

**Confidence: HIGH** -- Aligns with project requirements and Obsidian best practices (Steph Ango's vault approach).

### Processing Toolchain (Execution)

| Step | Tool | Input | Output |
|------|------|-------|--------|
| 1. Merge lessons per module | Bash script | `CFT/docs/transcripts_by_modules/module_XX/урок_*.txt` | `merged_module_XX.txt` |
| 2. Clean transcript | Claude Code sub-agent | Merged transcript + cleanup prompt + domain glossary | Clean structured text |
| 3. Extract knowledge notes | Claude Code sub-agent | Clean text + extraction prompt + frontmatter template + taxonomy | Obsidian .md files with frontmatter |
| 4. Extract parameters | Claude Code sub-agent | Clean text + parameter extraction prompt | Reference tables in Справочники/ |
| 5. Generate wiki-links | Claude Code sub-agent | All generated notes + canonical name list | Notes with `[[wiki-links]]` added |
| 6. Validate | Manual review in Obsidian | Generated vault | Corrected knowledge base |

**Important:** Step 2 must include a domain glossary to fix STT errors (see PITFALLS.md). Step 3 requires a pre-defined taxonomy and canonical note name list to ensure consistent organization.

**Confidence: HIGH** -- Straightforward pipeline for a known-size corpus.

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| LLM for processing | Claude Code (sub-agents) | Claude API Batch | Corpus too small to justify batch infra; sub-agents are interactive and iteratable |
| LLM for processing | Claude Code | GPT-4o / Gemini | Claude already in toolchain; excellent Russian language support; no need to add another API |
| Note structure | Obsidian YAML Properties (flat) | Dataview inline fields `[key:: value]` | Native Properties have better performance, broader compatibility, and are the modern Obsidian standard |
| Note structure | Flat YAML properties | Nested YAML (params.pH_min) | Obsidian Properties UI does not support nested YAML; flat keys are more portable |
| Vault organization | Topic-based folders + MOCs | Module-based folders | Project requirement explicitly states topic-based; also better for reference use |
| Vault organization | Shallow folders + links | Deep folder hierarchy | Obsidian graph and search work best with flat structures connected by links |
| Template engine | Templater | Core Templates | Templater supports dynamic dates, conditional logic, and JavaScript -- more flexible for consistent frontmatter |
| Query engine | Dataview | Manual search | Parameter tables and dashboards are core requirements; Dataview makes them dynamic |

## Installation

```bash
# No package installation needed -- Obsidian is a desktop app
# Plugins installed via Obsidian Settings > Community Plugins:
# 1. Dataview
# 2. Templater
# 3. Obsidian Git (likely already installed)

# For transcript processing, ensure Claude Code is available (already is)
```

## Key Sizing Data

| Metric | Value |
|--------|-------|
| Total transcript files | 82 |
| Total corpus size | ~720 KB |
| Modules already summarized | 2 of 11 |
| Modules needing processing | 9 (modules 3-11) |
| Estimated output notes | 50-80 atomic knowledge notes + 6-7 MOCs + 2-3 reference tables |
| Estimated processing time | 2-4 hours with Claude Code sub-agents |

## Sources

- [Obsidian Vault Best Practices - Steph Ango (Obsidian CEO)](https://stephango.com/vault)
- [Obsidian + AI Best Practices 2026 Edition](https://www.yppnote.com/en/obsidian-ai-best-practices-2026/)
- [Dataview Documentation](https://blacksmithgu.github.io/obsidian-dataview/)
- [Dataview Metadata - Adding Metadata](https://blacksmithgu.github.io/obsidian-dataview/annotation/add-metadata/)
- [Templater GitHub](https://github.com/SilentVoid13/Templater)
- [Best Obsidian Plugins 2026](https://www.dsebastien.net/the-must-have-obsidian-plugins-for-2026/)
- [Claude API Batch Processing Docs](https://platform.claude.com/docs/en/build-with-claude/batch-processing)
- [Claude Code Batch Processing Guide](https://smartscope.blog/en/generative-ai/claude/claude-code-batch-processing/)
