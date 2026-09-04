
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

export type ManagedEvent = {
  event: Event;
  acknowledged: boolean;
}


export type SpacecraftTelemetry = {
  simulation_id: string;
  spacecraft_id: string;
  mode: Mode;
  components: ComponentTelemetry[];
  events: Event[];
  timestamp: string;
};

export type GroundState = {
  spacecraft_telemetry: SpacecraftTelemetry;
  managed_events: ManagedEvent[];
}
