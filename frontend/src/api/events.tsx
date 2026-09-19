export async function acknowledgeEvent(eventID: number): Promise<void> {
    if (!Number.isSafeInteger(eventID) || eventID < 0) {
        throw new Error("Invalid event ID");
    }
    const eventIdSegment = encodeURIComponent(String(eventID));
    const response = await fetch(`api/events/${eventIdSegment}/acknowledge`, {
        method: "POST",
    });

    if (!response.ok) {
        throw new Error(`Failed to acknowledge event: ${response.status}`);
    }
}
