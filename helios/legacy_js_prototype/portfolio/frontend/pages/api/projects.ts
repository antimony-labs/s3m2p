import type { NextApiRequest, NextApiResponse } from 'next'
import { ProjectScanner } from '../../lib/project_scanner'
import path from 'path'

type Project = {
  id: string
  name: string
  description: string
  category: string
  language: string
  visibility: string
  featured: boolean
  tags: string[]
  live_url?: string
  github_url?: string
  thumbnail?: string
  created?: string
  updated?: string
}

type ApiResponse = {
  projects: Project[]
  categories: string[]
  total: number
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<ApiResponse | { error: string }>
) {
  try {
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    // Filter by visibility (for now, show public only)
    const visibility = req.query.visibility as string || 'public'
    const filteredProjects = scanner.getByVisibility(allProjects, visibility)
    
    // Get categories
    const categories = Array.from(new Set(allProjects.map(p => p.category))).sort()
    
    res.status(200).json({
      projects: filteredProjects,
      categories,
      total: filteredProjects.length
    })
  } catch (error) {
    console.error('Error scanning projects:', error)
    res.status(500).json({ error: 'Failed to scan projects' })
  }
}

