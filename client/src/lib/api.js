import { error } from '@sveltejs/kit';
import { API_URL } from '$env/static/private';

const base = API_URL;

async function send({ method, path, data, token }) {
    const opts = { method, headers: {} };

    if (!path) {
        path = '';
    }

    if (data) {
        opts.headers['Content-Type'] = 'application/json';
        opts.body = JSON.stringify(data);
    }

    if (token) {
        opts.headers['Authorization'] = `Bearer ${token}`;
    }

    const url = `${base}/${path}`;

    const res = await fetch(url, opts);
    console.log(res);
    if (res.ok || res.status === 422) {
        const text = await res.text();
        return text ? JSON.parse(text) : {};
    }

    throw error(res.status);
}

export function get(path, token) {
    return send({ method: 'GET', path, token });
}

export function del(path, token) {
    return send({ method: 'DELETE', path, token });
}

export function post(path, data, token) {
    return send({ method: 'POST', path, data, token });
}

export function put(path, data, token) {
    return send({ method: 'PUT', path, data, token });
}