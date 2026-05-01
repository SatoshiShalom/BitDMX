import { getLegalAge } from './countries';

export function calculateAge(birthdate: string, currentDate: string = new Date().toISOString().slice(0, 10)): number {
  const birth = new Date(birthdate);
  const current = new Date(currentDate);
  let age = current.getFullYear() - birth.getFullYear();
  const m = current.getMonth() - birth.getMonth();
  if (m < 0 || (m === 0 && current.getDate() < birth.getDate())) {
    age--;
  }
  return age;
}

export function isOfLegalAge(birthdate: string, country: string, currentDate?: string): boolean {
  const age = calculateAge(birthdate, currentDate);
  return age >= getLegalAge(country);
}
