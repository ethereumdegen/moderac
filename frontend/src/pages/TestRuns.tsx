import { useEffect, useState } from 'react'
import { Link, useParams } from 'react-router'
import { api, type TestRun } from '../lib/api'
import { useSession } from '../lib/auth-client'
import SiteHeader from '../components/SiteHeader'

export default function TestRuns() {
  const { id: projectId } = useParams()
  const { data: session } = useSession()
  const [runs, setRuns] = useState<TestRun[]>([])

  useEffect(() => {
    if (!projectId) return
    api.listRuns(projectId).then(setRuns)
  }, [projectId])

  const statusColor: Record<string, string> = {
    passed: 'text-green',
    failed: 'text-red',
    running: 'text-yellow',
    pending: 'text-text-muted',
    error: 'text-red',
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={session?.user} />
      <div className="flex-1 max-w-4xl mx-auto w-full px-6 py-10">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-2xl font-bold">Test Runs</h1>
          <Link to={`/projects/${projectId}`} className="text-sm text-text-muted hover:text-text">
            Back to project
          </Link>
        </div>

        {runs.length === 0 ? (
          <p className="text-text-muted text-sm py-8 text-center">No test runs yet.</p>
        ) : (
          <div className="space-y-2">
            {runs.map(r => (
              <Link
                key={r.id}
                to={`/projects/${projectId}/runs/${r.id}`}
                className="flex items-center justify-between p-4 bg-bg-card border border-border rounded-lg hover:border-border-hover transition-colors"
              >
                <div className="flex items-center gap-4">
                  <span className={`text-sm font-medium ${statusColor[r.status] || 'text-text-muted'}`}>
                    {r.status}
                  </span>
                  <span className="text-sm text-text-muted font-mono">{r.id.slice(0, 8)}</span>
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
  )
}
