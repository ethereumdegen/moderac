const BASE = '/api'

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    ...options,
  })
  if (!res.ok) {
    if (res.status === 401) {
      window.location.href = '/signin'
    }
    throw new Error(`${res.status} ${res.statusText}`)
  }
  if (res.status === 204) return undefined as T
  return res.json()
}

export interface Features {
  auth: boolean
  eval: boolean
}

let featuresCache: Features | null = null

export async function getFeatures(): Promise<Features> {
  if (featuresCache) return featuresCache
  try {
    const res = await fetch('/api/features')
    featuresCache = await res.json()
    return featuresCache!
  } catch {
    return { auth: false, eval: false }
  }
}

export const api = {
  // Projects
  listProjects: () => request<Project[]>('/projects'),
  createProject: (data: { name: string; description?: string; base_url?: string }) => request<Project>('/projects', { method: 'POST', body: JSON.stringify(data) }),
  getProject: (id: string) => request<Project>(`/projects/${id}`),
  updateProject: (id: string, data: Partial<Project>) => request<Project>(`/projects/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteProject: (id: string) => request(`/projects/${id}`, { method: 'DELETE' }),

  // Tests
  listTests: (projectId: string) => request<Test[]>(`/projects/${projectId}/tests`),
  createTest: (projectId: string, data: { name: string; prompt: string; expected?: string }) => request<Test>(`/projects/${projectId}/tests`, { method: 'POST', body: JSON.stringify(data) }),
  getTest: (projectId: string, testId: string) => request<Test>(`/projects/${projectId}/tests/${testId}`),
  updateTest: (projectId: string, testId: string, data: Partial<Test>) => request<Test>(`/projects/${projectId}/tests/${testId}`, { method: 'PUT', body: JSON.stringify(data) }),
  deleteTest: (projectId: string, testId: string) => request(`/projects/${projectId}/tests/${testId}`, { method: 'DELETE' }),

  // Runs
  listRuns: (projectId: string) => request<TestRun[]>(`/projects/${projectId}/runs`),
  createRun: (projectId: string) => request<TestRun>(`/projects/${projectId}/runs`, { method: 'POST' }),
  getRun: (projectId: string, runId: string) => request<RunDetail>(`/projects/${projectId}/runs/${runId}`),

  // API Keys
  listApiKeys: (projectId: string) => request<ApiKey[]>(`/projects/${projectId}/api-keys`),
  createApiKey: (projectId: string, name: string) => request<ApiKeyCreated>(`/projects/${projectId}/api-keys`, { method: 'POST', body: JSON.stringify({ name }) }),
  revokeApiKey: (projectId: string, keyId: string) => request(`/projects/${projectId}/api-keys/${keyId}`, { method: 'DELETE' }),
}

export interface Project {
  id: string
  user_id: string
  name: string
  description: string | null
  base_url: string | null
  created_at: string
}

export interface Test {
  id: string
  project_id: string
  name: string
  prompt: string
  expected: string | null
  eval_criteria: string | null
  config: string | null
  created_at: string
  updated_at: string
}

export interface TestRun {
  id: string
  project_id: string
  status: string
  started_at: string | null
  completed_at: string | null
  created_at: string
}

export interface TestResult {
  id: string
  run_id: string
  test_id: string
  status: string
  response: string | null
  evaluation: string | null
  score: number | null
  duration_ms: number | null
  created_at: string
}

export interface RunDetail {
  run: TestRun
  results: TestResult[]
}

export interface ApiKey {
  id: string
  project_id: string
  key_prefix: string
  name: string
  created_at: string
  revoked_at: string | null
}

export interface ApiKeyCreated {
  id: string
  key: string
  key_prefix: string
  name: string
}
