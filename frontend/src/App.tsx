import { useEffect, useState } from "react";

type Simulation = {
  id: string;
  status: string;
  created_at: string;
};

type Telemetry = {
  simulation_id: string;
  spacecraft_id: string;
  battery_voltage: number;
};



function App() {
  const [simulation, setSimulation] = useState<Simulation | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [telemetry, SetTelemetry] = useState<Telemetry | null>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080/ws");

    socket.onopen = () => {
      console.log("Connected to ADSOP telemetry stream");
    };

    socket.onmessage = (event) => {
      const telemetry: Telemetry = JSON.parse(event.data);

      SetTelemetry(telemetry);
    };

    socket.onerror = (event) => {
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
              <strong>Spacecraft:</strong>{telemetry.spacecraft_id}
            </p>
            <p>
              <strong>Battery voltage:</strong>{" "}
              {telemetry.battery_voltage.toFixed(2)} V
            </p>
          </section>
        )
      }
    </main>
  );
}

export default App;