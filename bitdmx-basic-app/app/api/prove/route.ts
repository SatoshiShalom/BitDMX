import { NextRequest, NextResponse } from 'next/server';

import { generateRandomness, generateCommitment } from '../../../lib/crypto';
import { isOfLegalAge } from '../../../lib/age';
import { generateProof } from '../../../zk/prover';

export async function POST(req: NextRequest) {
  const { country, birthdate } = await req.json();
  if (!country || !birthdate) {
    return NextResponse.json({ error: 'Missing country or birthdate' }, { status: 400 });
  }
  const randomness = generateRandomness();
  const commitment = generateCommitment(birthdate, randomness);
  const valid = isOfLegalAge(birthdate, country);
  let proof = null;
  if (valid) {
    proof = await generateProof(birthdate, randomness, country);
  }
  return NextResponse.json({
    commitment,
    proof,
    publicInputs: { country },
    valid
  });
}
