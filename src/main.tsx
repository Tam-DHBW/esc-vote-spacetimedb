import "./index.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { SpacetimeDBProvider } from "spacetimedb/react";

import App from "./App.tsx";
import { DbConnection } from "./module_bindings/index.ts";

const HOST = import.meta.env.VITE_SPACETIMEDB_HOST;
const DB_NAME = import.meta.env.VITE_SPACETIMEDB_DB_NAME;
const TOKEN_KEY = `${HOST}/${DB_NAME}/auth_token`;

const root = createRoot(document.getElementById("root")!);

const connectionBuilder = DbConnection.builder()
  .withUri(HOST)
  .withDatabaseName(DB_NAME)
  .withToken(localStorage.getItem(TOKEN_KEY) || undefined)
  .onConnect((conn, identity, token) => {
    localStorage.setItem(TOKEN_KEY, token);
    console.log("Connected:", identity.toHexString());
    conn.subscriptionBuilder().subscribeToAllTables();
  })
  .onDisconnect(() => console.log("Disconnected"))
  .onConnectError((_ctx, err) => {
    console.error("Connection error:", err);
    root.render(
      <StrictMode>
        <div className="flex h-screen items-center justify-center text-neutral-400">
          <p>Could not connect to the database.</p>
        </div>
      </StrictMode>,
    );
  });

root.render(
  <StrictMode>
    <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
      <App />
    </SpacetimeDBProvider>
  </StrictMode>,
);
