# Requirements: CityFarm Knowledge Base

**Defined:** 2026-03-09
**Core Value:** Вся информация по сити-фармингу структурирована по темам и доступна как справочник для принятия решений по проекту CityFarm

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Vault Setup

- [x] **VAULT-01**: Obsidian vault `CityFarm/` создан в корне проекта cityfarm с topic-based структурой папок
- [x] **VAULT-02**: Home.md как точка входа с навигацией по разделам vault
- [x] **VAULT-03**: Templater-шаблоны для каждого типа заметки (knowledge note, reference table, parameter card) с YAML frontmatter
- [ ] **VAULT-04**: Obsidian plugins настроены: Dataview, Templater, Obsidian Git
- [ ] **VAULT-05**: Domain glossary гидропонных терминов для коррекции STT-ошибок в транскрипциях

### Content Processing

- [ ] **PROC-01**: Транскрипции модуля 3 (Питательные растворы, 12 уроков) обработаны пилотно — очищены, структурированы, извлечены знания
- [ ] **PROC-02**: Транскрипции модулей 1-2 переструктурированы из существующих сводок в topic-based atomic notes
- [ ] **PROC-03**: Транскрипции модулей 4-6 обработаны — салаты/травы, микрозелень, устойчивое развитие
- [ ] **PROC-04**: Транскрипции модулей 7-9 обработаны — проектирование систем, микроклимат, освещение
- [ ] **PROC-05**: Транскрипции модулей 10-11 обработаны — выбор помещений, бизнес-основы

### Linking & Tags

- [ ] **LINK-01**: Wiki-ссылки `[[ссылка]]` между связанными заметками по всему vault
- [ ] **LINK-02**: Система тегов для категоризации заметок (домен, тип контента, статус)

### Reference & Integration

- [ ] **REF-01**: Справочные таблицы параметров — pH, EC, температура, влажность, освещённость, CO₂ с оптимальными диапазонами по культурам
- [ ] **REF-02**: Привязка параметров к CityFarm IoT — маппинг: параметр → датчик → порог → алерт → действие

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Navigation

- **NAV-01**: Maps of Content (MOC) как навигационные хабы по каждой теме
- **NAV-02**: Dataview-запросы для динамических списков и фильтрации заметок

### Content Enrichment

- **ENRICH-01**: Карточки культур — отдельная заметка на каждую культуру с полными параметрами выращивания
- **ENRICH-02**: Кросс-валидация числовых значений с PDF-конспектами курса
- **ENRICH-03**: Grow log / журнал наблюдений — шаблон для записи собственного опыта выращивания

## Out of Scope

| Feature | Reason |
|---------|--------|
| Копирование PDF в vault | Останутся в CFT, слишком большие для Git-трекинга |
| Бизнес-план и финмодель | Не релевантно для домашнего проекта |
| Перевод на английский | Vault на русском, как и курс |
| AI-плагины для Obsidian | Обработка через Claude Code, не внутри Obsidian |
| Мультиязычный glossary | Русский + базовые англ. термины достаточно |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| VAULT-01 | Phase 1 | Complete |
| VAULT-02 | Phase 1 | Complete |
| VAULT-03 | Phase 1 | Complete |
| VAULT-04 | Phase 1 | Pending |
| VAULT-05 | Phase 1 | Pending |
| PROC-01 | Phase 2 | Pending |
| PROC-02 | Phase 3 | Pending |
| PROC-03 | Phase 3 | Pending |
| PROC-04 | Phase 3 | Pending |
| PROC-05 | Phase 3 | Pending |
| LINK-01 | Phase 4 | Pending |
| LINK-02 | Phase 4 | Pending |
| REF-01 | Phase 5 | Pending |
| REF-02 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0

---
*Requirements defined: 2026-03-09*
*Last updated: 2026-03-09 after plan 01-01 completion*
