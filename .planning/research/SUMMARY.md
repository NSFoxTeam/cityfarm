# Research Summary: CityFarm Knowledge Base

**Domain:** Structured Obsidian knowledge base from course transcriptions
**Researched:** 2026-03-09
**Overall confidence:** HIGH

## Executive Summary

This project transforms 82 raw course transcription files (~720KB total, Russian language) into a structured Obsidian knowledge base organized by knowledge domain. The source material covers 11 modules of a "City Farmer: From Idea to PRO" course, with modules 1-2 already summarized and modules 3-11 requiring full processing. An existing partial knowledge base exists in CFT but follows a module-based structure that needs to be reorganized by topic.

The recommended approach is a Claude Code sub-agent pipeline: merge per-module, clean in Pass 1 (with domain glossary to fix STT errors), extract topic-based knowledge notes in Pass 2. The corpus is small enough that API batch processing adds unnecessary complexity -- interactive Claude Code processing with prompt iteration is the right tool. The vault uses a minimal Obsidian plugin set (Dataview + Templater + Git), organizes content by knowledge domain via MOCs (Maps of Content) and light folder grouping, and uses flat YAML frontmatter properties for Dataview queries.

The highest-value output is the Parameter Reference tables -- aggregating all numeric thresholds (pH, EC, temperature, humidity, light intensity, CO2) from across the course. These directly feed CityFarm firmware sensor thresholds, backend alert rules, and frontend dashboard ranges. The vault also bridges course knowledge to project decisions via an IoT Integration section.

The two most dangerous pitfalls are: (1) STT transcription errors propagating as facts (especially domain-specific terms like "микрозелин" instead of "микрозелень"), and (2) LLM-hallucinated numerical values ending up in parameter tables that drive actual sensor thresholds. Both require explicit mitigation in the processing pipeline design.

## Key Findings

**Stack:** Claude Code sub-agents for transcript processing; Obsidian with Dataview + Templater + Git plugins; flat YAML frontmatter (no nesting -- Obsidian Properties don't support it); topic-based MOC navigation.

**Architecture:** Two-pass pipeline (cleanup then extraction); topic-based vault with MOCs as navigation hubs; atomic notes (one concept per file); reference tables as the bridge to CityFarm IoT.

**Critical pitfalls:** (1) STT errors propagating as facts -- needs domain glossary pre-processing. (2) Hallucinated numeric values in parameter tables -- needs cross-validation against PDF course materials. (3) Processing all modules before validating output format on a pilot module.

## Implications for Roadmap

Based on research, suggested phase structure:

1. **Vault Setup & Taxonomy** - Create vault structure, install plugins, create templates, define topic taxonomy and canonical note names
   - Addresses: Folder structure, frontmatter schema, Templater templates, domain glossary for STT correction
   - Avoids: Over-engineering (Pitfall 9) by keeping structure minimal; Module-shaped thinking (Pitfall 3) by defining topics first
   - Also: Build hydroponic term glossary for transcript cleanup

2. **Pilot Module Processing** - Process Module 3 (Nutrient Solutions) end-to-end as validation
   - Addresses: Transcript cleanup, knowledge extraction, parameter tables, wiki-linking
   - Avoids: Full rework by validating the entire pipeline on the most complex module (12 lessons, most IoT-relevant)
   - Why Module 3: Largest module, covers pH/EC which are the most critical IoT parameters

3. **Batch Module Processing** - Process remaining modules 4-11 (+ restructure existing 1-2)
   - Addresses: Full corpus processing with validated prompts
   - Avoids: Schema drift (Pitfall 6) by using proven templates; Quality degradation (Pitfall 8) by processing per-module, not all at once
   - Priority order: Module 8 (microclimate) -> 9 (lighting) -> 7 (system design) -> rest

4. **Cross-Module Synthesis & Linking** - Generate MOCs, cross-link notes, build crop reference cards
   - Addresses: Wiki-links, MOC navigation hubs, crop-specific parameter consolidation
   - Avoids: Link soup (Pitfall 5) by linking in context; Module-shaped notes (Pitfall 3) by synthesizing across modules

5. **Reference Tables & Dashboard** - Build Parameter Reference, Dashboard with Dataview queries
   - Addresses: The highest-value deliverable (numeric thresholds for CityFarm)
   - Avoids: Hallucinated values (Pitfall 2) by cross-validating against PDF course materials

6. **IoT Integration** - Create CityFarm-specific reference notes bridging knowledge to sensors/firmware
   - Addresses: Sensor thresholds, alert logic mapping, dashboard parameter documentation
   - Depends on: Firmware/backend maturity for actual sensor threshold values

**Phase ordering rationale:**
- Phase 1 before 2: Need taxonomy and glossary before extraction
- Phase 2 before 3: Must validate pipeline before scaling (the most important ordering decision)
- Phase 3 before 4: Need all content before cross-module synthesis
- Phase 4 before 5: Need linked notes before building aggregate views
- Phase 5 before 6: Need verified reference data before mapping to IoT

**Research flags for phases:**
- Phase 2: Needs prompt template iteration -- may take 2-3 tries to get extraction quality right
- Phase 3: Watch for quality degradation in later modules; spot-check against PDFs
- Phase 6: Needs coordination with firmware/backend for actual sensor threshold values

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Standard Obsidian plugins; Claude Code already in toolchain; well-understood LLM task |
| Features | HIGH | Requirements clearly defined in PROJECT.md; existing knowledge base shows the pattern |
| Architecture | HIGH | MOC + atomic notes is established Obsidian pattern; two-pass pipeline is standard |
| Pitfalls | HIGH | STT errors verified against actual project transcripts; hallucination risks well-documented in literature |

## Gaps to Address

- **Prompt templates:** Exact prompts for cleanup and extraction need iteration during pilot module (Phase 2)
- **Canonical note names:** Full list of standard note titles needs to be generated from taxonomy before batch processing
- **Domain glossary:** Hydroponic term list for STT correction needs to be built (can seed from existing knowledge base)
- **PDF integration:** Strategy for incorporating PDF course materials as ground truth verification needs refinement
- **Existing content migration:** Modules 1-2 summaries and `city-farm-knowledge/` files need a decision: restructure into new vault or re-extract from transcripts
- **Cyrillic file names in Git:** Needs testing to confirm `core.quotepath=false` handles display correctly
