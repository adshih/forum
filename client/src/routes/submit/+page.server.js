import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (!cookies.get('jwt')) {
        throw redirect(307, '/login');
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();
        const jwt = cookies.get('jwt');

        const body = await api.post('api/threads', {
            title: data.get('title'),
            content: data.get('content')
        }, jwt);

        if (body.errors) {
            return fail(401, body);
        }

        throw redirect(303, '/recent');
    }
}