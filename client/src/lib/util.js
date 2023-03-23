export function hoursSince(datetime) {
    const now = new Date();
    const dif_millis = datetime.getTime() - now.getTime();
    const dif_hours = Math.ceil(dif_millis / (1000 * 60 * 60));
    return Math.abs(dif_hours);
}