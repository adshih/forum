import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies, params }) {
    const jwt = cookies.get('jwt');
    const parent = await api.get(`api/threads/${params.slug}/comments/${params.comment_id}`);
    const children = await api.get(`api/threads/${params.slug}/comments/${params.comment_id}/children`);
    const thread = await api.get(`api/threads/${params.slug}`, jwt);

    return {
        parent,
        children,
        thread
    }
}

export const actions = {
    vote: async ({ request, cookies, params: { slug } }) => {
        const jwt = cookies.get('jwt');

        if (!jwt) {
            throw redirect(301, '/login');
        }

        const data = await request.formData();
        const id = data.get('id');
        const body = await api.post(`api/threads/${slug}/comments/${id}/vote`, {}, jwt);

        if (body.errors) {
            return fail(400, body);
        }

        return body;
    },
    unvote: async ({ request, cookies, params: { slug } }) => {
        const jwt = cookies.get('jwt');

        if (!jwt) {
            throw redirect(301, '/login');
        }

        const data = await request.formData();
        const id = data.get('id');
        const body = await api.post(`api/threads/${slug}/comments/${id}/unvote`, {}, jwt);

        if (body.errors) {
            return fail(400, body);
        }

        return body;
    },
    reply: async ({ cookies, request, params: { slug, comment_id } }) => {
        const data = await request.formData();
        const jwt = cookies.get('jwt');
        const content = data.get('content');

        if (!jwt) {
            throw redirect(302, '/login');
        }

        if (content == '') {
            return fail(422);
        }

        const body = await api.post(`api/threads/${slug}/comments/${comment_id}`, {
            content
        }, jwt);

        if (body.errors) {
            return fail(401, body);
        }
    },
}