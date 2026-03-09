# Feature Landscape

**Domain:** Obsidian knowledge base for hydroponic agriculture (agri-tech domain vault)
**Researched:** 2026-03-09

## Table Stakes

Features users expect. Missing = vault feels like a pile of files, not a knowledge base.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Topic-based folder structure | Навигация по предметной области, а не по модулям курса. Пользователь ищет "pH", а не "Модуль 3" | Low | 8-12 top-level folders: растворы, микроклимат, освещение, культуры, системы и т.д. |
| Wiki-ссылки между заметками | Ядро Obsidian. Без связей vault = набор файлов без контекста | Low | `[[Питательные растворы]]` из заметки про культуры |
| Frontmatter properties (YAML) | Фильтрация, сортировка, Dataview-запросы. Без properties нет структурированных данных | Low | Стандартный набор: `type`, `tags`, `created`, `source`, `status` |
| Справочные таблицы параметров | Основная ценность для IoT-проекта: конкретные числа pH, EC, температура, влажность, освещённость по культурам | Med | Отдельные заметки-справочники с таблицами. Привязка к firmware thresholds |
| Атомарные заметки | Каждая тема = отдельная заметка. Не монолитные файлы на 500 строк | Low | Разбить module-based файлы на atomic notes по темам |
| Tags (теги) | Поиск и фильтрация поперёк структуры папок | Low | `#параметр`, `#культура/салат`, `#система/NFT`, `#IoT-интеграция` |
| Шаблоны заметок (templates) | Единообразие заметок, ускорение создания новых | Low | 3-4 шаблона: культура, параметр, система выращивания, заметка из опыта |
| Index / Home note | Точка входа в vault. Обзор всех разделов | Low | `000 Home.md` со ссылками на все MOC |

## Differentiators

Features that make the vault genuinely useful vs just organized files.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Maps of Content (MOC) | Навигационные хабы по темам. MOC "Питательные растворы" связывает pH, EC, макроэлементы, рецепты. Одна заметка может быть в нескольких MOC без дупликации | Med | 6-8 MOC: Растворы, Микроклимат, Освещение, Культуры, Системы, Оборудование, IoT-интеграция |
| Dataview-запросы | Автоматические таблицы и списки из frontmatter. "Все культуры с pH < 6.0" или "Все параметры со статусом мониторинга" | Med | Требует дисциплины в frontmatter. Dataview plugin |
| Параметры-карточки с привязкой к IoT | Каждый контролируемый параметр (pH, EC, температура, влажность, освещённость) = отдельная заметка с полями: оптимальный диапазон, критические значения, действия при отклонении, привязка к датчику в firmware | High | Ключевая связка vault <-> CityFarm firmware/backend. Источник истины для threshold values |
| Культуры-карточки | Структурированные карточки по каждой культуре: условия выращивания, pH/EC диапазоны, цикл, система выращивания, экономика | Med | Стандартизированный шаблон. Dataview может собирать сводные таблицы |
| Связь "знание -> решение проекта" | Явные ссылки из агрономических знаний к решениям CityFarm: "pH диапазон 5.5-6.5 -> firmware threshold -> backend alert -> frontend gauge" | Med | Двунаправленные ссылки. Отдельная секция "Применение в проекте" в заметках |
| Журнал наблюдений (grow log) | Записи собственного опыта: что посадили, что выросло, какие проблемы, какие числа реально получили vs курс | Low | Шаблон с датой, культурой, параметрами, наблюдениями. Со временем = собственная база данных |
| Диагностические деревья | "Листья желтеют" -> возможные причины -> диагностика -> решение. Flowchart-like заметки для troubleshooting | Med | Mermaid diagrams в Obsidian. Привязка к алертам в CityFarm |
| Глоссарий с перекрёстными ссылками | Термины (CEA, NFT, DWC, PPFD, EC) как отдельные заметки. Автоматические backlinks показывают где термин используется | Low | Папка `glossary/`. Каждый термин = atomic note |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Точное копирование структуры курса (11 модулей) | Модули курса = порядок обучения, не порядок использования. Информация о pH разбросана по модулям 2, 3, 7 | Реструктурировать по темам предметной области |
| Сложная система плагинов | Obsidian plugin hell: больше времени на настройку, чем на знания. Vault должен работать и без плагинов | Минимум плагинов: Dataview, Templates, возможно Calendar. Всё остальное = чистый Markdown |
| Бизнес-план и финансовая модель | Явно out of scope (домашний проект). Захламляет vault нерелевантным контентом | Если нужно, ссылка на PDF в CFT |
| Перевод на английский | Vault на русском. Двуязычность = двойная работа без выгоды | Технические термины на английском где принято (pH, EC, NFT, LED) |
| Автоматическая синхронизация vault -> firmware config | Слишком сложно, хрупко, опасно. Vault = справочник для человека, не config source для машины | Ручная сверка: обновил знание -> обновил threshold в firmware |
| Копирование PDF-файлов в vault | Дублирование, раздувание vault. PDF плохо индексируются в Obsidian | Ссылки на PDF в CFT. Извлечь ключевые данные в Markdown-заметки |
| Вложенные папки глубже 2 уровней | Глубокая иерархия = friction при навигации. "Где я это положил?" | Плоская структура: `topics/subtopic.md`, не `topics/subtopics/sub-subtopics/note.md` |
| Canvas / Whiteboard для каждой темы | Красиво, но maintenance cost высокий. Canvas не индексируется search/Dataview | Mermaid diagrams в Markdown заметках для визуализации |

## Feature Dependencies

```
Frontmatter properties -> Dataview-запросы (Dataview требует properties)
Atomic notes -> Wiki-ссылки (сначала разбить, потом связать)
Atomic notes -> MOC (MOC ссылается на atomic notes)
Шаблоны -> Культуры-карточки (шаблон должен существовать до создания карточек)
Шаблоны -> Параметры-карточки (шаблон должен существовать до создания карточек)
Topic-based structure -> Всё остальное (сначала папки, потом наполнение)
Справочные таблицы -> Связь "знание -> решение проекта" (сначала числа, потом привязка)
Glossary -> Перекрёстные ссылки (сначала термины, потом ссылки на них)
```

## MVP Recommendation

Prioritize (Phase 1 — vault foundation):
1. **Topic-based folder structure** — скелет vault, определяет всё остальное
2. **Шаблоны заметок** — единообразие с первого дня
3. **Frontmatter properties** — стандартный набор полей, дисциплина сразу
4. **Index / Home note** — точка входа

Prioritize (Phase 2 — content extraction):
5. **Атомарные заметки из транскрипций** — основной контент vault
6. **Wiki-ссылки между заметками** — связи по мере создания
7. **Tags** — классификация по мере создания
8. **Справочные таблицы параметров** — ключевая ценность для IoT

Prioritize (Phase 3 — navigation and enrichment):
9. **MOC (Maps of Content)** — навигационные хабы после накопления контента
10. **Глоссарий** — после создания основного контента
11. **Культуры-карточки и Параметры-карточки** — структурированные данные

Defer:
- **Dataview-запросы** — полезны только когда vault наполнен и frontmatter стабилен
- **Диагностические деревья** — требуют собственного опыта, не только курса
- **Журнал наблюдений** — начинать когда реально выращиваем
- **Связь "знание -> решение проекта"** — итеративно, по мере развития firmware/backend

## Proposed Folder Structure

```
CityFarm/
  000 Home.md
  templates/
    template-culture.md
    template-parameter.md
    template-system.md
    template-observation.md
  moc/
    MOC Растворы.md
    MOC Микроклимат.md
    MOC Освещение.md
    MOC Культуры.md
    MOC Системы.md
    MOC Оборудование.md
  topics/
    растворы/          # pH, EC, макро/микроэлементы, рецепты
    микроклимат/       # температура, влажность, CO2, вентиляция
    освещение/         # спектры, фотопериод, LED, расчёты
    культуры/          # карточки по каждой культуре
    системы/           # NFT, DWC, Ebb & Flow, аэропоника
    проектирование/    # компоненты, монтаж, материалы
    устойчивость/      # рециркуляция, экология
  reference/
    параметры-по-культурам.md
    сравнение-систем.md
    спектры-освещения.md
  glossary/
    CEA.md
    NFT.md
    DWC.md
    ...
  journal/             # grow log, наблюдения (позже)
  project-links/       # связь с CityFarm IoT (позже)
```

## Proposed Frontmatter Schema

```yaml
---
type: culture | parameter | system | concept | reference | observation | moc
tags: []
created: YYYY-MM-DD
source: "course-module-N" | "experience" | "external"
status: draft | review | complete
related-parameter: []   # для культур: какие параметры контролировать
sensor: ""              # для параметров: какой датчик в CityFarm
optimal-range: ""       # для параметров: "5.5-6.5"
---
```

## Sources

- [Obsidian + AI Best Practices 2026](https://www.yppnote.com/en/posts/obsidian-ai-best-practices-2026/)
- [Obsidian Note Organization: Folders vs MOCs vs Tags](https://blog.shuvangkardas.com/obsidian-note-organization/)
- [How I use Obsidian -- Steph Ango](https://stephango.com/vault)
- [An opinionated reflection on using folders, links, tags, and properties](https://forum.obsidian.md/t/an-opinionated-reflection-on-using-folders-links-tags-and-properties/78548)
- [How to Organize a Knowledge Base in Obsidian Using Luhmann's Method](https://tekkix.com/articles/software/2026/02/how-to-organize-a-knowledge-base-in-obsidian)
- [Dataview Documentation](https://blacksmithgu.github.io/obsidian-dataview/)
- [Electrical Conductivity and pH Guide for Hydroponics - Oklahoma State](https://extension.okstate.edu/fact-sheets/electrical-conductivity-and-ph-guide-for-hydroponics.html)
- [List of pH & EC Levels for 65+ Hydroponic Vegetables](https://hydrohowto.com/ph-ec-hydroponic-vegetable/)
- Existing CFT knowledge base: `/Users/nsfox/Development/CFT/docs/city-farm-knowledge/`
