# BitDMX Legal Age Proof App

Privacy-preserving age verification using zero-knowledge proofs.

This Next.js app demonstrates how a user can prove they meet the **legal age requirement of a specific country** without revealing their exact birth date.

---

## Overview

Traditional age verification requires exposing sensitive personal data (e.g. full birth date, ID).  
BitDMX enables:

- Proving: **“I am of legal age in country X”**
- Without revealing:
  - exact birth date
  - identity
  - other personal data

This is achieved using:

- Commitment schemes (hashing birth date)
- Zero-knowledge proofs (zk-STARKs)
- On-chain/off-chain verification (optional)

---

## How It Works

### 1. User Input
- Country (e.g. `PE`, `US`, `JP`)
- Birth date (private)

### 2. Commitment
The app generates a commitment:

```

commitment = Hash(birthdate || randomness)

```

This commitment:
- hides the actual birth date
- can be stored or shared safely

---

### 3. Proof Generation
A zk-proof is generated that verifies:

```

age(birthdate, country) ≥ legal_age(country)
AND
Hash(birthdate || r) == commitment

```

---

### 4. Verification
A verifier (smart contract or backend) checks:

- Proof validity
- Commitment consistency

Result:

- ✅ Valid → user is of legal age
- ❌ Invalid → user does not meet requirement

---

## Supported Countries (Example)

| Country | Code | Legal Age |
|--------|------|----------|
| Peru | PE | 18 |
| USA | US | 21 |
| Japan | JP | 18 |
| Germany | DE | 18 |

> This list is configurable and extendable.

---

## Tech Stack

- Next.js (App Router)
- TypeScript
- zk-STARKs via Cairo (proof generation external or service)
- Hashing (SHA-256 or Poseidon recommended)

---

## Project Structure

```

/app
/api
prove/route.ts        # proof generation endpoint
verify/route.ts       # proof verification
/page.tsx               # main UI

/lib
age.ts                  # age calculation logic
countries.ts            # legal age per country
crypto.ts               # hash + commitment utils

/zk
circuit.cairo           # zk circuit (age proof)
prover.ts               # proof generation script

```

---

## Core Logic

### Age Calculation

```

age = current_date - birthdate

```

### Legal Check

```

isValid = age >= legal_age[country]

```

### Commitment Binding (critical)

```

Hash(birthdate || r) == commitment

````

---

## API Endpoints

### `POST /api/prove`

Generates a proof.

**Input:**
```json
{
  "country": "PE",
  "birthdate": "1998-04-10"
}
````

**Output:**

```json
{
  "commitment": "...",
  "proof": "...",
  "publicInputs": {
    "country": "PE"
  }
}
```

---

### `POST /api/verify`

Verifies a proof.

**Input:**

```json
{
  "commitment": "...",
  "proof": "...",
  "country": "PE"
}
```

**Output:**

```json
{
  "valid": true
}
```

---

## Security Notes

### Critical Requirements

* Commitment must be bound to proof:

  ```
  Hash(birthdate || r) == commitment
  ```

* Use the same hash function everywhere (frontend, backend, zk)

* Never expose:

  * raw birth date
  * randomness `r`

---

### Known Limitations (Prototype)

* No on-chain verification by default
* No data availability layer
* No identity binding (user could lie unless tied to credentials)

---

## Future Improvements

* Integration with verifiable credentials (government IDs)
* On-chain verification (StarkNet / Bitcoin via BitVMX)
* Multi-attribute proofs (age + nationality)
* Anonymous reputation layer

---

## Running Locally

```bash
git clone https://github.com/your-org/bitdmx-age-proof
cd bitdmx-age-proof

npm install
npm run dev
```

---

## Environment Variables

```
NEXT_PUBLIC_HASH_TYPE=sha256
PROVER_ENDPOINT=http://localhost:3001
```

---

## Use Cases

* KYC-lite onboarding
* Adult content gating
* Alcohol / cannabis ecommerce
* Gaming platforms
* DAO membership restrictions

---

## License

MIT

---

## Concept

BitDMX explores **accountable privacy**:

> Prove what is required. Reveal nothing else.
