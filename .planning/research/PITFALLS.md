# Domain Pitfalls

**Domain:** Knowledge base creation from course transcriptions (AI-processed)
**Researched:** 2026-03-09
**Confidence:** HIGH (based on direct analysis of source transcripts + research)

## Critical Pitfalls

Mistakes that cause fundamentally broken knowledge base or require full rework.

### Pitfall 1: Propagating Speech-to-Text Errors as Facts

**What goes wrong:** Raw transcriptions contain systematic STT errors that look plausible but are factually wrong. The LLM processing the transcript may treat garbled text as authoritative and either propagate the error or hallucinate a "correction" that is also wrong.

**Why it happens:** Actual evidence from the project's own transcripts (module 5): "микрозелин" instead of "микрозелень", "крышок" instead of "корешок", "семидольных" instead of "семядольных", "бабилив" instead of "бэби-лив", "вращивания" instead of "выращивание", "стебилек" instead of "стебелёк". These are consistent phonetic corruptions from Russian speech recognition. An LLM may either: (a) faithfully reproduce the error into the knowledge base, or (b) "correct" it to something plausible but wrong (e.g., interpreting "бабилив" as a brand name rather than "baby leaf").

**Consequences:** Domain-specific terminology gets corrupted throughout the vault. Users searching for "микрозелень" won't find notes written as "микрозелин". Numerical values mentioned in garbled context may be attached to the wrong parameter.

**Prevention:**
- Pre-process transcripts with a terminology correction pass before knowledge extraction. Build a domain glossary (hydroponic terms) and use it as a reference during processing.
- Cross-reference extracted terms against the PDF course materials (which have correct spelling as they were authored, not transcribed).
- Never process raw transcripts in a single shot -- first clean, then extract.

**Detection:** Search the output vault for common STT artifacts: words ending in unexpected suffixes, inconsistent spelling of the same term across notes, terms that don't appear in any standard hydroponic reference.

**Phase:** Must be addressed in Phase 1 (transcript preprocessing), before any knowledge extraction begins.

---

### Pitfall 2: Hallucinated Numerical Values

**What goes wrong:** LLMs fabricate or misattribute specific numbers when processing transcripts. Research shows up to 75% hallucination rates in multi-document LLM summarization (ACL 2025 findings). For a hydroponic knowledge base, wrong pH ranges (e.g., 6.5 stated as 7.5), wrong EC values, wrong temperature thresholds directly lead to dead plants.

**Why it happens:** Transcripts mention numbers in conversational context ("pH should be around six, maybe six and a half"). The LLM may round, interpolate, or confuse which parameter a number belongs to. When processing multiple lessons that discuss similar parameters, cross-contamination of values between different crops or growth stages is likely.

**Consequences:** The entire point of this knowledge base is to provide reference values for the CityFarm IoT platform (sensor thresholds, alerts). Wrong numbers make the knowledge base actively harmful rather than just useless.

**Prevention:**
- Extract numerical parameters into a separate, dedicated pass with explicit source attribution (module, lesson, timestamp/context).
- Cross-validate every extracted number against the PDF course materials.
- For critical parameters (pH, EC, temperature, humidity, light), create a verification checklist that requires two independent sources.
- Structure numerical data as tables with explicit source columns, not inline text where errors hide.

**Detection:** Compare extracted values against known hydroponic reference ranges. Flag any pH outside 4.0-8.0, EC outside 0.5-4.0 mS/cm, temperatures outside 10-35C as suspicious. Cross-check values between related notes (same crop should have consistent parameters).

**Phase:** Phase 2 (knowledge extraction) with mandatory verification in Phase 3 (structuring into reference tables).

---

### Pitfall 3: Module-Shaped Notes Instead of Topic-Shaped Notes

**What goes wrong:** Despite intending to organize by topic, the knowledge base ends up mirroring the course module structure. Notes have titles like "Lighting" but their content follows the order and framing of Module 7 rather than synthesizing lighting information from all modules.

**Why it happens:** Processing module by module is the natural approach. Each module generates its own notes. Merging and cross-referencing across modules requires a second pass that often gets skipped or done superficially. The LLM produces clean-looking output from each module, creating a false sense of completeness.

**Consequences:** Fragmented knowledge. The same crop (e.g., lettuce) has its lighting needs in one note, its nutrient needs in another, its temperature needs in a third, with no unified "growing lettuce" reference. The vault becomes a reformatted course, not a reference system.

**Prevention:**
- Define the target topic structure (taxonomy) BEFORE processing any modules. Decide on note titles, categories, and the information each note should contain.
- Process ALL modules first into intermediate structured data, THEN synthesize into topic-based notes in a second pass.
- Use a crop-centric cross-reference: for each crop mentioned across modules, consolidate all parameters into one reference card.

**Detection:** Open the graph view in Obsidian. If notes cluster by module rather than by topic, the structure failed. Check if any note references information from 3+ modules -- if not, cross-referencing hasn't happened.

**Phase:** Phase 1 (define taxonomy) and Phase 3 (cross-module synthesis). The taxonomy must exist before extraction begins.

---

### Pitfall 4: Losing Practical Knowledge by Over-Summarizing

**What goes wrong:** LLM summarization strips out the practical, experience-based advice (the "tricks of the trade") while preserving the textbook-style definitions. The knowledge base ends up with perfect definitions but no actionable guidance.

**Why it happens:** Practical advice in lectures is often delivered informally, as asides, anecdotes, or warnings ("by the way, never do X because..."). These are structurally less prominent than the main topic and LLMs tend to classify them as noise during summarization. The existing Module 5 summary demonstrates this: it lists "Контроль влажности" as a bullet point but contains zero specific guidance on what humidity level, what happens when it's wrong, or what the lecturer's practical advice was.

**Consequences:** The knowledge base becomes a glossary rather than a reference. It can tell you what pH is but not what to do when pH drifts at 2am. The CityFarm project needs operational knowledge (what to monitor, what thresholds to set, how to respond to deviations), not textbook definitions.

**Prevention:**
- Extraction prompt must explicitly ask for: warnings, exceptions, practical tips, troubleshooting advice, personal recommendations from the lecturer.
- Use a two-tier extraction: first extract the structured topic content, then specifically extract practical/operational advice as a separate pass.
- For each extracted concept, require an "operational implications" section: what does this mean for day-to-day growing?

**Detection:** Review generated notes for the ratio of definitional content vs. actionable content. If a note about "Nutrient Solution" contains 80% "what nutrients are" and 20% "how to manage nutrients in practice", it's over-summarized.

**Phase:** Phase 2 (knowledge extraction). The extraction prompts must be designed to capture this from the start.

---

## Moderate Pitfalls

### Pitfall 5: Context-Free Wiki Links (Link Soup)

**What goes wrong:** Notes are aggressively interlinked with wiki-links but the links lack context. A note about pH links to [[Nutrient Solution]] and [[Lettuce]] and [[EC]] without explaining the relationship. The graph looks impressive but navigation is confusing.

**Prevention:**
- Every wiki-link should appear in a sentence that explains the relationship: "pH directly affects [[Nutrient Solution]] absorption rates" rather than just "See also: [[Nutrient Solution]]".
- Limit links to genuinely related concepts. A note about pH does not need to link to every crop.
- Use link context (Obsidian shows surrounding text in backlinks panel) -- ensure the surrounding sentence is meaningful.

**Detection:** Open any note's backlinks panel. If the surrounding context for most backlinks is just a bullet list, linking is too shallow.

**Phase:** Phase 3 (structuring and linking).

---

### Pitfall 6: Ignoring PDF Course Materials as Ground Truth

**What goes wrong:** Processing only the transcripts and ignoring the PDF course materials (which contain diagrams, tables, and clean text authored by the course creator). The PDFs are the authoritative source; transcripts are a lossy derivative.

**Prevention:**
- Extract reference tables and diagrams from PDFs first. Use these as the skeleton for the knowledge base.
- Use transcripts to add depth, practical advice, and context around the structured content from PDFs.
- When transcript and PDF disagree, PDF wins (it was authored and reviewed; the transcript is speech recognition of a lecture).

**Detection:** Check if any note cites or references PDF content. If all notes are purely transcript-derived, PDFs were underutilized.

**Phase:** Phase 1 (source inventory and preprocessing). PDFs should be processed before or alongside transcripts.

---

### Pitfall 7: No Source Attribution Trail

**What goes wrong:** Generated notes have no indication of which module, lesson, or timestamp the information came from. When a value seems wrong or a claim needs verification, there's no way to trace it back to the source.

**Prevention:**
- Every note should include a metadata section or frontmatter listing source modules/lessons.
- For critical claims (especially numerical values), include inline source references: "(Модуль 9, урок 3)".
- This is especially important given the STT error problem -- attribution enables verification.

**Detection:** Pick any factual claim in the vault and try to trace it to a specific transcript. If you can't, attribution is missing.

**Phase:** Phase 2 (knowledge extraction). Must be built into extraction prompts.

---

### Pitfall 8: Processing All 82 Files in One Shot

**What goes wrong:** Attempting to process all transcripts at once (or in very large batches) leads to context window overflow, lost information from middle sections, and inconsistent quality between early and late outputs.

**Prevention:**
- Process one module at a time into intermediate notes.
- Within each module, process lesson by lesson if transcripts are long.
- Use a consistent extraction template across all modules to ensure comparable output.
- After per-module extraction, do a cross-module synthesis pass.

**Detection:** Compare quality of notes derived from Module 1 vs Module 11. If later modules have thinner, less detailed notes, batch processing caused degradation.

**Phase:** Phase 2 (knowledge extraction). The processing pipeline design must account for this.

---

## Minor Pitfalls

### Pitfall 9: Over-Engineering Vault Structure Upfront

**What goes wrong:** Spending excessive time designing folder hierarchies, tag taxonomies, templates, and plugins before any content exists. The structure ends up not matching the actual content.

**Prevention:**
- Start with a minimal structure: 3-5 top-level folders based on the known course topics. Refine after the first 2-3 modules are processed.
- Tags can be added retroactively; folder structure is harder to change.

**Phase:** Phase 1 (vault setup). Keep it minimal.

---

### Pitfall 10: Bilingual Inconsistency

**What goes wrong:** Technical terms appear inconsistently in Russian and English across notes. "EC" vs "ЭП" (электропроводность), "pH" vs "пш", "NFT" (Nutrient Film Technique) vs "НФТ". Search becomes unreliable.

**Prevention:**
- Define a term convention upfront: use established international abbreviations (pH, EC, NFT, DWC) with Russian explanations on first use.
- Include a glossary note that maps Russian terms to international abbreviations.

**Detection:** Search for the same concept using both Russian and English terms. If results don't overlap, there's inconsistency.

**Phase:** Phase 1 (define conventions). Enforce during Phase 2 (extraction).

---

### Pitfall 11: Treating Summaries as Already-Processed Modules

**What goes wrong:** Modules 1-2 already have summaries (СВОДКА files), and it's tempting to treat them as "done". But the existing summaries vary wildly in quality -- Module 1's summary is detailed and structured, while Module 5's is a brief outline. Assuming all summaries are adequate skips necessary re-processing.

**Prevention:**
- Audit all existing summaries against the target note quality standard before deciding to skip re-processing.
- Even well-processed modules may need restructuring to fit the new topic-based (not module-based) organization.

**Phase:** Phase 1 (source inventory and quality audit).

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Source inventory and prep | Skipping PDF materials, treating all transcripts as equal quality | Audit every source file; rank by quality; prioritize PDFs as ground truth |
| Taxonomy design | Over-engineering OR mirroring module structure | Start with 5-7 topic areas from course overview; iterate after first module |
| Transcript cleaning | Propagating STT errors, especially domain terms | Build hydroponic glossary first; automated find/replace for known corruptions |
| Knowledge extraction | Hallucinated numbers, lost practical advice, no attribution | Explicit extraction prompts; separate passes for facts vs. practical tips; mandatory source refs |
| Cross-module synthesis | Notes stay module-shaped; no crop reference cards | Dedicated synthesis pass after all modules; crop-centric consolidation |
| Vault linking | Link soup without context; inconsistent terminology | Link in sentences; enforce term conventions; glossary note |
| Verification | Trusting LLM output without spot-checking | Sample-check 20% of numerical values against PDFs; verify terminology |

## Sources

- Direct analysis of project transcripts in `/Users/nsfox/Development/CFT/docs/transcripts_by_modules/`
- Direct analysis of existing knowledge base in `/Users/nsfox/Development/CFT/docs/city-farm-knowledge/`
- [How LLMs Hallucinate in Multi-Document Summarization (ACL 2025)](https://aclanthology.org/2025.findings-naacl.293/)
- [Hallucination detection and mitigation framework for text summarization](https://www.nature.com/articles/s41598-025-31075-1)
- [Top Mistakes to Avoid with AI Speech to Text Tools](https://notegpt.io/blog/ai-speech-to-text-mistakes-to-avoid)
- [AI Knowledge Base Guide 2026](https://www.knowledgebase.net/ai-knowledge-base/)
- [Atomic notes and Obsidian best practices](https://medium.com/@shehab2003magdy/how-to-create-your-own-knowledge-base-with-obsidian-cc16e2e206f5)
- [Conceptual Lecture Notes in Obsidian](https://www.aidanhelfant.com/my-step-by-step-process-for-taking-conceptual-lecture-notes-in-obsidian/)
