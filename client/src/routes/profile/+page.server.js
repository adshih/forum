import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    const jwt = cookies.get('jwt');

    if (!jwt) {
        throw redirect(307, '/');
    }

    return {
        profile: await api.get(`api/users`, jwt)
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        cookies.delete('jwt');
        throw redirect(303, '/');
    }
}