# Forum App

*This application is still in early development.*

## Setup
* A postgres instance must be setup with the schema outlined in `db/init.sql`.
* Create a `.env` file located at `/server` specifying a `DATABASE_URL` and `JWT_SECRET`.  
Alternatively just rename `.env.sample` to `.env`.

## Build and Run

**Server**:
```
cd server
cargo run
```

**Client**:
```
cd client
npm install
npm run dev
```

After everything is up and running, the server will be up and running at `http://localhost:3000` and client at `http://localhost:5172` if the ports are unoccupied.