import { NextRequest, NextResponse } from 'next/server';

  const { commitment, proof, country } = await req.json();
  // Simulate proof verification: check proof format
  const valid = typeof proof === 'string' && proof.startsWith('proof_');
  return NextResponse.json({ valid });
}
