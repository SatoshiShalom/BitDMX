export const countries: Record<string, string> = {
  PE: 'Peru',
  US: 'USA',
  JP: 'Japan',
  DE: 'Germany',
};

export const legalAges: Record<string, number> = {
  PE: 18,
  US: 21,
  JP: 18,
  DE: 18,
};

export function getLegalAge(country: string): number {
  return legalAges[country] || 18;
}
