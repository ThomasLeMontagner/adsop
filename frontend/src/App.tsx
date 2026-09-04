import type { SpacecraftTelemetry } from "./telemetry/types"
import { useEffect, useState } from "react";

type Simulation = {
  id: string;
  status: string;
  created_at: string;
};



function App() {
  const [simulation, setSimulation] = useState<Simulation | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [telemetry, SetTelemetry] = useState<SpacecraftTelemetry | null>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080/ws");

    socket.onopen = () => {
      console.log("Connected to ADSOP telemetry stream");
    };

    socket.onmessage = (event) => {
      const telemetry: SpacecraftTelemetry = JSON.parse(event.data);

      SetTelemetry(telemetry);
    };

    socket.onerror = (_) => {
      console.log("Disconnected from ADSOP telemetry stream");
    };

    return () => {
      socket.close();
    };
  }, []);

  async function createSimulation() {
    setIsCreating(true);

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

  return (
    <main>
      <h1>ADSOP Mission Control</h1>

      <button onClick={createSimulation} disabled={isCreating}>
        {isCreating ? "Creating simulation..." : "Create simulation"}
      </button>

      {error && <p>Could not create simulation: {error}</p>}

      {simulation && (
        <section>
          <h2>Simulation created</h2>
          <p>
            <strong>ID:</strong> {simulation.id}
          </p>
          <p>
            <strong>Status:</strong> {simulation.status}
          </p>
          <p>
            <strong>Created:</strong>{" "}
            {new Date(simulation.created_at).toLocaleString()}
          </p>
        </section>
      )}

      {telemetry && (
        <section>
          <h2>Live telemetry</h2>

          <p>
            <strong>Spacecraft:</strong> {telemetry.spacecraft_id}
          </p>

          <p>
            <strong>Mode:</strong> {telemetry.mode}
          </p>

          {telemetry.components.map((component) => (
            <div key={component.component_name}>
              <h3>{component.component_name}</h3>
              {component.data.map((data) => (
                <p key={data.name}>
                  <strong>{data.name}:</strong> {String(data.value.value)}
                </p>
              ))}
            </div>
          ))}
          {telemetry.events.map((event) => (
            <div key={event.id}>
              <strong>{event.severity}</strong>
              <span>{event.message}</span>
                <small>
                  {event.source} - {new Date(event.timestamp).toLocaleString()}
                </small>
            </div>
          ))}
        </section>
      )
      }
    </main>
  );
}

export default App;