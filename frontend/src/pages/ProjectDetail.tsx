import { useEffect, useState } from 'react'
import { Link, useParams } from 'react-router'
import { api, type Project, type Test, type TestRun } from '../lib/api'
import SiteHeader from '../components/SiteHeader'

export default function ProjectDetail() {
  const { id } = useParams()
  const [project, setProject] = useState<Project | null>(null)
  const [tests, setTests] = useState<Test[]>([])
  const [runs, setRuns] = useState<TestRun[]>([])
  const [user, setUser] = useState<{ id: string; email: string } | null>(null)
  const [running, setRunning] = useState(false)

  useEffect(() => {
    if (!id) return
    api.me().then(setUser)
    api.getProject(id).then(setProject)
    api.listTests(id).then(setTests)
    api.listRuns(id).then(setRuns)
  }, [id])

  async function handleRunTests() {
    if (!id) return
    setRunning(true)
    try {
      const run = await api.createRun(id)
      setRuns(prev => [run, ...prev])
    } finally {
      setRunning(false)
    }
  }

  if (!project) return <div className="min-h-screen bg-bg" />

  const statusColor: Record<string, string> = {
    passed: 'text-green',
    failed: 'text-red',
    running: 'text-yellow',
    pending: 'text-text-muted',
    error: 'text-red',
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={user} />
      <div className="flex-1 max-w-4xl mx-auto w-full px-6 py-10">
        <div className="flex items-center justify-between mb-2">
          <h1 className="text-2xl font-bold">{project.name}</h1>
          <div className="flex gap-3">
            <Link to={`/projects/${id}/keys`} className="px-3 py-1.5 border border-border hover:border-border-hover rounded-lg text-sm text-text-muted hover:text-text transition-colors">
              API Keys
            </Link>
            <button
              onClick={handleRunTests}
              disabled={running || tests.length === 0}
              className="px-4 py-1.5 bg-accent hover:bg-accent-hover disabled:opacity-50 text-white rounded-lg text-sm font-medium transition-colors"
            >
              {running ? 'Running...' : 'Run tests'}
            </button>
          </div>
        </div>
        {project.description && <p className="text-text-muted mb-8">{project.description}</p>}

        {/* Tests */}
        <div className="mt-8">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Tests</h2>
            <Link
              to={`/projects/${id}/tests/new`}
              className="px-3 py-1.5 border border-border hover:border-border-hover rounded-lg text-sm text-text-muted hover:text-text transition-colors"
            >
              New test
            </Link>
          </div>
          {tests.length === 0 ? (
            <p className="text-text-muted text-sm py-8 text-center">No tests defined yet.</p>
          ) : (
            <div className="space-y-2">
              {tests.map(t => (
                <Link
                  key={t.id}
                  to={`/projects/${id}/tests/${t.id}`}
                  className="block p-4 bg-bg-card border border-border rounded-lg hover:border-border-hover transition-colors"
                >
                  <div className="font-medium">{t.name}</div>
                  <div className="text-sm text-text-muted mt-1 truncate">{t.prompt}</div>
                </Link>
              ))}
            </div>
          )}
        </div>

        {/* Recent Runs */}
        <div className="mt-10">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Recent runs</h2>
            {runs.length > 0 && (
              <Link to={`/projects/${id}/runs`} className="text-sm text-text-muted hover:text-text">
                View all
              </Link>
            )}
          </div>
          {runs.length === 0 ? (
            <p className="text-text-muted text-sm py-8 text-center">No test runs yet.</p>
          ) : (
            <div className="space-y-2">
              {runs.slice(0, 5).map(r => (
                <Link
                  key={r.id}
                  to={`/projects/${id}/runs/${r.id}`}
                  className="flex items-center justify-between p-4 bg-bg-card border border-border rounded-lg hover:border-border-hover transition-colors"
                >
                  <div className="flex items-center gap-3">
                    <span className={`text-sm font-medium ${statusColor[r.status] || 'text-text-muted'}`}>
                      {r.status}
                    </span>
                    <span className="text-xs text-text-muted">{r.id.slice(0, 8)}</span>
                  </div>
                  <span className="text-xs text-text-muted">
                    {r.completed_at ? new Date(r.completed_at).toLocaleString() : 'In progress'}
                  </span>
                </Link>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
