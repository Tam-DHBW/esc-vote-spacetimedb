import { useState, useMemo, useEffect } from "react";
import { Navigate } from "react-router";
import { tables, reducers } from "../module_bindings";
import { useTable, useReducer } from "spacetimedb/react";

type VotableCountry = { participatingCountryId: number; name: string };

function useVotableCountries(): VotableCountry[] {
  const [votable] = useTable(tables.votable_countries);
  const [participatingCountries] = useTable(tables.participating_country);
  const [countries] = useTable(tables.country);

  return useMemo(() => {
    const participatingCountryById = new Map(
      participatingCountries.map((pc) => [pc.id, pc]),
    );
    const countryById = new Map(
      countries.map((country) => [country.id, country]),
    );

    return votable.map((entry) => {
      const participatingCountry = participatingCountryById.get(
        entry.participatingCountryId.value,
      );
      const country = participatingCountry
        ? countryById.get(participatingCountry.countryId)
        : undefined;

      return {
        participatingCountryId: entry.participatingCountryId.value,
        name: country ? `${country.emoji ?? ""} ${country.name}` : "?",
      };
    });
  }, [votable, participatingCountries, countries]);
}

function useSubmitFeedback(): {
  submitted: boolean;
  submit: (action: () => void) => void;
} {
  const [submitted, setSubmitted] = useState(false);

  const submit = (action: () => void) => {
    action();
    setSubmitted(true);
    setTimeout(() => setSubmitted(false), 2000);
  };

  return { submitted, submit };
}

export default function Vote() {
  const [[user]] = useTable(tables.current_user);
  const [[activeRound]] = useTable(tables.get_active_round);
  const countries = useVotableCountries();

  if (!user) return <Navigate to="/register" />;
  if (!activeRound)
    return <div className="p-8 text-neutral-400">No active round.</div>;
  if (!activeRound.votingOpen)
    return <div className="p-8 text-neutral-400">Voting is closed.</div>;
  if (countries.length === 0)
    return (
      <div className="p-8 text-neutral-400">
        Voting is not available right now.
      </div>
    );

  return user.role.tag === "Juror" ? (
    <JurorVote countries={countries} />
  ) : (
    <TeleVote countries={countries} />
  );
}

function TeleVote({ countries }: { countries: VotableCountry[] }) {
  const submitTeleVotes = useReducer(reducers.submitTeleVotes);
  const [votes, setVotes] = useState<Map<number, number>>(new Map());
  const { submitted, submit } = useSubmitFeedback();

  const totalVotes = useMemo(
    () => [...votes.values()].reduce((sum, count) => sum + count, 0),
    [votes],
  );

  const adjustVote = (countryId: number, delta: number) => {
    setVotes((prev) => {
      const next = new Map(prev);
      const newCount = (next.get(countryId) ?? 0) + delta;
      if (newCount > 0) {
        next.set(countryId, newCount);
      } else {
        next.delete(countryId);
      }
      return next;
    });
  };

  const handleSubmit = () => {
    const voteArray: { value: number }[] = [];
    for (const [countryId, count] of votes) {
      for (let i = 0; i < count; i++) {
        voteArray.push({ value: countryId });
      }
    }
    submit(() => submitTeleVotes({ votes: voteArray }));
  };

  return (
    <div className="mx-auto max-w-lg p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold">Televote</h2>
        <span className="text-neutral-400">{totalVotes}/20 votes</span>
      </div>
      <div className="space-y-2">
        {countries.map((country) => (
          <div
            key={country.participatingCountryId}
            className="flex items-center justify-between rounded-lg bg-neutral-800 px-4 py-2"
          >
            <span>{country.name}</span>
            <div className="flex items-center gap-2">
              <button
                onClick={() => adjustVote(country.participatingCountryId, -1)}
                disabled={!votes.has(country.participatingCountryId)}
                className="rounded bg-neutral-700 px-2 py-0.5 text-sm hover:bg-neutral-600 disabled:opacity-30"
              >
                −
              </button>
              <span className="w-6 text-center font-mono">
                {votes.get(country.participatingCountryId) ?? 0}
              </span>
              <button
                onClick={() => adjustVote(country.participatingCountryId, 1)}
                disabled={totalVotes >= 20}
                className="rounded bg-neutral-700 px-2 py-0.5 text-sm hover:bg-neutral-600 disabled:opacity-30"
              >
                +
              </button>
            </div>
          </div>
        ))}
      </div>
      <button
        onClick={handleSubmit}
        disabled={totalVotes === 0}
        className={`w-full rounded-lg py-2 font-medium ${submitted ? "bg-green-600" : "bg-blue-600 hover:bg-blue-500 disabled:opacity-50"}`}
      >
        {submitted ? "✓ Votes Submitted" : "Submit Votes"}
      </button>
    </div>
  );
}

function JurorVote({ countries }: { countries: VotableCountry[] }) {
  const submitJurorVotes = useReducer(reducers.submitJurorVotes);
  const [ranking, setRanking] = useState<VotableCountry[]>([]);
  const { submitted, submit } = useSubmitFeedback();

  useEffect(() => {
    if (countries.length > 0) {
      setRanking([...countries]);
    }
  }, [countries]);

  const move = (index: number, direction: -1 | 1) => {
    setRanking((prev) => {
      const next = [...prev];
      [next[index], next[index + direction]] = [
        next[index + direction],
        next[index],
      ];
      return next;
    });
  };

  const handleSubmit = () => {
    submit(() =>
      submitJurorVotes({
        ranking: ranking.map((country) => ({
          value: country.participatingCountryId,
        })),
      }),
    );
  };

  return (
    <div className="mx-auto max-w-lg p-6 space-y-4">
      <h2 className="text-xl font-bold">Juror Ranking</h2>
      <p className="text-sm text-neutral-400">
        Rank from best (top) to worst (bottom)
      </p>
      <div className="space-y-1">
        {ranking.map((country, index) => (
          <div
            key={country.participatingCountryId}
            className="flex items-center justify-between rounded-lg bg-neutral-800 px-4 py-2"
          >
            <span>
              <span className="text-neutral-500 mr-2">{index + 1}.</span>
              {country.name}
            </span>
            <div className="flex gap-1">
              <button
                onClick={() => move(index, -1)}
                disabled={index === 0}
                className="rounded bg-neutral-700 px-2 py-0.5 text-sm hover:bg-neutral-600 disabled:opacity-30"
              >
                ↑
              </button>
              <button
                onClick={() => move(index, 1)}
                disabled={index === ranking.length - 1}
                className="rounded bg-neutral-700 px-2 py-0.5 text-sm hover:bg-neutral-600 disabled:opacity-30"
              >
                ↓
              </button>
            </div>
          </div>
        ))}
      </div>
      <button
        onClick={handleSubmit}
        disabled={ranking.length === 0}
        className={`w-full rounded-lg py-2 font-medium ${submitted ? "bg-green-600" : "bg-blue-600 hover:bg-blue-500 disabled:opacity-50"}`}
      >
        {submitted ? "✓ Ranking Submitted" : "Submit Ranking"}
      </button>
    </div>
  );
}
