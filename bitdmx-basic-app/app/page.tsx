"use client";
import React, { useState } from 'react';
import { countries } from '../lib/countries';

export default function HomePage() {
  const [country, setCountry] = useState('PE');
  const [birthdate, setBirthdate] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const handleProve = async () => {
    setLoading(true);
    setResult(null);
    const res = await fetch('/api/prove', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ country, birthdate })
    });
    const data = await res.json();
    setResult(data);
    setLoading(false);
  };

  return (
    <main className="max-w-lg mx-auto mt-16 p-8 bg-white rounded-lg shadow-lg border border-gray-200">
      <h1 className="text-3xl font-bold mb-8 text-gray-900">BITDMX Legal Age Proof</h1>
      <form
        className="flex flex-col gap-6"
        onSubmit={e => { e.preventDefault(); handleProve(); }}
      >
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Country</label>
          <select
            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-black focus:ring-black p-2"
            value={country}
            onChange={e => setCountry(e.target.value)}
          >
            {Object.entries(countries).map(([code, name]) => (
              <option key={code} value={code}>{name}</option>
            ))}
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Birthdate</label>
          <input
            type="date"
            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-black focus:ring-black p-2"
            value={birthdate}
            onChange={e => setBirthdate(e.target.value)}
          />
        </div>
        <button
          type="submit"
          className="bg-black text-white py-2 px-4 rounded-md hover:bg-gray-800 disabled:opacity-50"
          disabled={loading || !birthdate}
        >
          {loading ? 'Proving...' : 'Prove Age'}
        </button>
      </form>
      {result && (
        <pre className="bg-gray-100 mt-8 p-4 rounded text-sm overflow-x-auto border border-gray-200">
          {JSON.stringify(result, null, 2)}
        </pre>
      )}
    </main>
  );
}
