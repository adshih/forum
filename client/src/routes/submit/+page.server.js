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

        const title = data.get('title');
        const content = data.get('content');

        if (title == '' || content == '') {
            return fail(422);
        }

        const body = await api.post('api/threads', {
            title,
            content
        }, jwt);

        if (body.errors) {
            return fail(401, body);
        }

        throw redirect(303, '/recent');
    }
}