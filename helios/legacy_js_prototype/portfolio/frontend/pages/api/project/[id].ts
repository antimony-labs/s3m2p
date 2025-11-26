import type { NextApiRequest, NextApiResponse } from 'next'
import { ProjectScanner } from '../../../lib/project_scanner'
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
  readme?: string
  structure?: string[]
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Project | { error: string }>
) {
  try {
    const { id } = req.query
    
    if (!id || typeof id !== 'string') {
      return res.status(400).json({ error: 'Project ID required' })
    }
    
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    const project = allProjects.find(p => p.id === id)
    
    if (!project) {
      return res.status(404).json({ error: 'Project not found' })
    }
    
    res.status(200).json(project)
  } catch (error) {
    console.error('Error fetching project:', error)
    res.status(500).json({ error: 'Failed to fetch project' })
  }
}

