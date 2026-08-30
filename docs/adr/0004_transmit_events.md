# ADR: Spacecraft Event Management

## Status

Accepted

## Context

The spacecraft simulator needs to report not only continuous telemetry values, but also discrete events such as:

* battery low detected;
* battery critical detected;
* high temperature detected;
* anomaly cleared;
* spacecraft mode changed from Nominal to Degraded or Safe.

Events must remain available for retransmission until the ground segment confirms that they were received.

A distinction is required between:

* health state: the current condition of a component;
* component event: a transition or occurrence detected by a component;
* managed event: an event enriched with metadata and tracked for reliable transmission;
* operator acknowledgement: a ground-side concern, separate from spacecraft delivery confirmation.

## Decision

### Component health and events

Components own their physical and health state.

For example, the `PowerSystem` maintains health state such as:

* battery health: `Nominal`, `Low`, or `Critical`;
* battery overheating state.

Components evaluate transitions in their health state and produce lightweight `ComponentEvent` values such as:

* `BatteryLowDetected`;
* `BatteryLowCleared`;
* `BatteryCriticalDetected`;
* `BatteryCriticalCleared`;
* `HighTemperatureDetected`;
* `HighTemperatureCleared`.

A component does not create event IDs, track retries, or manage transmission.

### Spacecraft event collection

The `Spacecraft` collects component events and forwards them to an `EventsManager`.

The spacecraft is also responsible for generating spacecraft-level events, such as mode changes resulting from autonomous rules.

For example:

`BatteryCritical` → autonomous rule → `Mode::Safe` → `ModeChangeEvent`

### EventsManager

The spacecraft owns a single `EventsManager`.

The `EventsManager` is responsible for:

* assigning sequential event IDs;
* adding timestamps;
* storing events awaiting delivery confirmation;
* tracking first transmission time;
* tracking last transmission time;
* tracking retry count;
* deciding which events are eligible for transmission;
* removing events once ground delivery is confirmed.

Pending transmission state is kept separately from the immutable event itself.

An event remains in the pending queue until ground confirms its delivery.

### Telemetry transmission

`SpacecraftTelemetry` contains:

* simulation identifier;
* spacecraft identifier;
* current spacecraft mode;
* component telemetry;
* events currently eligible for transmission;
* telemetry timestamp.

Events are cloned into outgoing telemetry packets rather than removed from the `EventsManager`.

Therefore an unconfirmed event may be included in multiple telemetry packets.

### Transmission and acknowledgement

After a telemetry packet is successfully transmitted, the spacecraft records the IDs of events contained in that transmission.

The ground segment later returns delivery confirmations containing event IDs.

Delivery confirmation means:

> The ground system received this event and the spacecraft may stop retransmitting it.

This is intentionally separate from operator acknowledgement.

An operator acknowledgement means:

> A human operator has seen or acknowledged an alert.

Operator acknowledgement is primarily a ground-segment concern and does not control spacecraft event retransmission.

## Consequences

### Positive

* Components remain independent from communication and retry logic.
* Event delivery is reliable across intermittent or delayed communication.
* Health state and historical events remain conceptually separate.
* Duplicate retransmissions can be identified using event IDs.
* Component-level and spacecraft-level events use the same transmission mechanism.
* The architecture can later support configurable retry intervals and prioritization.
* Ground-generated alerts can remain separate from spacecraft-generated events.

### Negative

* Events must be cloned when included in telemetry packets.
* The spacecraft must maintain pending event state until delivery is confirmed.
* Ground and spacecraft require an acknowledgement protocol based on event IDs.
* Additional state is required to prevent repeated component events while an anomaly remains active.

## Future Extensions

The initial implementation retransmits every pending event.

Later versions may add:

* retry intervals based on `last_sent_at`;
* maximum retry counts;
* severity-based prioritization;
* persistent event storage;
* event sequence numbers;
* deduplication on the ground;
* event expiration policies;
* separate event messages outside periodic telemetry packets;
* ground-side alert lifecycle management;
* event correlation and autonomous FDIR reasoning.
