import { useState } from "react";
import { Navigate } from "react-router";
import { tables, reducers } from "../module_bindings";
import { useSpacetimeDB, useTable, useReducer } from "spacetimedb/react";

export default function Register() {
  const { isActive: connected } = useSpacetimeDB();
  const [countries] = useTable(tables.country);
  const [[voter], ready] = useTable(tables.current_voter);
  const register = useReducer(reducers.register);

  const [countryId, setCountryId] = useState<number | null>(null);
  const [isJuror, setIsJuror] = useState(false);
  const [name, setName] = useState("");

  if (voter) return <Navigate to="/" />;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!connected) return;

    const role =
      countryId == null
        ? { tag: "World" as const }
        : isJuror
          ? {
              tag: "Juror" as const,
              value: { countryId: { value: countryId }, name },
            }
          : { tag: "Rep" as const, value: { countryId: { value: countryId } } };

    register({ role });
  };

  return (
    <div style={{ padding: "2rem", maxWidth: "400px", margin: "0 auto" }}>
      <h1>ESC Vote — Register</h1>
      {!ready && <p>Loading…</p>}
      {!connected && <p>Connecting…</p>}
      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: "1rem" }}>
          <label htmlFor="country">Country</label>
          <br />
          <select
            id="country"
            value={countryId?.toString() ?? ""}
            onChange={(e) => {
              const val = e.target.value;
              setCountryId(val ? Number(val) : null);
              if (!val) setIsJuror(false);
            }}
            disabled={!connected}
            style={{ width: "100%", padding: "0.5rem" }}
          >
            <option value="">None (Rest of the World)</option>
            {countries.map((c) => (
              <option key={c.id} value={c.id}>
                {c.name}
              </option>
            ))}
          </select>
        </div>

        {countryId != null && (
          <div style={{ marginBottom: "1rem" }}>
            <label>
              <input
                type="checkbox"
                checked={isJuror}
                onChange={(e) => setIsJuror(e.target.checked)}
              />{" "}
              I am a juror
            </label>
          </div>
        )}

        {isJuror && (
          <div style={{ marginBottom: "1rem" }}>
            <label htmlFor="name">Name</label>
            <br />
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Your name"
              required
              style={{ width: "100%", padding: "0.5rem" }}
            />
          </div>
        )}

        <button
          type="submit"
          disabled={!connected}
          style={{ padding: "0.5rem 1rem" }}
        >
          Register
        </button>
      </form>
    </div>
  );
}
