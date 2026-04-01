import { BrowserRouter, Routes, Route } from 'react-router'
import Landing from './pages/Landing'
import SignIn from './pages/SignIn'
import Dashboard from './pages/Dashboard'
import ProjectDetail from './pages/ProjectDetail'
import TestEditor from './pages/TestEditor'
import TestRuns from './pages/TestRuns'
import RunDetail from './pages/RunDetail'
import ApiKeys from './pages/ApiKeys'

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Landing />} />
        <Route path="/signin" element={<SignIn />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/projects/:id" element={<ProjectDetail />} />
        <Route path="/projects/:id/tests/new" element={<TestEditor />} />
        <Route path="/projects/:id/tests/:testId" element={<TestEditor />} />
        <Route path="/projects/:id/runs" element={<TestRuns />} />
        <Route path="/projects/:id/runs/:runId" element={<RunDetail />} />
        <Route path="/projects/:id/keys" element={<ApiKeys />} />
      </Routes>
    </BrowserRouter>
  )
}
