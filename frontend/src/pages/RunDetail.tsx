import { useEffect, useState } from 'react'
import { Link, useParams } from 'react-router'
import { api, type RunDetail as RunDetailType } from '../lib/api'
import SiteHeader from '../components/SiteHeader'

export default function RunDetail() {
  const { id: projectId, runId } = useParams()
  const [data, setData] = useState<RunDetailType | null>(null)
  const [user, setUser] = useState<{ id: string; email: string } | null>(null)

  useEffect(() => {
    if (!projectId || !runId) return
    api.me().then(setUser)
    api.getRun(projectId, runId).then(setData)
  }, [projectId, runId])

  const statusColor: Record<string, string> = {
    passed: 'text-green bg-green/10',
    failed: 'text-red bg-red/10',
    error: 'text-red bg-red/10',
  }

  if (!data) return <div className="min-h-screen bg-bg" />

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={user} />
      <div className="flex-1 max-w-4xl mx-auto w-full px-6 py-10">
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-2xl font-bold">Run {data.run.id.slice(0, 8)}</h1>
            <p className="text-text-muted text-sm mt-1">
              Status: <span className={statusColor[data.run.status]?.split(' ')[0] || 'text-text-muted'}>{data.run.status}</span>
              {data.run.completed_at && <> &middot; Completed {new Date(data.run.completed_at).toLocaleString()}</>}
            </p>
          </div>
          <Link to={`/projects/${projectId}/runs`} className="text-sm text-text-muted hover:text-text">
            All runs
          </Link>
        </div>

        <div className="space-y-3">
          {data.results.map(r => (
            <div key={r.id} className="p-5 bg-bg-card border border-border rounded-xl">
              <div className="flex items-center justify-between mb-3">
                <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${statusColor[r.status] || 'text-text-muted bg-bg-hover'}`}>
                  {r.status}
                </span>
                <div className="flex items-center gap-4 text-xs text-text-muted">
                  {r.score !== null && <span>Score: {(r.score * 100).toFixed(0)}%</span>}
                  {r.duration_ms !== null && <span>{r.duration_ms}ms</span>}
                </div>
              </div>
              {r.response && (
                <div className="mb-2">
                  <div className="text-xs text-text-muted mb-1">Response</div>
                  <p className="text-sm">{r.response}</p>
                </div>
              )}
              {r.evaluation && (
                <div>
                  <div className="text-xs text-text-muted mb-1">Evaluation</div>
                  <p className="text-sm text-text-muted">{r.evaluation}</p>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
