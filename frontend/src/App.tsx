import { useEffect, useState } from "react";
import type { SpacecraftTelemetry } from "./telemetry/types";
import "./App.css";

type Simulation = {
  id: string;
  status: string;
  created_at: string;
};

type ConnectionStatus = "connecting" | "connected" | "disconnected";

const labelize = (value: string) =>
  value
    .replaceAll("_", " ")
    .replace(/\b\w/g, (character) => character.toUpperCase());

function formatTelemetryValue(name: string, value: unknown) {
  if (typeof value === "boolean") {
    return value ? "Enabled" : "Disabled";
  }

  if (typeof value === "number") {
    const formatted = new Intl.NumberFormat("en", {
      maximumFractionDigits: 2,
    }).format(value);

    if (name.includes("temperature")) return `${formatted} °C`;
    if (name.includes("voltage")) return `${formatted} V`;
    return formatted;
  }

  if (typeof value === "object" && value !== null) {
    return JSON.stringify(value);
  }

  return String(value);
}

function App() {
  const [simulation, setSimulation] = useState<Simulation | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [telemetry, setTelemetry] = useState<SpacecraftTelemetry | null>(null);
  const [connectionStatus, setConnectionStatus] =
    useState<ConnectionStatus>("connecting");

  useEffect(() => {
    const socketProtocol = window.location.protocol === "https:" ? "wss" : "ws";
    const socket = new WebSocket(
      `${socketProtocol}://${window.location.hostname}:8080/ws`,
    );

    socket.onopen = () => setConnectionStatus("connected");

    socket.onmessage = (event) => {
      try {
        const latestTelemetry: SpacecraftTelemetry = JSON.parse(event.data);
        setTelemetry(latestTelemetry);
      } catch {
        console.error("Received an invalid telemetry packet");
      }
    };

    socket.onerror = () => setConnectionStatus("disconnected");
    socket.onclose = () => setConnectionStatus("disconnected");

    return () => socket.close();
  }, []);

  async function createSimulation() {
    setIsCreating(true);
    setError(null);

    try {
      const response = await fetch("/api/simulations", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          spacecraft_id: "mars-orbiter-1",
        }),
      });

      if (!response.ok) {
        throw new Error(`Backend returned status ${response.status}`);
      }

      const createdSimulation: Simulation = await response.json();
      setSimulation(createdSimulation);
    } catch (err) {
      const message =
        err instanceof Error ? err.message : "Unknown error occurred";
      setError(message);
    } finally {
      setIsCreating(false);
    }
  }

  const missionStatus = simulation?.status ?? (telemetry ? "Active" : "Not started");
  const spacecraftId = telemetry?.spacecraft_id ?? "mars-orbiter-1";
  const latestPacket = telemetry?.timestamp
    ? new Date(telemetry.timestamp).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      })
    : "Waiting";

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="page-width topbar__content">
          <a className="brand" href="#top" aria-label="ADSOP Mission Control home">
            <span className="brand__mark" aria-hidden="true">
              <span />
            </span>
            <span>
              <strong>ADSOP</strong>
              <small>Mission control</small>
            </span>
          </a>

          <div
            className={`connection connection--${connectionStatus}`}
            role="status"
          >
            <span className="connection__dot" aria-hidden="true" />
            Telemetry {connectionStatus}
          </div>
        </div>
      </header>

      <main id="top" className="page-width dashboard">
        <section className="dashboard__heading" aria-labelledby="page-title">
          <div>
            <p className="eyebrow">Operational overview</p>
            <h1 id="page-title">Mission control</h1>
            <p className="dashboard__intro">
              Monitor spacecraft health, live telemetry, and mission events.
            </p>
          </div>

          <button
            className="primary-button"
            onClick={createSimulation}
            disabled={isCreating}
          >
            {isCreating && <span className="spinner" aria-hidden="true" />}
            {isCreating
              ? "Starting simulation"
              : simulation
                ? "Start new simulation"
                : "Create simulation"}
          </button>
        </section>

        {error && (
          <div className="alert" role="alert">
            <span className="alert__icon" aria-hidden="true">
              !
            </span>
            <div>
              <strong>Simulation could not be created</strong>
              <p>{error}</p>
            </div>
          </div>
        )}

        <section className="status-grid" aria-label="Mission summary">
          <article className="status-card">
            <p>Mission status</p>
            <div className="status-card__value">
              <span
                className={`status-indicator ${simulation || telemetry ? "status-indicator--active" : ""}`}
                aria-hidden="true"
              />
              <strong>{labelize(missionStatus)}</strong>
            </div>
          </article>

          <article className="status-card">
            <p>Spacecraft</p>
            <strong className="status-card__value status-card__value--mono">
              {spacecraftId}
            </strong>
          </article>

          <article className="status-card">
            <p>Flight mode</p>
            <strong
              className={`mode-badge mode-badge--${telemetry?.mode.toLowerCase() ?? "unknown"}`}
            >
              {telemetry?.mode ?? "Standby"}
            </strong>
          </article>

          <article className="status-card">
            <p>Latest packet</p>
            <strong className="status-card__value status-card__value--mono">
              {latestPacket}
            </strong>
          </article>
        </section>

        <div className="content-grid">
          <section className="panel" aria-labelledby="telemetry-title">
            <div className="panel__header">
              <div>
                <p className="eyebrow">Systems</p>
                <h2 id="telemetry-title">Live telemetry</h2>
              </div>
              {telemetry && (
                <span className="live-label">
                  <span aria-hidden="true" /> Live
                </span>
              )}
            </div>

            {telemetry?.components.length ? (
              <div className="component-list">
                {telemetry.components.map((component) => (
                  <article
                    className="component-card"
                    key={component.component_name}
                  >
                    <div className="component-card__header">
                      <span className="component-icon" aria-hidden="true">
                        <svg viewBox="0 0 24 24" fill="none">
                          <path d="M13 2 5.5 13h5L9 22l7.5-11h-5L13 2Z" />
                        </svg>
                      </span>
                      <div>
                        <h3>{labelize(component.component_name)}</h3>
                        <p>{component.data.length} data points</p>
                      </div>
                      <span className="health-label">Reporting</span>
                    </div>

                    <div className="metrics-grid">
                      {component.data.map((data) => (
                        <div className="metric" key={data.name}>
                          <p>{labelize(data.name)}</p>
                          <strong>
                            {formatTelemetryValue(data.name, data.value.value)}
                          </strong>
                          <small>{data.value.type}</small>
                        </div>
                      ))}
                    </div>
                  </article>
                ))}
              </div>
            ) : (
              <div className="empty-state">
                <span className="empty-state__icon" aria-hidden="true">
                  <svg viewBox="0 0 24 24" fill="none">
                    <path d="M4 17.5h3l2.2-6 3.2 10L15.7 8l1.8 4.5H20" />
                  </svg>
                </span>
                <h3>No telemetry received yet</h3>
                <p>
                  Create a simulation to begin receiving live spacecraft data.
                </p>
              </div>
            )}
          </section>

          <aside className="side-column">
            <section className="panel panel--compact" aria-labelledby="events-title">
              <div className="panel__header">
                <div>
                  <p className="eyebrow">Activity</p>
                  <h2 id="events-title">Event log</h2>
                </div>
                <span className="event-count">
                  {telemetry?.events.length ?? 0}
                </span>
              </div>

              {telemetry?.events.length ? (
                <div className="event-list">
                  {telemetry.events.map((event) => (
                    <article className="event" key={event.id}>
                      <span
                        className={`event__marker event__marker--${event.severity}`}
                        aria-hidden="true"
                      />
                      <div>
                        <div className="event__meta">
                          <span className={`severity severity--${event.severity}`}>
                            {event.severity}
                          </span>
                          <time dateTime={event.timestamp}>
                            {new Date(event.timestamp).toLocaleTimeString([], {
                              hour: "2-digit",
                              minute: "2-digit",
                            })}
                          </time>
                        </div>
                        <p>{event.message}</p>
                        <small>{labelize(event.source)}</small>
                      </div>
                    </article>
                  ))}
                </div>
              ) : (
                <div className="event-empty">
                  <span className="event-empty__check" aria-hidden="true">✓</span>
                  <div>
                    <strong>No active events</strong>
                    <p>New mission events will appear here.</p>
                  </div>
                </div>
              )}
            </section>

            <section className="panel panel--compact" aria-labelledby="details-title">
              <div className="panel__header panel__header--details">
                <div>
                  <p className="eyebrow">Session</p>
                  <h2 id="details-title">Simulation details</h2>
                </div>
              </div>

              <dl className="detail-list">
                <div>
                  <dt>Simulation ID</dt>
                  <dd title={simulation?.id ?? telemetry?.simulation_id}>
                    {simulation?.id ?? telemetry?.simulation_id ?? "—"}
                  </dd>
                </div>
                <div>
                  <dt>Created</dt>
                  <dd>
                    {simulation
                      ? new Date(simulation.created_at).toLocaleString([], {
                          dateStyle: "medium",
                          timeStyle: "short",
                        })
                      : "—"}
                  </dd>
                </div>
                <div>
                  <dt>Components</dt>
                  <dd>{telemetry?.components.length ?? 0}</dd>
                </div>
              </dl>
            </section>
          </aside>
        </div>
      </main>
    </div>
  );
}

export default App;
