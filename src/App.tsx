import { tables } from "./module_bindings";
import { useTable } from "spacetimedb/react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router";
import Register from "./pages/Register.tsx";

function Home() {
  const [[voter], ready] = useTable(tables.current_voter);
  const [countries] = useTable(tables.country);

  if (!ready) return <p className="p-8 text-neutral-400">Loading…</p>;
  if (!voter) return <Navigate to="/register" />;

  const { role } = voter;
  const countryId = role.tag !== "World" ? role.value.countryId.value : null;
  const country =
    countryId != null ? countries.find((c) => c.id === countryId) : null;

  return (
    <div className="mx-auto max-w-lg p-8">
      <h1 className="mb-6 text-3xl font-bold">ESC Vote</h1>
      <div className="space-y-2 rounded-lg bg-neutral-800 p-6">
        <p>
          <span className="text-neutral-400">Role:</span>{" "}
          {role.tag === "Juror" ? `Juror (${role.value.name})` : role.tag}
        </p>
        {country && (
          <p>
            <span className="text-neutral-400">Country:</span> {country.name} (
            {country.code})
          </p>
        )}
      </div>
    </div>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/register" element={<Register />} />
      </Routes>
    </BrowserRouter>
  );
}
