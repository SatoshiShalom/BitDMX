import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { Layout } from './components/Layout'
import { Dashboard } from './pages/Dashboard'
import { Batches } from './pages/Batches'
import { Proofs } from './pages/Proofs'
import { Challenges } from './pages/Challenges'

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/batches" element={<Batches />} />
          <Route path="/proofs" element={<Proofs />} />
          <Route path="/challenges" element={<Challenges />} />
        </Routes>
      </Layout>
    </Router>
  )
}

export default App
