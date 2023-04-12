import express from 'express';
import fs from 'fs';
import https from 'https';
import { handler } from './build/handler.js';

const app = express();

app.use(handler);

const httpsServer = https.createServer({
    key: fs.readFileSync('/etc/letsencrypt/live/forum.adamshih.dev/privkey.pem'),
    cert: fs.readFileSync('/etc/letsencrypt/live/forum.adamshih.dev/fullchain.pem'),
}, app);


httpsServer.listen(443, () => {
    console.log('HTTPS Server running on port 443');
});