import * as api from '$lib/api.js';

export async function load({ cookies }) {
    let user;
    let jwt = cookies.get('jwt');

    if (jwt) {
        user = await api.get('api/users', jwt);
    }

    return {
        user: user
    }
}   