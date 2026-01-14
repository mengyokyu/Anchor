# Anchor SDK — Product Requirements Document

> **"The memory infrastructure for AI — local, encrypted, relationship-based."**

---

## Table of Contents

1. [Vision](#vision)
2. [Problem Statement](#problem-statement)
3. [Solution Overview](#solution-overview)
4. [Core Concepts](#core-concepts)
5. [Architecture](#architecture)
6. [Features](#features)
7. [Technical Specification](#technical-specification)
8. [Phased Roadmap](#phased-roadmap)
9. [Target Users & Domains](#target-users--domains)
10. [Competitive Landscape](#competitive-landscape)
11. [Business Model](#business-model)
12. [Success Metrics](#success-metrics)
13. [Open Questions](#open-questions)
14. [Non-Goals](#non-goals-what-anchor-is-not)
15. [Encryption Flow](#encryption-flow-detailed)
16. [User Experience Examples](#user-experience-examples)
17. [Key Differentiators](#key-differentiators-summary)
18. [Glossary](#appendix-glossary)

---

## Vision

Anchor is **memory infrastructure for AI applications**.

It's not an AI assistant. It's not a product end-users interact with directly. It's the **memory layer** that developers integrate into their AI applications — enabling those AIs to remember, learn, and reason across sessions with 100% deterministic, auditable, local-first storage.

### The One-Liner

> **"Obsidian for your AI."**

Where Obsidian gives humans a second brain with local markdown and linked notes, Anchor gives AI applications the same — a local, human-readable, relationship-based memory they can read, write, and update.

### The Tagline

> **"Stop searching for context. Give your AI a map."**

---

## Problem Statement

### The "Vector Soup" Problem

Current AI memory solutions rely on **vector embeddings** and **semantic similarity search**. This creates fundamental issues:

| Problem | Description |
|---------|-------------|
| **Unreliable retrieval** | Embeddings "guess" what's relevant based on similarity, missing structural dependencies and causal relationships |
| **No relationship understanding** | "This function exists BECAUSE of that decision" is impossible to express in vector space |
| **Black box** | Users can't see, understand, or edit what the AI "remembers" |
| **Cloud dependency** | Most solutions require cloud storage, creating privacy, latency, and cost issues |
| **Context walls** | AI tools work in silos — Cursor can't see your meeting notes, Claude can't see your other repos |
| **Non-deterministic** | Same query can return different results, making AI behavior unpredictable |

### What Developers Need

Developers building AI applications need:

1. **Persistent memory** — AI that remembers across sessions
2. **Relationship awareness** — Understanding how things connect, not just what's similar
3. **Local-first** — Data stays on user's machine, no cloud required
4. **Transparency** — Users can see and edit what the AI knows
5. **Determinism** — Same query = same result, always
6. **Privacy** — End-to-end encryption, data never exposed
7. **Easy integration** — SDK + MCP that works with existing AI tools

---

## Solution Overview

Anchor is an **SDK + MCP server** that provides **Deterministic Structural Memory** for AI applications.

### Core Innovation: Structural Memory vs Vector Memory

| Aspect | Vector Memory (Current) | Structural Memory (Anchor) |
|--------|------------------------|---------------------------|
| **Storage** | Embeddings in vector DB | Markdown "Blueprints" |
| **Retrieval** | Similarity search ("what's like this?") | Graph traversal ("what's connected to this?") |
| **Relationships** | Implicit (inferred from similarity) | Explicit (typed, directional links) |
| **Human-readable** | No | Yes |
| **Editable** | No | Yes |
| **Deterministic** | No | Yes |
| **Local-first** | Rarely | Always |

### How It Works

```
┌─────────────────────────────────────────────────────────────┐
│                        AI APPLICATION                        │
│                  (Healthcare AI, Coding Tool, etc.)          │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ SDK calls / MCP protocol
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         ANCHOR SDK                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Reader    │  │   Writer    │  │   Relationship      │  │
│  │   Engine    │  │   Engine    │  │   Engine            │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Auto-Decay │  │ Auto-Update │  │   Query Engine      │  │
│  │   Manager   │  │   Manager   │  │   (Graph Traversal) │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              Encryption Layer (E2E)                     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Encrypted read/write
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    LOCAL FILE SYSTEM                         │
│                                                              │
│   📁 .anchor/                                                │
│   ├── 📄 index.md          (Master index of all memories)   │
│   ├── 📄 relationships.md  (Graph of all connections)       │
│   ├── 📁 blueprints/       (Individual memory units)        │
│   │   ├── 📄 memory_001.md                                  │
│   │   ├── 📄 memory_002.md                                  │
│   │   └── ...                                               │
│   └── 📄 .anchor.lock      (Encryption metadata)            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Concepts

### 1. Blueprints

A **Blueprint** is a single unit of memory stored as a markdown file. It contains:

- **Content**: The actual information being remembered
- **Metadata**: Timestamps, source, confidence, decay status
- **Relationships**: Typed links to other blueprints

```markdown
---
id: bp_2026_01_14_001
created: 2026-01-14T10:30:00Z
updated: 2026-01-14T14:22:00Z
source: user_input
confidence: 0.95
decay_status: active
tags: [diagnosis, patient_123, cardiology]
---

# Patient 123 — Initial Diagnosis

Patient presented with chest pain radiating to left arm. ECG showed ST elevation.

## Summary
Diagnosed with acute myocardial infarction (STEMI).

## Relationships
- **caused_by**: [[bp_2026_01_10_042]] (Previous hypertension history)
- **led_to**: [[bp_2026_01_14_002]] (Treatment plan: PCI)
- **contradicts**: [[bp_2025_08_20_011]] (Previous "low cardiac risk" assessment)
- **related_to**: [[bp_2026_01_14_003]] (Family history of heart disease)
```

### 2. Relationships

Relationships are **typed, directional links** between blueprints. Unlike vector similarity, relationships are explicit and meaningful.

#### Relationship Types

| Type | Direction | Example |
|------|-----------|---------|
| `caused_by` | A ← B | "This diagnosis was caused by this symptom" |
| `led_to` | A → B | "This decision led to this implementation" |
| `depends_on` | A → B | "This function depends on this module" |
| `contradicts` | A ↔ B | "This finding contradicts that previous finding" |
| `updates` | A → B | "This memory updates/supersedes that one" |
| `related_to` | A ↔ B | "These are related but no causal link" |
| `part_of` | A → B | "This is a component of that whole" |
| `references` | A → B | "This mentions/cites that" |

### 3. Memory Lifecycle

Memories in Anchor have a lifecycle managed automatically by the AI:

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  CREATE  │ ──▶ │  ACTIVE  │ ──▶ │ DECAYING │ ──▶ │ ARCHIVED │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                      │                                  │
                      │ (reinforced)                     │ (recalled)
                      ◀──────────────────────────────────┘
```

| State | Description |
|-------|-------------|
| **Active** | Recently created or accessed, high relevance |
| **Decaying** | Not accessed recently, relevance decreasing |
| **Archived** | Low relevance but preserved, can be recalled |
| **Deleted** | Permanently removed (explicit action only) |

### 4. Auto-Decay

The AI automatically manages memory relevance:

- **Time-based decay**: Memories accessed less frequently decay over time
- **Relationship-based persistence**: Memories with many active relationships decay slower
- **Reinforcement**: Accessing a memory resets its decay timer
- **Archival, not deletion**: Decayed memories are archived, not lost

### 5. Auto-Update

The AI can update memories when new information arrives:

- **Contradiction detection**: New info contradicts old → flag relationship
- **Supersession**: New memory explicitly replaces old one
- **Augmentation**: New info adds to existing memory
- **Confidence adjustment**: New evidence changes confidence scores

---

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                          ANCHOR SDK                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                     PUBLIC API LAYER                      │   │
│  │  • create_memory()      • query_related()                 │   │
│  │  • update_memory()      • get_context_for()               │   │
│  │  • link_memories()      • traverse_from()                 │   │
│  │  • decay_memory()       • export_subgraph()               │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                     MCP SERVER LAYER                      │   │
│  │  Tools:                  Resources:                       │   │
│  │  • anchor_remember       • anchor://blueprints            │   │
│  │  • anchor_recall         • anchor://relationships         │   │
│  │  • anchor_connect        • anchor://graph                 │   │
│  │  • anchor_forget                                          │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                      CORE ENGINES                         │   │
│  │                                                           │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │   │
│  │  │   Parser    │  │   Graph     │  │   Query     │       │   │
│  │  │   Engine    │  │   Engine    │  │   Engine    │       │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘       │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │   │
│  │  │   Decay     │  │   Update    │  │   Index     │       │   │
│  │  │   Manager   │  │   Manager   │  │   Manager   │       │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘       │   │
│  │                                                           │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                   ENCRYPTION LAYER                        │   │
│  │  • At-rest encryption (AES-256-GCM)                       │   │
│  │  • In-memory decryption only during operations            │   │
│  │  • Key derivation (Argon2)                                │   │
│  │  • Zero-knowledge design                                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    STORAGE LAYER                          │   │
│  │  • Local filesystem (markdown files)                      │   │
│  │  • Atomic writes (crash safety)                           │   │
│  │  • File watching (external edits)                         │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

#### Writing a Memory

```
1. AI calls create_memory(content, relationships)
2. Parser Engine extracts structure from content
3. Graph Engine validates and adds relationships
4. Index Manager updates master index
5. Encryption Layer encrypts the blueprint
6. Storage Layer writes to .anchor/blueprints/
7. Return memory_id to AI
```

#### Reading/Querying Memory

```
1. AI calls query_related(context) or traverse_from(memory_id)
2. Encryption Layer decrypts relevant blueprints into RAM
3. Query Engine traverses relationship graph
4. Decay Manager updates access timestamps
5. Results compiled and returned
6. Decrypted data cleared from RAM
7. Re-encrypt if any updates occurred
```

---

## Features

### Phase 1 Features (MVP)

#### Core Memory Operations

| Feature | Description |
|---------|-------------|
| **Create memory** | Store new information as a blueprint |
| **Read memory** | Retrieve a specific blueprint by ID |
| **Update memory** | Modify existing blueprint content |
| **Delete memory** | Archive or permanently remove a blueprint |
| **List memories** | Get all blueprints, with filtering options |

#### Relationship Management

| Feature | Description |
|---------|-------------|
| **Link memories** | Create typed relationship between two blueprints |
| **Unlink memories** | Remove a relationship |
| **Get relationships** | List all relationships for a blueprint |
| **Traverse graph** | Follow relationship chain from a starting point |

#### Query Operations

| Feature | Description |
|---------|-------------|
| **Query by content** | Find blueprints containing specific text |
| **Query by tag** | Find blueprints with specific tags |
| **Query by relationship** | Find all blueprints connected to X |
| **Context retrieval** | Get relevant context for a given input |

#### Lifecycle Management

| Feature | Description |
|---------|-------------|
| **Auto-decay** | Automatic relevance decay over time |
| **Manual decay** | Force a blueprint to archived state |
| **Reinforce** | Reset decay timer on access |
| **Bulk archive** | Archive old/unused memories |

#### Security

| Feature | Description |
|---------|-------------|
| **E2E encryption** | All data encrypted at rest |
| **Memory-only decryption** | Data only decrypted in RAM during use |
| **Key management** | Secure key derivation and storage |

#### Integration

| Feature | Description |
|---------|-------------|
| **MCP server** | Full MCP protocol support |
| **SDK (Rust)** | Native Rust library |
| **SDK (Python)** | Python bindings |
| **CLI** | Command-line interface for testing/debugging |

### Phase 2 Features (Future)

| Feature | Description |
|---------|-------------|
| **Cross-domain memory** | Connect memories across different domains |
| **Multi-agent support** | Multiple AIs sharing memory with permissions |
| **Sync (optional)** | Encrypted sync across devices |
| **Schema definitions** | Custom blueprint schemas per domain |
| **Plugins** | Extensible parser/relationship types |
| **Visualization** | Graph visualization tools |

---

## Technical Specification

### Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Core SDK** | Rust | Performance, memory safety, cross-platform |
| **Python bindings** | PyO3 | Python ecosystem access |
| **MCP server** | Rust (or TypeScript) | MCP protocol compatibility |
| **Encryption** | AES-256-GCM + Argon2 | Industry standard, auditable |
| **Storage** | Local filesystem (Markdown) | Human-readable, no dependencies |
| **Index** | In-memory graph + file-based persistence | Fast queries, durability |

### File Structure

```
.anchor/
├── anchor.toml              # Configuration file
├── index.json               # Master index (encrypted)
├── relationships.json       # Relationship graph (encrypted)
├── blueprints/              # Individual memories
│   ├── 2026/
│   │   └── 01/
│   │       ├── bp_001.md.enc
│   │       ├── bp_002.md.enc
│   │       └── ...
│   └── ...
├── archive/                 # Decayed memories
│   └── ...
└── .keys/                   # Key material (encrypted)
    └── master.key.enc
```

### Blueprint Schema

```markdown
---
# Required fields
id: string                   # Unique identifier
created: ISO8601             # Creation timestamp
updated: ISO8601             # Last update timestamp

# Content metadata
source: string               # Origin (user_input, ai_generated, imported)
confidence: float            # 0.0 - 1.0 confidence score
tags: [string]               # Categorization tags

# Lifecycle
decay_status: enum           # active | decaying | archived
last_accessed: ISO8601       # Last access timestamp
access_count: integer        # Total access count
decay_rate: float            # Custom decay rate (optional)

# Relationships (in frontmatter for indexing)
relationships:
  caused_by: [string]        # List of blueprint IDs
  led_to: [string]
  depends_on: [string]
  contradicts: [string]
  updates: [string]
  related_to: [string]
  part_of: [string]
  references: [string]
---

# Title

Content goes here in standard markdown.

## Sections

Can include any markdown formatting.

## Inline Relationships

Can also reference [[other_blueprint_id]] inline.
```

### API Design

#### Rust SDK

```rust
use anchor_sdk::{Anchor, Blueprint, Relationship, QueryResult};

// Initialize
let anchor = Anchor::new(".anchor")
    .with_encryption("user_password")
    .build()?;

// Create memory
let memory = anchor.create(Blueprint {
    content: "Patient diagnosed with STEMI".into(),
    tags: vec!["diagnosis", "cardiology"],
    source: Source::UserInput,
    ..Default::default()
})?;

// Link memories
anchor.link(
    &memory.id,
    &previous_memory.id,
    Relationship::CausedBy,
)?;

// Query related memories
let context = anchor.query()
    .related_to(&memory.id)
    .with_depth(3)
    .execute()?;

// Traverse relationship graph
let chain = anchor.traverse(&memory.id)
    .follow(Relationship::CausedBy)
    .until(|bp| bp.tags.contains("root_cause"))
    .collect()?;
```

#### Python SDK

```python
from anchor import Anchor, Blueprint, Relationship

# Initialize
anchor = Anchor(".anchor", password="user_password")

# Create memory
memory = anchor.create(
    content="Patient diagnosed with STEMI",
    tags=["diagnosis", "cardiology"],
    source="user_input"
)

# Link memories
anchor.link(
    memory.id,
    previous_memory.id,
    relationship=Relationship.CAUSED_BY
)

# Query related memories
context = anchor.query(
    related_to=memory.id,
    depth=3
)

# Traverse relationship graph
chain = anchor.traverse(
    start=memory.id,
    follow=Relationship.CAUSED_BY,
    until=lambda bp: "root_cause" in bp.tags
)
```

#### MCP Tools

```json
{
  "tools": [
    {
      "name": "anchor_remember",
      "description": "Store a new memory",
      "inputSchema": {
        "type": "object",
        "properties": {
          "content": { "type": "string" },
          "tags": { "type": "array", "items": { "type": "string" } },
          "relationships": { "type": "object" }
        },
        "required": ["content"]
      }
    },
    {
      "name": "anchor_recall",
      "description": "Retrieve memories related to a context",
      "inputSchema": {
        "type": "object",
        "properties": {
          "context": { "type": "string" },
          "depth": { "type": "integer", "default": 2 }
        },
        "required": ["context"]
      }
    },
    {
      "name": "anchor_connect",
      "description": "Create a relationship between memories",
      "inputSchema": {
        "type": "object",
        "properties": {
          "from_id": { "type": "string" },
          "to_id": { "type": "string" },
          "relationship": { "type": "string" }
        },
        "required": ["from_id", "to_id", "relationship"]
      }
    },
    {
      "name": "anchor_forget",
      "description": "Archive or delete a memory",
      "inputSchema": {
        "type": "object",
        "properties": {
          "memory_id": { "type": "string" },
          "permanent": { "type": "boolean", "default": false }
        },
        "required": ["memory_id"]
      }
    }
  ]
}
```

---

## Phased Roadmap

### Phase 1: Foundation (MVP)

**Goal:** Working SDK + MCP with core memory operations

| Milestone | Deliverables | Duration |
|-----------|--------------|----------|
| **1.1 Core Storage** | Blueprint schema, file operations, encryption | 2 weeks |
| **1.2 Relationship Engine** | Graph structure, linking, basic traversal | 2 weeks |
| **1.3 Query Engine** | Content search, tag search, relationship queries | 2 weeks |
| **1.4 Lifecycle Management** | Auto-decay, archival, reinforcement | 1 week |
| **1.5 Rust SDK** | Public API, documentation | 1 week |
| **1.6 Python Bindings** | PyO3 bindings, Python API | 1 week |
| **1.7 MCP Server** | Full MCP protocol implementation | 2 weeks |
| **1.8 CLI** | Command-line interface | 1 week |
| **1.9 Testing & Docs** | Test suite, documentation, examples | 2 weeks |

**Total Phase 1:** ~14 weeks

### Phase 2: Enhancement

**Goal:** Production hardening, advanced features

| Milestone | Deliverables |
|-----------|--------------|
| **2.1 Performance** | Optimized indexing, caching, large memory support |
| **2.2 Advanced Queries** | Complex graph queries, pattern matching |
| **2.3 Schema System** | Custom blueprint schemas per domain |
| **2.4 Plugin System** | Extensible parsers, relationship types |
| **2.5 Visualization** | Graph visualization, debugging tools |

### Phase 3: Cross-Domain

**Goal:** Enable cross-domain memory for advanced AI systems

| Milestone | Deliverables |
|-----------|--------------|
| **3.1 Multi-Domain** | Domain separation, cross-domain linking |
| **3.2 Multi-Agent** | Permission system, shared memory |
| **3.3 Sync (Optional)** | Encrypted device sync |

---

## Target Users & Domains

### Primary Users

**Developers building AI applications** who need:
- Persistent memory for their AI
- Local-first data storage
- Transparent, auditable AI memory
- Privacy-compliant solutions

### Domain Applications

| Domain | Use Case | Value Proposition |
|--------|----------|-------------------|
| **Healthcare** | Patient history, treatment chains, longitudinal care | Traceable reasoning, HIPAA-friendly local storage |
| **Legal** | Case relationships, precedent chains, contract analysis | Audit trails, explainable AI decisions |
| **Software Engineering** | Code relationships, decision history, cross-repo context | "Why was this built this way?" |
| **Research** | Paper relationships, experiment history, literature review | Citation chains, finding connections |
| **Personal Knowledge** | Notes, ideas, learning paths | "What did I think about X last year?" |
| **Enterprise** | Institutional memory, decision tracking | "Why did we decide this?" |

---

## Competitive Landscape

### Direct Competitors

| Product | Local? | Vector? | Human-Readable? | MCP? | Relationship-Based? |
|---------|--------|---------|-----------------|------|---------------------|
| **Mem0** | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Zep** | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Memvid** | ✅ | ✅ | ❌ | ✅ | ❌ |
| **Memphora** | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Anchor** | ✅ | ❌ | ✅ | ✅ | ✅ |

### Adjacent Products

| Product | Relationship to Anchor |
|---------|----------------------|
| **Obsidian** | Similar philosophy (local markdown, links), but for humans, not AI |
| **GraphRAG** | Uses knowledge graphs, but requires infrastructure, not local-first |
| **LangChain Memory** | Memory abstractions, but vector-based, not relationship-based |

### Anchor's Unique Position

> **"Obsidian's philosophy + Mem0's purpose + MCP integration"**

No existing product combines:
- ✅ Local-first markdown storage
- ✅ Explicit relationship graphs (not vectors)
- ✅ Human AND AI readable/writable
- ✅ MCP-native integration
- ✅ E2E encryption
- ✅ Domain-agnostic SDK

---

## Business Model

### Open-Core Model

| Tier | Price | Features |
|------|-------|----------|
| **Open Source** | Free | Full SDK, MCP server, CLI, local storage, encryption |
| **Pro** | $X/mo | Priority support, advanced analytics, team features |
| **Enterprise** | Custom | On-prem support, SLA, custom integrations, compliance |

### Revenue Streams

1. **Enterprise licenses** — On-prem deployments for regulated industries
2. **Support contracts** — Priority support for businesses
3. **Managed sync (optional)** — Encrypted sync service
4. **Consulting** — Integration services for complex deployments

### Why Open Source Core?

- Adoption & community building
- Trust through transparency (critical for memory/privacy)
- Contributions from users
- Enterprise upsell path

---

## Success Metrics

### Phase 1 Success Criteria

| Metric | Target |
|--------|--------|
| **Working MVP** | SDK + MCP with all Phase 1 features |
| **Performance** | <10ms query latency for 10K memories |
| **Test coverage** | >80% code coverage |
| **Documentation** | Complete API docs + getting started guide |
| **Early adopters** | 10+ developers testing |

### Long-Term Metrics

| Metric | Target |
|--------|--------|
| **GitHub stars** | 1K+ in first year |
| **Downloads** | 10K+ SDK downloads |
| **Integrations** | 5+ published integrations |
| **Enterprise customers** | 3+ paying customers |

---

## Open Questions

### Technical

1. **Index structure**: File-based JSON vs SQLite for relationship index?
2. **Encryption granularity**: Per-file vs per-vault encryption?
3. **Sync protocol**: If we add sync, which protocol?
4. **Memory limits**: Max size per blueprint? Max total memories before performance degrades?
5. **Conflict resolution**: If human edits a blueprint while AI is updating, who wins?
6. **Relationship inference**: Should AI auto-suggest relationships, or only explicit links?

### Product

1. **Default domain schemas**: Should we ship pre-built schemas for common domains (healthcare, legal, etc.)?
2. **Migration tools**: How do users import from Obsidian, Notion, or other tools?
3. **Visualization priority**: Is graph visualization MVP or Phase 2?
4. **Offline-first vs local-first**: Same thing, or different guarantees?

### Business

1. **Pricing model**: Per-seat? Per-memory? Flat rate?
2. **Open source license**: Apache 2.0? MIT? AGPL?
3. **Enterprise requirements**: What compliance certifications matter most? (SOC2, HIPAA, etc.)

---

## Non-Goals (What Anchor is NOT)

Clarity on scope — these are explicitly **out of scope**:

| Non-Goal | Why |
|----------|-----|
| **End-user product** | Anchor is infrastructure, not a consumer app. Developers build products ON Anchor. |
| **AI assistant** | Anchor is memory, not intelligence. It doesn't chat, reason, or generate — it remembers. |
| **Vector search** | We're deliberately NOT doing embedding similarity. That's the whole point. |
| **Cloud service (Phase 1)** | Local-first is the core value prop. Cloud sync is optional, later. |
| **General database** | Anchor is optimized for AI memory patterns, not arbitrary data storage. |
| **Real-time collaboration** | Single-user/single-agent first. Multi-agent is Phase 2+. |

---

## Encryption Flow (Detailed)

The E2E encryption follows a strict "decrypt only in RAM" model:

```
┌─────────────────────────────────────────────────────────────────┐
│                        AT REST (Disk)                            │
│                                                                  │
│   All blueprints stored as .md.enc (encrypted)                  │
│   Index files encrypted                                          │
│   Keys derived from user password via Argon2                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ AI requests memory
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DECRYPTION (RAM Only)                       │
│                                                                  │
│   1. Load encrypted blueprint from disk                          │
│   2. Decrypt into RAM using derived key                          │
│   3. Process request (read/query/traverse)                       │
│   4. If updates needed → modify in RAM                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Operation complete
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      RE-ENCRYPTION (Disk)                        │
│                                                                  │
│   1. Re-encrypt modified data                                    │
│   2. Atomic write to disk (crash-safe)                          │
│   3. Clear plaintext from RAM                                    │
│   4. Securely zero memory                                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Security Guarantees

| Guarantee | Implementation |
|-----------|----------------|
| **Data at rest** | Always encrypted (AES-256-GCM) |
| **Data in transit** | N/A — local only, no network |
| **Data in memory** | Plaintext only during active operations |
| **Key derivation** | Argon2id with high memory cost |
| **Key storage** | Derived from password, never stored plaintext |
| **Memory clearing** | Explicit zeroing after use |

---

## User Experience Examples

### What the AI Sees

When an AI application uses Anchor, this is the experience:

#### Example 1: Healthcare AI Recalling Patient Context

```
AI receives: "Patient 123 is back with chest pain"

AI calls: anchor_recall(context="Patient 123 chest pain")

Anchor returns:
{
  "memories": [
    {
      "id": "bp_2026_01_14_001",
      "summary": "Patient 123 — Initial Diagnosis: STEMI",
      "relationships": {
        "caused_by": ["bp_2026_01_10_042 (Hypertension history)"],
        "led_to": ["bp_2026_01_14_002 (PCI treatment)"],
        "contradicts": ["bp_2025_08_20_011 (Previous low risk assessment)"]
      },
      "relevance_chain": "chest pain → cardiac → previous STEMI diagnosis"
    }
  ],
  "suggested_context": [
    "Patient has history of STEMI (Jan 2026)",
    "Previous 'low cardiac risk' assessment was wrong",
    "Successfully treated with PCI",
    "Underlying cause: chronic hypertension"
  ]
}

AI can now respond with full context, tracing WHY it knows what it knows.
```

#### Example 2: Coding AI Understanding a Function

```
AI receives: "Why does auth.py use Redis instead of sessions?"

AI calls: anchor_traverse(
  start="bp_auth_module",
  follow=["caused_by", "references"],
  depth=3
)

Anchor returns:
{
  "chain": [
    {
      "id": "bp_auth_module",
      "content": "Auth module uses Redis for session storage"
    },
    {
      "id": "bp_2025_09_meeting_034",
      "relationship": "caused_by",
      "content": "Team decided Redis for horizontal scaling (Sept 15 meeting)"
    },
    {
      "id": "bp_2025_09_perf_analysis",
      "relationship": "caused_by", 
      "content": "Performance analysis showed session DB bottleneck"
    }
  ]
}

AI: "auth.py uses Redis because in September, the team found a session 
     database bottleneck. In the Sept 15 meeting, they decided Redis 
     would allow horizontal scaling. Here's the meeting note: ..."
```

#### Example 3: AI Updating Its Own Memory

```
User says: "Actually, Patient 123's diagnosis was wrong. It was actually anxiety."

AI calls: anchor_remember(
  content="Patient 123 diagnosis corrected: anxiety-induced chest pain, not STEMI",
  relationships={
    "updates": "bp_2026_01_14_001",
    "contradicts": "bp_2026_01_14_001"
  },
  tags=["diagnosis", "correction", "patient_123"]
)

AI calls: anchor.update(
  id="bp_2026_01_14_001",
  decay_status="archived",
  note="Superseded by bp_2026_01_15_001 — misdiagnosis corrected"
)

Memory is now updated. Next time AI recalls Patient 123, 
it will see the correction and the history of the misdiagnosis.
```

---

## Key Differentiators Summary

### Why Anchor Wins

| vs Competition | Anchor's Advantage |
|----------------|-------------------|
| **vs Mem0/Zep** | Local-first, human-readable, no cloud dependency |
| **vs Memvid** | Relationship-based not vector-based, human-editable markdown |
| **vs Obsidian** | AI-native, MCP integration, designed for agents not humans |
| **vs GraphRAG** | No infrastructure required, local files, encrypted |
| **vs LangChain Memory** | Deterministic, persistent, relationship-aware |

### The Core Bet

Anchor bets that **relationships > similarity**.

- Vector search asks: "What's similar to this?"
- Anchor asks: "What's connected to this, and how?"

For any domain where **causality, history, and reasoning chains matter**, relationship-based memory will outperform similarity-based memory.

### The Tagline Variations

- **Technical:** "Deterministic structural memory for AI applications"
- **Developer:** "Give your AI a memory it can actually understand"
- **Simple:** "Obsidian for your AI"
- **Provocative:** "Stop searching for context. Give your AI a map."

---

## Appendix: Glossary

| Term | Definition |
|------|------------|
| **Blueprint** | A single unit of memory stored as a markdown file |
| **Relationship** | A typed, directional link between two blueprints |
| **Decay** | The process by which unused memories lose relevance over time |
| **Reinforcement** | Accessing a memory resets its decay timer |
| **Traverse** | Following relationship links through the memory graph |
| **MCP** | Model Context Protocol — standard for AI tool integration |
| **Structural Memory** | Memory organized by explicit relationships, not vector similarity |
| **Blueprint Schema** | The frontmatter + content structure of a memory file |

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 0.1 | 2026-01-14 | Initial PRD draft |

---

*End of Document*