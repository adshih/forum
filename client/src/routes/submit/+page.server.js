import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (!cookies.get('jwt')) {
        throw redirect(307, '/');
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();

        const body = await api.post('api/threads', {
            username: data.get('title'),
            password: data.get('content')
        });

        if (body.errors) {
            console.log(body.errors);
            // return fail(401, body);
        }

        // throw redirect(303, '/');
    }
}