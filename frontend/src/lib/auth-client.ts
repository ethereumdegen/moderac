import { useState, useEffect } from 'react'

export const authClient = {
  emailOtp: {
    async sendVerificationOtp({ email }: { email: string; type?: string }) {
      const res = await fetch('/api/auth/send-otp', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ email }),
      })
      if (!res.ok) {
        const data = await res.json().catch(() => ({}))
        return { error: { message: data.error || 'Failed to send code' } }
      }
      return res.json()
    },
    async verifyEmail({ email, otp }: { email: string; otp: string }) {
      const res = await fetch('/api/auth/verify-otp', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ email, code: otp }),
      })
      const data = await res.json().catch(() => ({}))
      if (!res.ok) {
        return { error: { message: data.error || 'Verification failed' } }
      }
      return { data, error: null }
    },
  },
}

export async function signOut() {
  await fetch('/api/auth/sign-out', {
    method: 'POST',
    credentials: 'include',
  })
  window.location.href = '/'
}

export function useSession() {
  const [data, setData] = useState<{ user: { email: string } } | null>(null)
  const [isPending, setIsPending] = useState(true)

  useEffect(() => {
    fetch('/api/auth/session', { credentials: 'include' })
      .then(res => {
        if (!res.ok) return null
        return res.json()
      })
      .then(session => {
        setData(session)
        setIsPending(false)
      })
      .catch(() => {
        setData(null)
        setIsPending(false)
      })
  }, [])

  return { data, isPending }
}
