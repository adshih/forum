import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ params }) {
    return {
        thread: await api.get(`api/threads/${params.slug}`),
        comments: await api.get(`api/threads/${params.slug}/comments`),
        votes: await api.get(`api/threads/${params.slug}/vote`)
    }
}

export const actions = {
    comment: async ({ cookies, request, params: { slug } }) => {
        const data = await request.formData();
        const jwt = cookies.get('jwt');

        if (!jwt) {
            throw redirect(307, '/login');
        }

        const body = await api.post(`api/threads/${slug}/comments`, {
            content: data.get('content')
        }, jwt);

        if (body.errors) {
            return fail(401, body);
        }
    },

    vote: async ({ cookies, params: { slug } }) => {
        const jwt = cookies.get('jwt');

        if (!jwt) {
            throw redirect(307, '/login');
        }

        const body = await api.post(`api/threads/${slug}/vote`, {}, jwt);

        if (body.errors) {
            return fail(401, body);
        }

        return body;
    }
}