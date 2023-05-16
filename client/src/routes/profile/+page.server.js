import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    const jwt = cookies.get('jwt');

    if (!jwt) {
        throw redirect(307, '/');
    }

    const profile = await api.get(`api/users`, jwt);
    const threads = await api.get(`api/profiles/${profile.username}/threads`, jwt);

    return {
        profile,
        threads
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        cookies.delete('jwt');
        throw redirect(303, '/');
    }
}