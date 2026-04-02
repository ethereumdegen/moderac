import { Link } from 'react-router'
import { useEffect, useState } from 'react'
import { signOut } from '../lib/auth-client'
import { getFeatures } from '../lib/api'

export default function SiteHeader({ user }: { user?: { email: string } | null }) {
  const [authEnabled, setAuthEnabled] = useState(false)

  useEffect(() => {
    getFeatures().then(f => setAuthEnabled(f.auth))
  }, [])

  return (
    <header className="border-b border-border px-6 py-4 flex items-center justify-between">
      <Link to="/" className="text-lg font-semibold tracking-tight">
        <span className="text-accent">moderac</span>
      </Link>
      <nav className="flex items-center gap-6 text-sm">
        {user ? (
          <>
            <Link to="/dashboard" className="text-text-muted hover:text-text transition-colors">Dashboard</Link>
            <button
              onClick={() => signOut()}
              className="text-text-muted hover:text-text transition-colors"
            >
              Sign out
            </button>
          </>
        ) : authEnabled ? (
          <Link to="/signin" className="text-text-muted hover:text-text transition-colors">Sign in</Link>
        ) : null}
      </nav>
    </header>
  )
}
