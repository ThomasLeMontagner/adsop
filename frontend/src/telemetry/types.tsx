
type Mode = "Nominal" | "Degraded" | "Safe"

type DataValue = {
  type: string
  value: unknown
}

type Data = {
  name: string;
  value: DataValue;
}

type ComponentTelemetry = {
  component_name: string;
  data: Data[];
}

type ComponentEventType = string;

type EventType =
  | {
    component: ComponentEventType;
  }
  | {
    mode_change: {
      from: Mode;
      to: Mode;
    };
  };

type Severity = "information" | "warning" | "critical";

type Event = {
  id: number;
  timestamp: string;
  source: string;
  event_type: EventType;
  severity: Severity;
  message: string;
};

/** An event enriched with ground-side acknowledgement state. */
export type ManagedEvent = {
  event: Event;
  acknowledged: boolean;
}


/** A timestamped spacecraft state received from the backend. */
export type SpacecraftTelemetry = {
  simulation_id: string;
  spacecraft_id: string;
  mode: Mode;
  components: ComponentTelemetry[];
  timestamp: string;
};

/** The telemetry and managed-event snapshot delivered over WebSocket. */
export type GroundState = {
  spacecraft_telemetry: SpacecraftTelemetry;
  managed_events: ManagedEvent[];
}
