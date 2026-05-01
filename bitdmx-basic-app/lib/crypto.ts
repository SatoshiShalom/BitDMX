import { createHash, randomBytes } from 'crypto';

export function generateRandomness(length = 16): string {
  return randomBytes(length).toString('hex');
}

export function generateCommitment(birthdate: string, randomness: string): string {
  const hash = createHash('sha256');
  hash.update(birthdate + randomness);
  return hash.digest('hex');
}
