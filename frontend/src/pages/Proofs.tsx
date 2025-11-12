export function Proofs() {
  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-8">STARK Proofs</h1>
      
      <div className="bg-gray-900 rounded-lg border border-gray-800 overflow-hidden">
        <table className="min-w-full divide-y divide-gray-800">
          <thead className="bg-gray-800">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Proof ID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Batch ID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Commitment
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Bitcoin TX
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Status
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-800">
            <tr>
              <td colSpan={5} className="px-6 py-4 text-center text-gray-400">
                No proofs yet
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  )
}
