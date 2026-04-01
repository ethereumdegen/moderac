import { Link, useNavigate } from 'react-router'
import { api } from '../lib/api'

export default function SiteHeader({ user }: { user?: { email: string } | null }) {
  const navigate = useNavigate()

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
              onClick={async () => { await api.logout(); navigate('/') }}
              className="text-text-muted hover:text-text transition-colors"
            >
              Sign out
            </button>
          </>
        ) : (
          <Link to="/signin" className="text-text-muted hover:text-text transition-colors">Sign in</Link>
        )}
      </nav>
    </header>
  )
}
