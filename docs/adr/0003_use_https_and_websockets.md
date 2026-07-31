# ADR-0003: Use HTTPS and WebSockets

## Status

Accepted

## Context

ADSOP requires both request-response interactions (e.g. project configuration, command submission) and real-time communication (e.g. telemetry, alerts, simulation updates).

## Decision

Communication between the frontend and backend will use HTTPS for standard API requests and WebSockets for real-time bidirectional communication.

## Consequences

### Positive

- Simple and widely supported architecture.
- Suitable for both configuration workflows and live mission monitoring.
- Avoids introducing additional messaging infrastructure during the early development phases.

### Negative

- May need to evolve towards a more scalable event-driven architecture as the platform grows.
