

# Anchor

**Agent Server Protocol (ASP)**

---

## What is Anchor?

**Anchor is an Agent Server Protocol (ASP): a local background daemon that provides AI coding agents with deterministic context and safe write capabilities.**

AI agents like Claude Code and Cursor are very good at *thinking* about code, but they are still bad at:

* knowing the real structure of a codebase
* navigating files safely
* applying multi-file changes without breaking things

Anchor exists to solve exactly that.

---

## The Core Idea

Anchor separates responsibilities cleanly:

### Agent = Brain

* Understands intent
* Reasons about logic
* Decides *what* should change
* Generates domain-specific code

### Anchor = Body

* Maintains a structural map of the codebase
* Knows where symbols live and how files are connected
* Applies edits safely on disk
* Prevents broken builds and corrupted files

**Agents think. Anchor acts.**

---

## Why “Agent Server Protocol” (ASP)?

Just like **Language Server Protocol (LSP)** standardized how editors talk to language engines, **ASP standardizes how AI agents talk to a codebase-aware execution engine**.

Anchor is:

* long-running
* stateful
* authoritative
* queried by agents

Agents come and go.
Anchor stays.

---

## The Problems Anchor Solves

AI coding agents today suffer from systemic issues:

### 1. Hallucinated Structure

Agents guess file locations, dependencies, and ownership because they don’t have a real map.

### 2. Context Rot

Understanding of the codebase decays across long or multi-step tasks.

### 3. Probabilistic Semantics

Semantic search answers “this looks related”, but codebases require “this **is** related”.

### 4. Unsafe Writes

Text-based edits break syntax, formatting, or builds—especially across multiple files.

Anchor addresses these by acting as a **deterministic authority** instead of a guesser.

---

## What Anchor Actually Does

Anchor provides **two core capabilities** to agents:

### 1. Perfect Context (The Oracle)

* Builds and maintains a structural graph of the codebase
* Answers factual questions like:

  * Where is this symbol defined?
  * What references this file?
  * What depends on this module?

No guessing. No grep spam.

---

### 2. Surgical Hands (The Executor)

* Anchor, not the agent, touches the filesystem
* Applies:

  * safe refactors
  * multi-file edits
  * precise code insertions
* Uses structural parsing, not text replacement

Agents never directly edit files.
Anchor guarantees consistency.

---

## How Agents Use Anchor

Anchor is **not an AI**.
It is a **server** that agents query and command.

Typical flow:

1. Agent asks Anchor for context
2. Anchor returns deterministic structural facts
3. Agent decides what should change
4. Agent instructs Anchor to apply changes
5. Anchor executes safely and atomically

---

## Scope (v0 / v1)

Initial versions of Anchor focus on:

* Monorepo only
* Single language at a time
* Static structure only
* Local execution

Cross-repo linking and polyglot support come later.

---

## Why Anchor Matters

Anchor turns AI coding from:

> “Read files, guess structure, hope nothing breaks”

into:

> “Query structure, reason safely, execute precisely”

It doesn’t make agents smarter.
It makes them **grounded**.

---

## One-Line Summary

> **Anchor is an Agent Server Protocol that gives AI coding agents a persistent map of the codebase and safe hands to modify it.**

