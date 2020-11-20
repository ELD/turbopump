# Turbopump

Primitives for stateful sessions in Rocket. Providing multiple drivers:

- Cookie
- Database
- Cache (i.e. Redis, etc)
- In-Memory

### What belongs in the session store?

- Checking for auth? No. Auth is separate and tied to the session data, but not part of the actual session management.
- Reading and writing the session source? Yes.
- What belongs in the session store?
  - A unique identifier
  - Plain data
- Session operations
  - Start (implicit - onResponse?)
  - Read (implicit - RequestGuard)
  - Destroy (explicit? destroy entire cookie and all references to session)
  - Clear (clear the data in the cookie)

### Blueprint

- SessionFairing
  - Actual piece that's attached to the Rocket instance
  - on_attach -> store a `SessionHandler<S: SessionStore>` instance in Rocket's shared state
  - on_response -> set the session cookie if it hasn't been set yet
- SessionStore
  - Stores the actual session data (or proxies it to another store, i.e. Database, etc)
  - Basic session operations
    - load
    - store
    - clear
    - destroy
- Session

  - Actual type with a FromRequest impl
  - Contains the data stored by the handler
  - ```rust
      #[derive(Serialize, Deserialize)]
      struct Session {
        session_id: SessionId,
        data: HashMap<String, String>
      }

      impl FromRequest for Session {...}

      impl Default for Session {...}
    ```

### Supported versions

- Rocket 0.5 and beyond - async only
