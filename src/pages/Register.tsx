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
    <div className="mx-auto max-w-md p-8">
      <h1 className="mb-6 text-3xl font-bold">ESC Vote — Register</h1>
      {!ready && <p className="text-neutral-400">Loading…</p>}
      {!connected && <p className="text-neutral-400">Connecting…</p>}
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="country"
            className="mb-1 block text-sm text-neutral-400"
          >
            Country
          </label>
          <select
            id="country"
            value={countryId?.toString() ?? ""}
            onChange={(e) => {
              const val = e.target.value;
              setCountryId(val ? Number(val) : null);
              if (!val) setIsJuror(false);
            }}
            disabled={!connected}
            className="w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
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
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={isJuror}
              onChange={(e) => setIsJuror(e.target.checked)}
              className="rounded"
            />
            I am a juror
          </label>
        )}

        {isJuror && (
          <div>
            <label
              htmlFor="name"
              className="mb-1 block text-sm text-neutral-400"
            >
              Name
            </label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Your name"
              required
              className="w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        )}

        <button
          type="submit"
          disabled={!connected}
          className="rounded-lg bg-blue-600 px-4 py-2 font-medium transition hover:bg-blue-500 disabled:opacity-50"
        >
          Register
        </button>
      </form>
    </div>
  );
}
