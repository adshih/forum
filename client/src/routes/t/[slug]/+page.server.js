import * as api from '$lib/api.js';

export async function load({ params }) {
    return await api.get(`api/threads/${params.slug}`)
}