# ADR-0002: Use a Modular Monolith

## Status

Accepted

## Context

ADSOP provides several capabilities, including project management, command management, health monitoring and mission simulation. These capabilities are strongly coupled during the initial development phase.

## Decision

The backend will initially be implemented as a single modular Go application. Functional areas will be separated into logical modules, while remaining part of the same deployable service.

## Consequences

### Positive

- Reduced architectural complexity.
- Easier debugging and local development.
- Clear module boundaries that can later evolve into independent services if justified.

### Negative

- Independent scaling of individual modules is not possible.
- Future service extraction may require additional refactoring.
