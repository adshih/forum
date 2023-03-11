import * as api from '$lib/api.js';
import { fail, redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (!cookies.get('jwt')) {
        throw redirect(307, '/');
    }
}

export const actions = {
    default: async ({ cookies, request }) => {
        cookies.delete('jwt');
        throw redirect(303, '/');
    }
}