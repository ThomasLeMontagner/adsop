# ADR-0001: Use a Monorepo

## Status

Accepted

## Context

ADSOP consists of several closely related components, including the frontend, backend, simulation environment, documentation and MBSE models. During the early stages of the project, these components will evolve together.

## Decision

ADSOP will be developed as a single monorepository containing all source code, documentation and supporting artefacts.

## Consequences

### Positive

- Single source of truth.
- Easier refactoring across components.
- Simplified versioning and CI/CD.

### Negative

- Repository size will grow over time.
- Independent releases of individual components may require additional tooling in the future.
