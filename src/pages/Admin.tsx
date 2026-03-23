import { useState } from "react";
import { tables, reducers } from "../module_bindings";
import { useTable, useReducer } from "spacetimedb/react";
import { formatRoundKind } from "../util";

export default function Admin() {
  const [[activeRound]] = useTable(tables.get_active_round);
  const createSemiFinals = useReducer(reducers.createSemiFinals);
  const advanceRound = useReducer(reducers.advanceRound);

  const [year, setYear] = useState(new Date().getFullYear());

  return (
    <div className="mx-auto max-w-md p-8 space-y-6">
      <h1 className="text-2xl font-bold">Admin</h1>

      <div className="rounded-lg bg-neutral-800 p-4 space-y-1">
        <p className="text-sm text-neutral-400">Current Status</p>
        {activeRound ? (
          <p className="font-medium">
            {activeRound.year} — {formatRoundKind(activeRound.kind.tag)}
          </p>
        ) : (
          <p className="text-neutral-500">No active round</p>
        )}
      </div>

      {!activeRound ? (
        <div className="space-y-3">
          <label className="block text-sm text-neutral-400">Year</label>
          <input
            type="number"
            value={year}
            onChange={(event) => setYear(Number(event.target.value))}
            className="w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={() => createSemiFinals({ year })}
            className="w-full rounded-lg bg-green-600 py-2 font-medium hover:bg-green-500"
          >
            Create Semi-Finals
          </button>
        </div>
      ) : (
        <button
          onClick={() => advanceRound()}
          className="w-full rounded-lg bg-amber-600 py-2 font-medium hover:bg-amber-500"
        >
          {activeRound.kind.tag === "GrandFinal"
            ? "Conclude Contest"
            : "Advance to Next Round"}
        </button>
      )}
    </div>
  );
}
