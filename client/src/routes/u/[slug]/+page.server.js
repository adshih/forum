import * as api from '$lib/api';

export async function load({ cookies, params: { slug } }) {
    const jwt = cookies.get('jwt');
    let profile = await api.get(`api/profiles/${slug}`);
    let threads = await api.get(`api/profiles/${slug}/threads`, jwt);


    return {
        profile,
        threads
    }
}