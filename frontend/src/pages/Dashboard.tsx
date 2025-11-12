import { useEffect, useState } from 'react'
import { Activity, Database, Shield, Clock } from 'lucide-react'
import axios from 'axios'

interface Stats {
  blockHeight: number
  totalBatches: number
  activeChallenges: number
  uptime: string
}

export function Dashboard() {
  const [stats, setStats] = useState<Stats | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const response = await axios.get('/api/state')
        setStats({
          blockHeight: response.data.block_height,
          totalBatches: 0,
          activeChallenges: 0,
          uptime: '99.9%',
        })
      } catch (error) {
        console.error('Failed to fetch stats:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchStats()
    const interval = setInterval(fetchStats, 5000)
    return () => clearInterval(interval)
  }, [])

  if (loading) {
    return <div className="text-white">Loading...</div>
  }

  const statCards = [
    { label: 'Block Height', value: stats?.blockHeight || 0, icon: Database, color: 'text-blue-400' },
    { label: 'Total Batches', value: stats?.totalBatches || 0, icon: Shield, color: 'text-green-400' },
    { label: 'Active Challenges', value: stats?.activeChallenges || 0, icon: Activity, color: 'text-yellow-400' },
    { label: 'Uptime', value: stats?.uptime || '0%', icon: Clock, color: 'text-purple-400' },
  ]

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-8">Dashboard</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {statCards.map(({ label, value, icon: Icon, color }) => (
          <div key={label} className="bg-gray-900 rounded-lg p-6 border border-gray-800">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">{label}</p>
                <p className="text-2xl font-bold text-white mt-2">{value}</p>
              </div>
              <Icon className={`h-8 w-8 ${color}`} />
            </div>
          </div>
        ))}
      </div>

      <div className="bg-gray-900 rounded-lg p-6 border border-gray-800">
        <h2 className="text-xl font-bold text-white mb-4">Recent Activity</h2>
        <div className="text-gray-400">
          <p>No recent activity</p>
        </div>
      </div>
    </div>
  )
}
