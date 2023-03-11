import * as api from '$lib/api.js';

export async function load() {
    return {
        threads: await api.get('api/threads')
    }
}
