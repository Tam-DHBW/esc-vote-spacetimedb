import { tables } from "./module_bindings";
import { useTable } from "spacetimedb/react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router";
import Register from "./pages/Register.tsx";

function Home() {
  const [[voter], ready] = useTable(tables.current_voter);
  const [countries] = useTable(tables.country);

  if (!ready) return <p>Loading…</p>;
  if (!voter) return <Navigate to="/register" />;

  const { role } = voter;
  const countryId = role.tag !== "World" ? role.value.countryId.value : null;
  const country =
    countryId != null ? countries.find((c) => c.id === countryId) : null;

  return (
    <div style={{ padding: "2rem" }}>
      <h1>ESC Vote</h1>
      <p>
        Role: {role.tag === "Juror" ? `Juror (${role.value.name})` : role.tag}
      </p>
      {country && (
        <p>
          Country: {country.name} ({country.code})
        </p>
      )}
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
