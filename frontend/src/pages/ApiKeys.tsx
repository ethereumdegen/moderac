import { useEffect, useState } from 'react'
import { Link, useParams } from 'react-router'
import { api, type ApiKey } from '../lib/api'
import { useSession } from '../lib/auth-client'
import SiteHeader from '../components/SiteHeader'

export default function ApiKeys() {
  const { id: projectId } = useParams()
  const { data: session } = useSession()
  const [keys, setKeys] = useState<ApiKey[]>([])
  const [name, setName] = useState('')
  const [showCreate, setShowCreate] = useState(false)
  const [newKey, setNewKey] = useState<string | null>(null)

  useEffect(() => {
    if (!projectId) return
    api.listApiKeys(projectId).then(setKeys)
  }, [projectId])

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault()
    if (!projectId) return
    const created = await api.createApiKey(projectId, name)
    setNewKey(created.key)
    setKeys(prev => [{ id: created.id, project_id: projectId, key_prefix: created.key_prefix, name: created.name, created_at: new Date().toISOString(), revoked_at: null }, ...prev])
    setShowCreate(false)
    setName('')
  }

  async function handleRevoke(keyId: string) {
    if (!projectId) return
    if (!confirm('Revoke this API key?')) return
    await api.revokeApiKey(projectId, keyId)
    setKeys(prev => prev.map(k => k.id === keyId ? { ...k, revoked_at: new Date().toISOString() } : k))
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={session?.user} />
      <div className="flex-1 max-w-4xl mx-auto w-full px-6 py-10">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-2xl font-bold">API Keys</h1>
          <div className="flex gap-3">
            <Link to={`/projects/${projectId}`} className="text-sm text-text-muted hover:text-text">
              Back to project
            </Link>
            <button
              onClick={() => setShowCreate(true)}
              className="px-4 py-1.5 bg-accent hover:bg-accent-hover text-white rounded-lg text-sm font-medium transition-colors"
            >
              New key
            </button>
          </div>
        </div>

        {newKey && (
          <div className="mb-6 p-4 bg-green/10 border border-green/20 rounded-xl">
            <p className="text-sm text-green font-medium mb-2">Key created! Copy it now — it won't be shown again.</p>
            <code className="block p-3 bg-bg rounded-lg font-mono text-sm break-all select-all">{newKey}</code>
            <button onClick={() => setNewKey(null)} className="mt-2 text-xs text-text-muted hover:text-text">Dismiss</button>
          </div>
        )}

        {showCreate && (
          <form onSubmit={handleCreate} className="mb-6 p-4 bg-bg-card border border-border rounded-xl flex gap-3">
            <input
              type="text"
              placeholder="Key name (e.g. CI/CD)"
              value={name}
              onChange={e => setName(e.target.value)}
              required
              className="flex-1 px-3 py-2 bg-bg border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent text-sm"
            />
            <button type="submit" className="px-4 py-2 bg-accent hover:bg-accent-hover text-white rounded-lg text-sm font-medium">
              Create
            </button>
            <button type="button" onClick={() => setShowCreate(false)} className="px-3 py-2 text-text-muted hover:text-text text-sm">
              Cancel
            </button>
          </form>
        )}

        <div className="space-y-2">
          {keys.map(k => (
            <div key={k.id} className="flex items-center justify-between p-4 bg-bg-card border border-border rounded-lg">
              <div>
                <div className="font-medium text-sm">{k.name}</div>
                <div className="text-xs text-text-muted font-mono mt-1">{k.key_prefix}</div>
              </div>
              <div className="flex items-center gap-4">
                <span className="text-xs text-text-muted">{new Date(k.created_at).toLocaleDateString()}</span>
                {k.revoked_at ? (
                  <span className="text-xs text-red">Revoked</span>
                ) : (
                  <button onClick={() => handleRevoke(k.id)} className="text-xs text-red hover:underline">
                    Revoke
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
