import type { NextApiRequest, NextApiResponse } from 'next'
import { ProjectScanner } from '../../../lib/project_scanner'
import path from 'path'

type CategoryResponse = {
  projects: any[]
  category: string
  total: number
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<CategoryResponse | { error: string }>
) {
  try {
    const { category } = req.query
    
    if (!category || typeof category !== 'string') {
      return res.status(400).json({ error: 'Category required' })
    }
    
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    const categoryProjects = scanner.getByCategory(allProjects, category)
    const publicProjects = scanner.getByVisibility(categoryProjects, 'public')
    
    res.status(200).json({
      projects: publicProjects,
      category,
      total: publicProjects.length
    })
  } catch (error) {
    console.error('Error fetching category:', error)
    res.status(500).json({ error: 'Failed to fetch category' })
  }
}

