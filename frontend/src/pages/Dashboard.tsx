import { useEffect, useState } from 'react'
import { Link } from 'react-router'
import { api, type Project } from '../lib/api'
import SiteHeader from '../components/SiteHeader'

export default function Dashboard() {
  const [projects, setProjects] = useState<Project[]>([])
  const [user, setUser] = useState<{ id: string; email: string } | null>(null)
  const [showCreate, setShowCreate] = useState(false)
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')

  useEffect(() => {
    api.me().then(setUser)
    api.listProjects().then(setProjects)
  }, [])

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault()
    const project = await api.createProject({ name, description: description || undefined })
    setProjects(prev => [project, ...prev])
    setShowCreate(false)
    setName('')
    setDescription('')
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={user} />
      <div className="flex-1 max-w-4xl mx-auto w-full px-6 py-10">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-2xl font-bold">Projects</h1>
          <button
            onClick={() => setShowCreate(true)}
            className="px-4 py-2 bg-accent hover:bg-accent-hover text-white rounded-lg text-sm font-medium transition-colors"
          >
            New project
          </button>
        </div>

        {showCreate && (
          <form onSubmit={handleCreate} className="mb-8 p-6 bg-bg-card border border-border rounded-xl space-y-4">
            <input
              type="text"
              placeholder="Project name"
              value={name}
              onChange={e => setName(e.target.value)}
              required
              className="w-full px-4 py-2 bg-bg border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent"
            />
            <input
              type="text"
              placeholder="Description (optional)"
              value={description}
              onChange={e => setDescription(e.target.value)}
              className="w-full px-4 py-2 bg-bg border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent"
            />
            <div className="flex gap-3">
              <button type="submit" className="px-4 py-2 bg-accent hover:bg-accent-hover text-white rounded-lg text-sm font-medium">
                Create
              </button>
              <button type="button" onClick={() => setShowCreate(false)} className="px-4 py-2 text-text-muted hover:text-text text-sm">
                Cancel
              </button>
            </div>
          </form>
        )}

        {projects.length === 0 ? (
          <div className="text-center py-20 text-text-muted">
            <p className="text-lg mb-2">No projects yet</p>
            <p className="text-sm">Create your first project to start defining prompt-based tests.</p>
          </div>
        ) : (
          <div className="space-y-3">
            {projects.map(p => (
              <Link
                key={p.id}
                to={`/projects/${p.id}`}
                className="block p-5 bg-bg-card border border-border rounded-xl hover:border-border-hover transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-semibold">{p.name}</h3>
                    {p.description && <p className="text-sm text-text-muted mt-1">{p.description}</p>}
                  </div>
                  <span className="text-xs text-text-muted">{new Date(p.created_at).toLocaleDateString()}</span>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
