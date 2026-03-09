# Roadmap: CityFarm Knowledge Base

## Overview

Transform 82 raw course transcription files into a structured Obsidian knowledge base organized by topic. The vault serves as the agronomic reference for CityFarm IoT decisions -- sensor thresholds, alert rules, dashboard parameters. The journey: scaffold the vault, validate the processing pipeline on the hardest module, batch-process the rest, cross-link everything, then distill reference tables that feed directly into firmware and backend.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Vault Foundation** - Obsidian vault structure, templates, plugins, and domain glossary
- [ ] **Phase 2: Pilot Processing** - Process Module 3 (Nutrient Solutions) end-to-end to validate the pipeline
- [ ] **Phase 3: Batch Processing** - Process all remaining modules (1-2 restructure, 4-11 from transcripts)
- [ ] **Phase 4: Cross-Linking** - Wiki-links between notes and tag system across the entire vault
- [ ] **Phase 5: Reference Tables & IoT Integration** - Parameter reference tables and CityFarm sensor/alert mapping

## Phase Details

### Phase 1: Vault Foundation
**Goal**: The vault exists with a clear structure, ready to receive processed content
**Depends on**: Nothing (first phase)
**Requirements**: VAULT-01, VAULT-02, VAULT-03, VAULT-04, VAULT-05
**Success Criteria** (what must be TRUE):
  1. `CityFarm/` directory exists in project root with topic-based folder structure (not module-based)
  2. Home.md opens as the vault entry point and links to all top-level sections
  3. Creating a new note from any template (knowledge note, reference table, parameter card) produces a file with correct YAML frontmatter
  4. Dataview, Templater, and Obsidian Git plugins are installed and functional
  5. Domain glossary of hydroponic terms exists and can be used for STT error correction during processing
**Plans**: 2 plans

Plans:
- [ ] 01-01-PLAN.md — Vault structure, Home.md entry point, and Templater note templates
- [ ] 01-02-PLAN.md — Obsidian plugins (Dataview, Templater, Git) and domain glossary for STT correction

### Phase 2: Pilot Processing
**Goal**: Module 3 (Nutrient Solutions, 12 lessons) is fully processed into atomic topic-based notes, validating the entire cleanup-and-extraction pipeline
**Depends on**: Phase 1
**Requirements**: PROC-01
**Success Criteria** (what must be TRUE):
  1. All 12 Module 3 transcripts are cleaned (STT errors fixed using glossary) and structured
  2. Knowledge is extracted into atomic notes (one concept per file) following vault templates
  3. Numeric parameters (pH, EC ranges for nutrient solutions) are extracted into reference format with source attribution
  4. Processing prompts are validated and reusable for batch processing in Phase 3
**Plans**: TBD

Plans:
- [ ] 02-01: TBD

### Phase 3: Batch Processing
**Goal**: All remaining course modules are processed into the vault -- modules 1-2 restructured from existing summaries, modules 4-11 from raw transcripts
**Depends on**: Phase 2
**Requirements**: PROC-02, PROC-03, PROC-04, PROC-05
**Success Criteria** (what must be TRUE):
  1. Modules 1-2 existing summaries are restructured into topic-based atomic notes matching Phase 2 output format
  2. Modules 4-6 (salads/herbs, microgreens, sustainable development) are processed from transcripts into atomic notes
  3. Modules 7-9 (system design, microclimate, lighting) are processed from transcripts into atomic notes
  4. Modules 10-11 (premises, business basics) are processed from transcripts into atomic notes
  5. All notes follow the same template schema and frontmatter conventions established in Phase 1-2
**Plans**: TBD

Plans:
- [ ] 03-01: TBD
- [ ] 03-02: TBD
- [ ] 03-03: TBD

### Phase 4: Cross-Linking
**Goal**: All notes in the vault are interconnected through wiki-links and categorized with a consistent tag system
**Depends on**: Phase 3
**Requirements**: LINK-01, LINK-02
**Success Criteria** (what must be TRUE):
  1. Related notes are connected via `[[wiki-links]]` where one topic references another (not link-for-linking-sake)
  2. A consistent tag taxonomy is applied across all notes (domain tags, content type tags, status tags)
  3. Navigating from Home.md, a user can reach any note within 2-3 clicks through links
**Plans**: TBD

Plans:
- [ ] 04-01: TBD

### Phase 5: Reference Tables & IoT Integration
**Goal**: The vault delivers its highest-value output -- aggregated parameter tables and direct mapping to CityFarm sensors, thresholds, and alerts
**Depends on**: Phase 4
**Requirements**: REF-01, REF-02
**Success Criteria** (what must be TRUE):
  1. Reference tables exist for all key parameters (pH, EC, temperature, humidity, light intensity, CO2) with optimal ranges broken down by crop type
  2. Each parameter table entry includes source attribution (which module/lesson the value came from)
  3. IoT integration notes map each parameter to its CityFarm sensor, threshold value, alert condition, and recommended action
  4. A developer working on firmware or backend can look up any sensor threshold directly from the vault
**Plans**: TBD

Plans:
- [ ] 05-01: TBD
- [ ] 05-02: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Vault Foundation | 0/2 | Planning complete | - |
| 2. Pilot Processing | 0/1 | Not started | - |
| 3. Batch Processing | 0/3 | Not started | - |
| 4. Cross-Linking | 0/1 | Not started | - |
| 5. Reference Tables & IoT Integration | 0/2 | Not started | - |
