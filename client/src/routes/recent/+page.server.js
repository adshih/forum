import * as api from '$lib/api.js';

export async function load({ cookies }) {
    const jwt = cookies.get('jwt');

    return {
        threads: await api.get('api/threads', jwt)
    }
}