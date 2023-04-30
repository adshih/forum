import * as api from '$lib/api.js';

export async function load({ cookies, params }) {
    const jwt = cookies.get('jwt');
    console.log(params);
    const parent = await api.get(`api/threads/${params.slug}/comments/${params.comment_id}`);
    const children = await api.get(`api/threads/${params.slug}/comments/${params.comment_id}/children`);
    const thread = await api.get(`api/threads/${params.slug}`, jwt);

    return {
        parent,
        children,
        thread
    }
}