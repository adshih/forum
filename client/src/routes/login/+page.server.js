import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (cookies.get('jwt')) {
        throw redirect(302, '/');
    }
}

export const actions = {
    login: async ({ cookies, request }) => {
        const data = await request.formData();

        const username = data.get('username');
        const password = data.get('password');

        if (username == '' || password == '') {
            return fail(422);
        }
            
        const body = await api.post('api/users/login', {
            username,
            password
        });

        if (body.errors) {
            return fail(401, body);
        }

        cookies.set('jwt', body.token, { path: '/' });

        throw redirect(302, '/');
    }
}