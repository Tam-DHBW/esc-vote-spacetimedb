import { useState, useEffect } from "react";
import { tables } from "./module_bindings";
import { useTable } from "spacetimedb/react";
import { BrowserRouter, Routes, Route, Link } from "react-router";
import { formatRoundKind } from "./util";
import Landing from "./pages/Landing";
import Vote from "./pages/Vote";
import Register from "./pages/Register";
import Admin from "./pages/Admin";

export default function App() {
  const [[activeRound]] = useTable(tables.get_active_round);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const handler = (event: PromiseRejectionEvent) =>
      setError(event.reason?.message ?? String(event.reason));
    window.addEventListener("unhandledrejection", handler);
    return () => window.removeEventListener("unhandledrejection", handler);
  }, []);

  return (
    <BrowserRouter>
      {error && <ErrorToast message={error} onClose={() => setError(null)} />}
      <nav className="flex items-center gap-4 border-b border-neutral-800 px-6 py-3 text-sm">
        <Link to="/" className="font-bold text-lg">
          🎤 ESC Vote
        </Link>
        <Link to="/" className="text-neutral-400 hover:text-white">
          Rankings
        </Link>
        <Link to="/vote" className="text-neutral-400 hover:text-white">
          Vote
        </Link>
        <Link to="/admin" className="text-neutral-400 hover:text-white">
          Admin
        </Link>
        <span className="ml-auto flex items-center gap-4">
          {activeRound && (
            <span className="text-neutral-500">
              {activeRound.year} — {formatRoundKind(activeRound.kind.tag)}
            </span>
          )}
          <UserBadge />
        </span>
      </nav>
      <Routes>
        <Route path="/" element={<Landing />} />
        <Route path="/vote" element={<Vote />} />
        <Route path="/register" element={<Register />} />
        <Route path="/admin" element={<Admin />} />
      </Routes>
    </BrowserRouter>
  );
}

function ErrorToast({
  message,
  onClose,
}: {
  message: string;
  onClose: () => void;
}) {
  useEffect(() => {
    const timeout = setTimeout(onClose, 5000);
    return () => clearTimeout(timeout);
  }, [onClose]);

  return (
    <div className="fixed top-4 right-4 z-50 max-w-sm rounded border border-red-800 bg-red-950 px-4 py-3 text-sm text-red-200 shadow-lg">
      <div className="flex items-start gap-2">
        <span className="flex-1">{message}</span>
        <button onClick={onClose} className="text-red-400 hover:text-red-200">
          ✕
        </button>
      </div>
    </div>
  );
}

function UserBadge() {
  const [[user]] = useTable(tables.current_user);
  const [countries] = useTable(tables.country);
  const [participatingCountries] = useTable(tables.participating_country);

  if (!user) {
    return (
      <Link
        to="/register"
        className="rounded border border-neutral-700 px-3 py-1 text-xs text-neutral-400 hover:border-neutral-500 hover:text-white transition"
      >
        Register
      </Link>
    );
  }

  const { role } = user;

  const resolveCountry = (id: number | undefined) =>
    countries.find((country) => country.id === id);

  let countryId: number | undefined;
  let displayName: string;

  if (role.tag === "Juror") {
    const participatingCountry = participatingCountries.find(
      (pc) => pc.id === role.value.participatingCountryId.value,
    );
    countryId = participatingCountry?.countryId;
    displayName = role.value.name;
  } else {
    countryId = role.value.countryId.value;
    displayName = resolveCountry(countryId)?.name ?? "?";
  }

  const country = resolveCountry(countryId);
  const label = `${country?.emoji ?? ""} ${displayName}`;

  return (
    <button
      className="rounded border border-neutral-700 px-3 py-1 text-xs text-neutral-400 hover:border-neutral-500 hover:text-white transition"
      onClick={() => {
        localStorage.clear();
        location.reload();
      }}
    >
      {label} · Sign out
    </button>
  );
}
