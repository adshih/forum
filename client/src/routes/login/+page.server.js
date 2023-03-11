import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (cookies.get('jwt')) {
        throw redirect(307, '/');
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();

        const body = await api.post('api/users/login', {
            username: data.get('username'),
            password: data.get('password')
        });

        if (body.errors) {
            return fail(401, body);
        }

        cookies.set('jwt', JSON.stringify(body.token).slice(1, -1), { path: '/' });

        throw redirect(303, '/');
    }
}