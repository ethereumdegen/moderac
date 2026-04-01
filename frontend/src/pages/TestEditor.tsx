import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router'
import { api } from '../lib/api'
import SiteHeader from '../components/SiteHeader'

export default function TestEditor() {
  const { id: projectId, testId } = useParams()
  const navigate = useNavigate()
  const isNew = !testId
  const [name, setName] = useState('')
  const [prompt, setPrompt] = useState('')
  const [expected, setExpected] = useState('')
  const [saving, setSaving] = useState(false)
  const [user, setUser] = useState<{ id: string; email: string } | null>(null)

  useEffect(() => {
    api.me().then(setUser)
    if (testId && projectId) {
      api.getTest(projectId, testId).then(t => {
        setName(t.name)
        setPrompt(t.prompt)
        setExpected(t.expected || '')
      })
    }
  }, [projectId, testId])

  async function handleSave(e: React.FormEvent) {
    e.preventDefault()
    if (!projectId) return
    setSaving(true)
    try {
      if (isNew) {
        await api.createTest(projectId, { name, prompt, expected: expected || undefined })
      } else {
        await api.updateTest(projectId, testId!, { name, prompt, expected: expected || undefined })
      }
      navigate(`/projects/${projectId}`)
    } finally {
      setSaving(false)
    }
  }

  async function handleDelete() {
    if (!projectId || !testId) return
    if (!confirm('Delete this test?')) return
    await api.deleteTest(projectId, testId)
    navigate(`/projects/${projectId}`)
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader user={user} />
      <div className="flex-1 max-w-3xl mx-auto w-full px-6 py-10">
        <h1 className="text-2xl font-bold mb-8">{isNew ? 'New test' : 'Edit test'}</h1>
        <form onSubmit={handleSave} className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-text-muted mb-2">Name</label>
            <input
              type="text"
              value={name}
              onChange={e => setName(e.target.value)}
              required
              placeholder="e.g. User signup flow"
              className="w-full px-4 py-2 bg-bg-card border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-text-muted mb-2">Prompt</label>
            <textarea
              value={prompt}
              onChange={e => setPrompt(e.target.value)}
              required
              rows={4}
              placeholder="Describe what to test..."
              className="w-full px-4 py-3 bg-bg-card border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent font-mono text-sm resize-y"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-text-muted mb-2">Expected behavior</label>
            <textarea
              value={expected}
              onChange={e => setExpected(e.target.value)}
              rows={3}
              placeholder="Describe what should happen..."
              className="w-full px-4 py-3 bg-bg-card border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent text-sm resize-y"
            />
          </div>
          <div className="flex items-center justify-between pt-4">
            <div className="flex gap-3">
              <button
                type="submit"
                disabled={saving}
                className="px-5 py-2 bg-accent hover:bg-accent-hover disabled:opacity-50 text-white rounded-lg text-sm font-medium transition-colors"
              >
                {saving ? 'Saving...' : isNew ? 'Create test' : 'Save'}
              </button>
              <button
                type="button"
                onClick={() => navigate(`/projects/${projectId}`)}
                className="px-5 py-2 text-text-muted hover:text-text text-sm"
              >
                Cancel
              </button>
            </div>
            {!isNew && (
              <button
                type="button"
                onClick={handleDelete}
                className="px-4 py-2 text-red hover:underline text-sm"
              >
                Delete
              </button>
            )}
          </div>
        </form>
      </div>
    </div>
  )
}
