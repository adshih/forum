import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ params }) {
    return {
        thread: await api.get(`api/threads/${params.slug}`),
        comments: await api.get(`api/threads/${params.slug}/comments`)
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();
        const jwt = cookies.get('jwt');
        const slug = request.url.split('/').slice(-1);

        if (!jwt) {
            throw redirect(307, '/login')
        }

        const body = await api.post(`api/threads/${slug}/comments`, {
            content: data.get('content')
        }, jwt);

        if (body.errors) {
            return fail(401, body);
        }
    }
}