export function timeSince(initial_time) {
    const initial = new Date(initial_time);
    const now = new Date();

    const millis = now.getTime() - initial.getTime();
    const seconds = Math.floor(millis / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    let res = '';

    if (days) {
        res = `${days} days ago`;
    } else if (hours) {
        res = `${hours} hours ago`;
    } else if (minutes) {
        res = `${minutes} minutes ago`;
    } else {
        res = `${seconds} seconds ago`;
    }

    return res;
}