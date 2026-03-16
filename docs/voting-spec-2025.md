# Eurovision Song Contest 2025 — Voting Process Specification

Eurovision 2025 was held in Basel, Switzerland, with 37 participating countries.

## Contest Structure

The contest consists of three shows:

1. **Semi-Final 1** (13 May 2025)
2. **Semi-Final 2** (15 May 2025)
3. **Grand Final** (17 May 2025)

### Automatic Finalists ("Big 5" + Host)

The following countries qualify directly to the Grand Final without competing in a semi-final:

- France, Germany, Italy, Spain, United Kingdom (Big 5 — largest financial contributors to the EBU)
- Switzerland (host country)

All other participating countries must qualify through one of the two semi-finals.

---

## Semi-Final Voting (100% Televote)

Semi-finals use **televoting only** — no jury vote.

### Who Can Vote

- Viewers in all participating countries can vote via phone, SMS, or the official Eurovision app.
- Viewers **cannot vote for their own country**.
- Each viewer can cast up to **20 votes** per show (per phone number / payment method).
- Viewers in non-participating countries can vote online; these are aggregated as the **"Rest of the World"** vote.

### How Votes Become Points

Each country's televote results are ranked. The top 10 songs in each country's ranking receive points:

| Rank | Points |
|------|--------|
| 1st  | 12     |
| 2nd  | 10     |
| 3rd  | 8      |
| 4th  | 7      |
| 5th  | 6      |
| 6th  | 5      |
| 7th  | 4      |
| 8th  | 3      |
| 9th  | 2      |
| 10th | 1      |

Songs ranked 11th or lower receive **0 points** from that country.

The "Rest of the World" vote also awards one set of points using the same scale.

### Qualification

The **top 10 songs** by total points in each semi-final advance to the Grand Final.

### San Marino Exception

San Marino does not have an independent telephone network, so it cannot organise a public televote. In the semi-finals, a **jury vote** is used instead. In the Grand Final, the televote is simulated by an algorithm.

### Fallback

If a country's televoting cannot deliver a valid result, a backup **jury** is used for that country.

---

## Grand Final Voting (50% Jury + 50% Televote)

The Grand Final uses a **combined system**: each country awards **two independent sets of points** — one from a professional jury and one from the public televote.

### Jury Vote

- Each country assembles a **national jury of 5 music industry professionals** appointed by the national broadcaster.
- Jurors must have the nationality of the country they represent.
- Jurors **cannot vote for their own country**.
- The jury watches and votes during a **Jury Show** (a full dress rehearsal held the day before the live Grand Final).
- Jurors assess performances on four criteria:
  1. Overall impression of the performance
  2. Composition and originality
  3. Vocal ability of the artist
  4. Performance and staging
- Individual juror rankings are aggregated using an **exponential weight model** (giving more weight to higher-ranked songs, reducing the impact of one juror placing a song unusually low).
- The aggregated jury ranking awards points to the top 10: **12, 10, 8, 7, 6, 5, 4, 3, 2, 1**.

### Televote

- Same rules as semi-finals: phone, SMS, or app; up to 20 votes; cannot vote for own country.
- Each country's televote ranking awards a separate set of **12, 10, 8, 7, 6, 5, 4, 3, 2, 1** to the top 10.
- The "Rest of the World" online vote awards an additional set of points on the same scale.

### Total Score

A country's Grand Final score = sum of all jury point sets + sum of all televote point sets.

### Fallback Rules

- If a country's **jury** cannot deliver a valid result → that country's televote result is used in its place.
- If a country's **televote** cannot deliver a valid result → an aggregated result is used. If that also fails, the jury result is used.

---

## Results Announcement (Grand Final)

1. **Jury points** are announced first: a spokesperson from each country reads out only the **12-point** recipient live on air. Points 1–10 are added to the scoreboard automatically.
2. **Televote points** are then announced by the presenters in a combined fashion — countries are revealed in order from lowest to highest total televote score, building suspense.

---

## Tie-Breaking Rules

If two or more countries are tied on total points:

1. The country with **more televote points** wins.
2. If still tied: the country that received televote points from **more countries** wins.
3. If still tied: the country with more **12-point televote scores** wins, then 10, then 8, and so on down to 1.
4. If still tied after all of the above: the country that **performed earlier** in the running order wins.

This applies to all positions, not just first place.

---

## Maximum Possible Score (2025)

With 37 participating countries + the "Rest of the World" vote:

- **Jury max**: (37 − 1) × 12 = **432** (36 other countries × 12 points each)
- **Televote max**: (37 − 1 + 1) × 12 = **444** (36 other countries + Rest of the World × 12 points each)
- **Theoretical maximum**: 432 + 444 = **876 points**

---

## How Votes Are Cast

### Jurors

Each juror individually **ranks all performing songs** (excluding their own country) from best to worst. They assess performances on four criteria: overall impression, composition and originality, vocal ability, and staging. The 5 individual rankings per country are then aggregated using an exponential weight model into a single national jury ranking, and the top 10 receive points (12, 10, 8, 7, 6, 5, 4, 3, 2, 1).

### Televoters (Public / Rest of the World)

Viewers vote via phone, SMS, or the official app. Each viewer has a budget of **20 votes**. The app presents all performing countries, and the viewer taps a country to give it +1 vote. The same country can be tapped multiple times. Votes can be distributed however the viewer likes — all 20 to one country, or spread across several. There is no ranking involved; each vote simply adds +1 to that song's tally. All votes from a country are then tallied into a ranking, and the top 10 receive points on the same scale.

### Key Difference

- Jurors: **full ranking** of all songs (ordered list)
- Televoters: **pick favourites** (up to 20 unordered votes)

---

## Summary Table

| Aspect              | Semi-Finals              | Grand Final                        |
|---------------------|--------------------------|------------------------------------|
| Voting method       | 100% televote            | 50% jury + 50% televote            |
| Points per country  | 1 set (12→1)             | 2 sets (jury: 12→1, televote: 12→1)|
| Rest of the World   | Yes (1 set)              | Yes (1 set televote)               |
| Max votes per viewer| 20                       | 20                                 |
| Vote for own country| No                       | No                                 |
| Jury show           | N/A                      | Day before the live final           |
| Qualification       | Top 10 advance           | Winner = highest total points       |

---

Content was rephrased for compliance with licensing restrictions.

Sources:
- [Eurovision voting system explained — eurovisionandfriends.com](https://www.eurovisionandfriends.com/en/systeme-vote-eurovision-jury-televote/)
- [Voting at the Eurovision Song Contest — Wikipedia](https://en.wikipedia.org/wiki/Voting_at_the_Eurovision_Song_Contest)
