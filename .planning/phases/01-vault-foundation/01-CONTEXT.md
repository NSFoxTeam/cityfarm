# Phase 1: Vault Foundation - Context

**Gathered:** 2026-03-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Создать Obsidian vault `CityFarm/` в корне проекта cityfarm с topic-based структурой папок, шаблонами заметок (Templater), настроенными плагинами (Dataview, Templater, Obsidian Git) и доменным глоссарием гидропонных терминов для коррекции STT-ошибок.

</domain>

<decisions>
## Implementation Decisions

### Claude's Discretion
Все решения по реализации оставлены на усмотрение Claude:

- **Структура папок** — тематические разделы vault (на основе анализа 11 модулей курса), глубина вложенности, именование папок
- **Шаблоны заметок** — поля YAML frontmatter, структура тела заметки для каждого типа (knowledge note, reference table, parameter card)
- **Home.md** — формат точки входа, группировка разделов, стиль навигации
- **Доменный глоссарий** — формат файла, категории терминов, русский/английский маппинг, покрытие STT-ошибок
- **Настройка плагинов** — конфигурация Dataview, Templater, Obsidian Git

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches.

Ориентиры из существующих материалов:
- В `CFT/docs/city-farm-knowledge/00-INDEX.md` есть структура по модулям (модуль → тема) — полезно как карта содержания
- В `CFT/docs/transcripts_by_modules/` — 82 файла .txt по 11 модулям (материал для обработки в Phase 2-3)
- Модули 1-2 уже имеют сводки в CFT, модули 3-11 — сырые транскрипции

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- Нет существующего vault — создаём с нуля
- `CFT/docs/city-farm-knowledge/00-INDEX.md` — индекс существующей базы знаний, может информировать topic structure

### Established Patterns
- Проект уже использует Obsidian для проектной памяти (`ObsidianDB/`)
- Wiki-ссылки `[[ссылка]]` — стандартный формат для Obsidian
- Язык vault — русский (технические термины на английском допускаются)

### Integration Points
- Vault `CityFarm/` в корне проекта (рядом с `firmware/`, `backend/`, `frontend/`, `ml/`)
- Obsidian Git для синхронизации через Git
- Phase 2+ будут наполнять vault контентом из `CFT/docs/transcripts_by_modules/`
- Phase 5 свяжет параметры vault с IoT-платформой (датчики, пороги, алерты)

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-vault-foundation*
*Context gathered: 2026-03-09*
