import { useState } from "react";
import { Link } from "react-router";
import { tables } from "../module_bindings";
import { useTable } from "spacetimedb/react";
import type { RankingKind } from "../module_bindings/types";

export default function Landing() {
  const [[activeRound]] = useTable(tables.get_active_round);

  if (!activeRound)
    return <div className="p-8 text-neutral-400">No active round.</div>;

  return <RoundRankings roundId={activeRound.id} />;
}

function RoundRankings({ roundId }: { roundId: number }) {
  const [countries] = useTable(tables.country);
  const [participatingCountries] = useTable(tables.participating_country);
  const [[rotwCountry]] = useTable(tables.rotw_country);
  const [jurors] = useTable(tables.juror);

  const [rankings] = useTable(
    tables.ranking.where((row) => row.roundId.eq(roundId)),
  );
  const [jurorVotes] = useTable(
    tables.juror_vote.where((row) => row.roundId.eq(roundId)),
  );

  const [selectedCountryId, setSelectedCountryId] = useState<number | null>(
    null,
  );

  const countryById = new Map(
    countries.map((country) => [country.id, country]),
  );

  const countryDisplayName = (participatingCountryId: number): string => {
    const participatingCountry = participatingCountries.find(
      (pc) => pc.id === participatingCountryId,
    );
    if (!participatingCountry) return "?";
    const country = countryById.get(participatingCountry.countryId);
    if (!country) return "?";
    return `${country.emoji ?? ""} ${country.name}`;
  };

  const filterRankings = (kind: RankingKind["tag"], fromCountryId: number) =>
    rankings
      .filter(
        (ranking) =>
          ranking.kind.tag === kind && ranking.fromCountryId === fromCountryId,
      )
      .sort((a, b) => a.rank - b.rank);

  const overallRankings = filterRankings(
    "Overall",
    rotwCountry?.countryId ?? -1,
  );

  const teleRankings =
    selectedCountryId != null
      ? filterRankings("TeleVote", selectedCountryId)
      : [];

  const selectedParticipatingCountry =
    selectedCountryId != null
      ? participatingCountries.find((pc) => pc.countryId === selectedCountryId)
      : undefined;

  const jurorBreakdown = selectedParticipatingCountry
    ? jurors
        .filter(
          (juror) =>
            juror.participatingCountryId === selectedParticipatingCountry.id,
        )
        .map((juror) => ({
          name: juror.name,
          votes: jurorVotes
            .filter((vote) => vote.jurorId === juror.id)
            .sort((a, b) => a.rank - b.rank),
        }))
    : [];

  return (
    <div className="mx-auto max-w-4xl p-6 space-y-8">
      <section>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Overall Rankings</h2>
          <Link
            to="/vote"
            className="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium hover:bg-blue-500"
          >
            Vote →
          </Link>
        </div>
        <table className="w-full text-left">
          <thead className="text-neutral-400 text-sm border-b border-neutral-800">
            <tr>
              <th className="py-2 w-16">#</th>
              <th className="py-2">Country</th>
              <th className="py-2 text-right">Points</th>
            </tr>
          </thead>
          <tbody>
            {overallRankings.map((ranking) => (
              <tr
                key={ranking.id.toString()}
                className="border-b border-neutral-800/50"
              >
                <td className="py-2 text-neutral-400">{ranking.rank + 1}</td>
                <td className="py-2">
                  {countryDisplayName(ranking.toCountryId)}
                </td>
                <td className="py-2 text-right font-mono">{ranking.score}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      <section>
        <h2 className="text-xl font-bold mb-4">Votes by Country</h2>
        <select
          value={selectedCountryId?.toString() ?? ""}
          onChange={(event) =>
            setSelectedCountryId(
              event.target.value ? Number(event.target.value) : null,
            )
          }
          className="mb-4 w-full rounded-lg bg-neutral-800 px-3 py-2 outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Select a country…</option>
          {countries.map((country) => (
            <option key={country.id} value={country.id}>
              {country.emoji ?? ""} {country.name}
            </option>
          ))}
        </select>

        {selectedCountryId != null && (
          <div className="grid gap-6 md:grid-cols-2">
            <div>
              <h3 className="text-sm font-semibold text-neutral-400 mb-2">
                Televotes
              </h3>
              {teleRankings.length === 0 ? (
                <p className="text-neutral-500 text-sm">No votes</p>
              ) : (
                <table className="w-full text-left text-sm">
                  <thead className="text-neutral-500 border-b border-neutral-800">
                    <tr>
                      <th className="py-1 w-10">#</th>
                      <th className="py-1">Country</th>
                      <th className="py-1 text-right">Votes</th>
                    </tr>
                  </thead>
                  <tbody>
                    {teleRankings.map((ranking) => (
                      <tr
                        key={ranking.id.toString()}
                        className="border-b border-neutral-800/30"
                      >
                        <td className="py-1 text-neutral-400">
                          {ranking.rank + 1}
                        </td>
                        <td className="py-1">
                          {countryDisplayName(ranking.toCountryId)}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {ranking.score}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </div>

            <div>
              <h3 className="text-sm font-semibold text-neutral-400 mb-2">
                Juror Rankings
              </h3>
              {jurorBreakdown.length === 0 ? (
                <p className="text-neutral-500 text-sm">No jurors</p>
              ) : (
                <div className="space-y-4">
                  {jurorBreakdown.map((juror) => (
                    <div key={juror.name}>
                      <p className="text-sm font-medium mb-1">{juror.name}</p>
                      {juror.votes.length === 0 ? (
                        <p className="text-neutral-500 text-xs">
                          No ranking submitted
                        </p>
                      ) : (
                        <ol className="text-sm text-neutral-300 space-y-0.5">
                          {juror.votes.map((vote) => (
                            <li key={vote.rank} className="flex gap-2">
                              <span className="text-neutral-500 w-5 text-right">
                                {vote.rank}.
                              </span>
                              {countryDisplayName(vote.rankedCountryId)}
                            </li>
                          ))}
                        </ol>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </section>
    </div>
  );
}
