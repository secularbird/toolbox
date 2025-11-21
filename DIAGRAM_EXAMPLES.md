# Diagram Examples for Wiki

This document demonstrates the PlantUML and Mermaid diagram support in the wiki.

## Mermaid Diagrams

### 1. Flowchart Example

```mermaid
graph TD
    A[Start] --> B{Is it?}
    B -->|Yes| C[OK]
    C --> D[Rethink]
    D --> B
    B ---->|No| E[End]
```

### 2. Sequence Diagram

```mermaid
sequenceDiagram
    autonumber
    actor Customer
    participant UI as Web UI
    participant S as OrderService
    participant DB as Database

    Customer->>UI: Click "View Orders"
    UI->>S: getOrders(userId)
    S->>DB: SELECT * FROM orders WHERE userId = ?
    DB-->>S: orders[]
    S-->>UI: orders[]
    UI-->>Customer: Render order list
```

### 3. Class Diagram

```mermaid
classDiagram
    class WikiPage {
        +String title
        +String content
        +Date created_at
        +Date updated_at
        +render()
        +save()
    }
    class Renderer {
        +renderMarkdown()
        +renderDiagram()
    }
    WikiPage --> Renderer
```

### 4. Gantt Chart

```mermaid
gantt
    title Project Timeline
    dateFormat  YYYY-MM-DD
    section Design
    Research           :a1, 2024-01-01, 30d
    Design             :a2, after a1, 20d
    section Development
    Implementation     :a3, after a2, 45d
    Testing            :a4, after a3, 15d
```

### 5. Pie Chart

```mermaid
pie title Project Time Distribution
    "Research" : 25
    "Design" : 20
    "Development" : 40
    "Testing" : 15
```

## PlantUML Diagrams

### 1. Simple Class Diagram

```plantuml
@startuml
class User {
  +String name
  +String email
  +login()
  +logout()
}

class WikiPage {
  +String title
  +String content
  +save()
  +delete()
}

User "1" -- "*" WikiPage : creates
@enduml
```

### 2. Sequence Diagram

```plantuml
@startuml
actor User
participant "Wiki App" as Wiki
participant "Markdown Renderer" as Renderer
database "Storage" as DB

User -> Wiki: Create new page
Wiki -> Renderer: Parse markdown
Renderer -> Renderer: Detect diagram blocks
Renderer -> Wiki: Return HTML with diagrams
Wiki -> DB: Save page
DB --> Wiki: Confirm saved
Wiki --> User: Display rendered page
@enduml
```

### 3. Activity Diagram

```plantuml
@startuml
start
:User opens wiki;
if (Page exists?) then (yes)
  :Load page;
  :Render markdown;
  :Display diagrams;
else (no)
  :Show empty editor;
endif
:User can edit;
:Save changes;
stop
@enduml
```

### 4. Component Diagram

```plantuml
@startuml
package "Wiki Application" {
  [Wiki Editor]
  [Wiki Preview]
  [Markdown Parser]
  [Diagram Renderer]
}

package "External Services" {
  [PlantUML Server]
  [Mermaid.js]
}

[Wiki Editor] --> [Markdown Parser]
[Markdown Parser] --> [Diagram Renderer]
[Diagram Renderer] --> [PlantUML Server]
[Diagram Renderer] --> [Mermaid.js]
[Diagram Renderer] --> [Wiki Preview]
@enduml
```

### 5. State Diagram

```plantuml
@startuml
[*] --> Draft
Draft --> UnderReview : submit
UnderReview --> Published : approve
UnderReview --> Draft : reject
Published --> Archived : archive
Archived --> [*]
@enduml
```

## Testing Instructions

To test these diagrams:

1. Create a new wiki page in the application
2. Copy any of the diagram examples above
3. Paste into the wiki editor
4. The preview pane should automatically render the diagrams
5. Verify that both PlantUML and Mermaid diagrams render correctly

## Supported Diagram Types

### Mermaid (Client-side rendering)
- Flowcharts
- Sequence diagrams
- Class diagrams
- State diagrams
- Entity Relationship diagrams
- Gantt charts
- Pie charts
- Git graphs
- And more...

### PlantUML (Server-side rendering via plantuml.com)
- Class diagrams
- Sequence diagrams
- Use case diagrams
- Activity diagrams
- Component diagrams
- State diagrams
- Object diagrams
- Deployment diagrams
- Timing diagrams
- And more...

## Notes

- PlantUML diagrams are rendered using the public PlantUML server at www.plantuml.com
- Mermaid diagrams are rendered client-side using Mermaid.js
- Both diagram types support dark/light mode themes
- Diagrams are responsive and will scale to fit the preview pane
