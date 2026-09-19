export async function acknowledgeEvent(eventID: number): Promise<void> {
    const response = await fetch(`api/events/${eventID}/acknowledge`, {
        method: "POST",
    });

    if (!response.ok) {
        throw new Error(`Failed to acknowledge event: ${response.status}`);
    }
}
