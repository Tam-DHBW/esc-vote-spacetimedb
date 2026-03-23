import { useState } from "react";
import { Navigate } from "react-router";
import { tables, reducers } from "../module_bindings";
import { useTable, useReducer } from "spacetimedb/react";

export default function Register() {
  const [countries] = useTable(tables.country);
  const [participatingCountries] = useTable(tables.participating_country);
  const [[currentUser]] = useTable(tables.current_user);
  const register = useReducer(reducers.register);

  const [isJuror, setIsJuror] = useState(false);
  const [countryId, setCountryId] = useState<number | null>(null);
  const [name, setName] = useState("");

  if (currentUser) return <Navigate to="/" />;

  const participatingCountryIds = new Set(
    participatingCountries.map((pc) => pc.countryId),
  );

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    if (countryId == null) return;

    if (isJuror) {
      const participatingCountry = participatingCountries.find(
        (pc) => pc.countryId === countryId,
      );
      if (!participatingCountry) return;
      register({
        role: {
          tag: "Juror",
          value: {
            participatingCountryId: { value: participatingCountry.id },
            name,
          },
        },
      });
    } else {
      register({
        role: {
          tag: "Viewer",
          value: { countryId: { value: countryId } },
        },
      });
    }
  };

  return (
    <div className="mx-auto max-w-md p-8">
      <h1 className="mb-6 text-2xl font-bold">Register</h1>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="mb-1 block text-sm text-neutral-400">Role</label>
          <div className="flex gap-4">
            <label className="flex items-center gap-2">
              <input
                type="radio"
                checked={!isJuror}
                onChange={() => setIsJuror(false)}
              />
              Televoter
            </label>
            <label className="flex items-center gap-2">
              <input
                type="radio"
                checked={isJuror}
                onChange={() => setIsJuror(true)}
              />
              Juror
            </label>
          </div>
        </div>

        <div>
          <label className="mb-1 block text-sm text-neutral-400">Country</label>
          <select
            value={countryId?.toString() ?? ""}
            onChange={(event) =>
              setCountryId(
                event.target.value ? Number(event.target.value) : null,
              )
            }
            className="w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Select country…</option>
            {countries
              .filter(
                (country) =>
                  !isJuror || participatingCountryIds.has(country.id),
              )
              .map((country) => (
                <option key={country.id} value={country.id}>
                  {country.emoji ?? ""} {country.name}
                </option>
              ))}
          </select>
        </div>

        {isJuror && (
          <div>
            <label className="mb-1 block text-sm text-neutral-400">Name</label>
            <input
              type="text"
              value={name}
              onChange={(event) => setName(event.target.value)}
              required
              placeholder="Your name"
              className="w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        )}

        <button
          type="submit"
          disabled={!countryId || (isJuror && !name)}
          className="rounded-lg bg-blue-600 px-4 py-2 font-medium hover:bg-blue-500 disabled:opacity-50"
        >
          Register
        </button>
      </form>
    </div>
  );
}
