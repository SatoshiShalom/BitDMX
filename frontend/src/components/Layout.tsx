import { ReactNode } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { Zap, Box, Shield, AlertTriangle } from 'lucide-react'

interface LayoutProps {
  children: ReactNode
}

export function Layout({ children }: LayoutProps) {
  const location = useLocation()

  const navItems = [
    { path: '/', label: 'Dashboard', icon: Zap },
    { path: '/batches', label: 'Batches', icon: Box },
    { path: '/proofs', label: 'Proofs', icon: Shield },
    { path: '/challenges', label: 'Challenges', icon: AlertTriangle },
  ]

  return (
    <div className="min-h-screen bg-bitcoin-dark">
      <nav className="bg-gray-900 border-b border-gray-800">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center">
              <Zap className="h-8 w-8 text-bitcoin-orange" />
              <span className="ml-2 text-xl font-bold text-white">BitVMX-Z Explorer</span>
            </div>
            <div className="flex space-x-4">
              {navItems.map(({ path, label, icon: Icon }) => (
                <Link
                  key={path}
                  to={path}
                  className={`flex items-center px-3 py-2 rounded-md text-sm font-medium ${
                    location.pathname === path
                      ? 'bg-gray-800 text-bitcoin-orange'
                      : 'text-gray-300 hover:bg-gray-800 hover:text-white'
                  }`}
                >
                  <Icon className="h-4 w-4 mr-2" />
                  {label}
                </Link>
              ))}
            </div>
          </div>
        </div>
      </nav>
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {children}
      </main>
    </div>
  )
}
