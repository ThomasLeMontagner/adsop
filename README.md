# ADSOP (Autonomous Deep Space Operations Platform)

## Mission Statement

ADSOP (Autonomous Deep Space Operations Platform) is an open-source platform for designing, simulating, and operating autonomous space missions. It provides a common operational environment where spacecraft behaviour, mission operations, and onboard autonomy can be developed, validated, and explored before deployment to real missions.

## Vision

Enable anyone, from students and researchers to mission engineers,to prototype, simulate and operate realistic space missions using modern software engineering practices.

## Guiding Principles

**Mission-driven**

The platform models complete mission operations rather than isolated software components.

**Technology agnostic**

Mission concepts are separated from implementation technologies.

**Modular**

Every subsystem should be independently replaceable or extendable.

**Digital Twin**

The platform should maintain a coherent representation of spacecraft state.

**Open**

The platform should encourage experimentation, learning and collaboration.

## Capabilities

The following diagram illustrates the envisioned capabilities of ADSOP:
![Envisioned capabilities of ADSOP](./mbse/exports/[MCB]%20Capabilities.jpg)

## High-level architecture

The ADSOP architecture is designed around a clear separation of responsibilities between the user interface, mission operations, simulation environment, and data persistence. The platform acts as the bridge between mission engineers and a simulated (or eventually real) spacecraft by providing a unified environment for monitoring spacecraft health, managing commands, supporting mission planning, and analysing mission behaviour.

The frontend provides an intuitive web interface through which users interact with one or more mission projects. The backend orchestrates the core mission operations, including authentication, project management, command handling, health monitoring, and communication with the simulation environment. Spacecraft behaviour is reproduced by a set of Rust-based simulators (e.g., spacecraft, communication link and ground station), allowing complete missions to be executed and evaluated in a realistic environment. Finally, a PostgreSQL database stores persistent information such as projects, mission configurations, telemetry, commands, alerts and mission history.

To keep the initial implementation simple, ADSOP starts with a single Go backend service that groups all core platform functionalities. Communication between the frontend and the backend relies on standard HTTPS requests for user interactions and WebSockets for real-time updates such as telemetry and alerts. Likewise, the backend communicates directly with the simulation environment without introducing a message broker or microservice architecture. This approach reduces complexity during the early development phases while leaving room for future evolution towards a more distributed architecture if required.

The simulation environment is intentionally kept independent from the mission operations layer, allowing different simulators to be developed, replaced or extended without impacting the rest of the platform. Although ADSOP initially focuses on mission simulation, the architecture is designed so that simulated components can progressively be replaced by real spacecraft interfaces, enabling the same operational platform to support both simulated and real mission operations.

![high-level architecture](./docs/architecture//High-level%20archtiecture.png)

ADSOP uses a lightweight, container-assisted development environment. Shared services are provided through Docker Compose, while the frontend, backend, and simulators can be run directly for fast development. See 'docs/development/getting-started.md' for setup instructions.
