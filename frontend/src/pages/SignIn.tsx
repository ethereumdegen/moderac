import { useState } from 'react'
import { useNavigate } from 'react-router'
import { api } from '../lib/api'
import SiteHeader from '../components/SiteHeader'

export default function SignIn() {
  const [email, setEmail] = useState('')
  const [code, setCode] = useState('')
  const [step, setStep] = useState<'email' | 'code'>('email')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate()

  async function handleSendCode(e: React.FormEvent) {
    e.preventDefault()
    setLoading(true)
    setError('')
    try {
      await api.login(email)
      setStep('code')
    } catch {
      setError('Failed to send code')
    } finally {
      setLoading(false)
    }
  }

  async function handleVerify(e: React.FormEvent) {
    e.preventDefault()
    setLoading(true)
    setError('')
    try {
      await api.verify(email, code)
      navigate('/dashboard')
    } catch {
      setError('Invalid code')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex flex-col">
      <SiteHeader />
      <div className="flex-1 flex items-center justify-center px-6">
        <div className="w-full max-w-sm">
          <h1 className="text-2xl font-bold mb-8 text-center">Sign in to Moderac</h1>
          {step === 'email' ? (
            <form onSubmit={handleSendCode} className="space-y-4">
              <input
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={e => setEmail(e.target.value)}
                required
                className="w-full px-4 py-3 bg-bg-card border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent"
              />
              <button
                type="submit"
                disabled={loading}
                className="w-full py-3 bg-accent hover:bg-accent-hover disabled:opacity-50 text-white rounded-lg font-medium transition-colors"
              >
                {loading ? 'Sending...' : 'Send code'}
              </button>
            </form>
          ) : (
            <form onSubmit={handleVerify} className="space-y-4">
              <p className="text-sm text-text-muted text-center mb-4">Enter the code sent to {email}</p>
              <input
                type="text"
                placeholder="000000"
                value={code}
                onChange={e => setCode(e.target.value)}
                required
                maxLength={6}
                className="w-full px-4 py-3 bg-bg-card border border-border rounded-lg text-text placeholder:text-text-muted focus:outline-none focus:border-accent text-center text-2xl tracking-widest font-mono"
              />
              <button
                type="submit"
                disabled={loading}
                className="w-full py-3 bg-accent hover:bg-accent-hover disabled:opacity-50 text-white rounded-lg font-medium transition-colors"
              >
                {loading ? 'Verifying...' : 'Verify'}
              </button>
              <button type="button" onClick={() => setStep('email')} className="w-full text-sm text-text-muted hover:text-text">
                Use a different email
              </button>
            </form>
          )}
          {error && <p className="text-red text-sm text-center mt-4">{error}</p>}
        </div>
      </div>
    </div>
  )
}
