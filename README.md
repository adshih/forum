# Forum App

*This application is still in early development.*

## Build Instructions
The client and server need to be run seperately.

For the server to work, a postgres instance must be setup with the schema outlined in `db/init.sql`.

**Run Server**:
```
cd server
cargo run
```

**Run Client**:
```
cd client
npm install
npm run dev
```

After everything is up and running, the website will be up and running at `http://localhost:3000`.