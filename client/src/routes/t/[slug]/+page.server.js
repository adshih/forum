import * as api from '$lib/api.js';
import { compile } from 'mdsvex';

export async function load({ params }) {
    const res = await api.get(`api/threads/${params.slug}`);

    return res;
}