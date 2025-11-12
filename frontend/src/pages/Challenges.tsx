import { AlertTriangle } from 'lucide-react'

export function Challenges() {
  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-8">Active Challenges</h1>
      
      <div className="bg-gray-900 rounded-lg border border-gray-800 overflow-hidden">
        <table className="min-w-full divide-y divide-gray-800">
          <thead className="bg-gray-800">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Challenge ID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Batch ID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Challenger
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Deadline
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                Status
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-800">
            <tr>
              <td colSpan={5} className="px-6 py-4 text-center text-gray-400">
                <div className="flex items-center justify-center">
                  <AlertTriangle className="h-5 w-5 mr-2 text-green-400" />
                  <span>No active challenges</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div className="mt-8 bg-gray-900 rounded-lg p-6 border border-gray-800">
        <h2 className="text-xl font-bold text-white mb-4">Challenge Statistics</h2>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <p className="text-gray-400 text-sm">Total Challenges</p>
            <p className="text-2xl font-bold text-white mt-1">0</p>
          </div>
          <div>
            <p className="text-gray-400 text-sm">Resolved</p>
            <p className="text-2xl font-bold text-green-400 mt-1">0</p>
          </div>
          <div>
            <p className="text-gray-400 text-sm">Disputed</p>
            <p className="text-2xl font-bold text-red-400 mt-1">0</p>
          </div>
        </div>
      </div>
    </div>
  )
}
