import * as api from '$lib/api.js';
import { redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
    if (!cookies.get('jwt')) {
        throw redirect(307, '/login');
    }
}