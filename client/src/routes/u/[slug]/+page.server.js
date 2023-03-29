import * as api from '$lib/api';

export async function load({ params: { slug } }) {
    let profile = await api.get(`api/profiles/${slug}`);
    let threads = await api.get(`api/profiles/${slug}/threads`);

    return {
        profile,
        threads
    }
}