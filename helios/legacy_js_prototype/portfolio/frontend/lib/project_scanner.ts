import fs from 'fs'
import path from 'path'

export type Project = {
  id: string
  path: string
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
  readme?: string
  structure?: string[]
}

export class ProjectScanner {
  private projectsDir: string

  constructor(projectsDir: string = 'projects') {
    this.projectsDir = projectsDir
  }

  scan(): Project[] {
    if (!fs.existsSync(this.projectsDir)) {
      return []
    }

    const projects: Project[] = []
    this.scanDirectory(this.projectsDir, projects)
    return projects
  }

  private scanDirectory(dir: string, projects: Project[]): void {
    try {
      const entries = fs.readdirSync(dir, { withFileTypes: true })

      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name)

        if (entry.isDirectory()) {
          // Check if this directory has a project.json
          const projectJsonPath = path.join(fullPath, 'project.json')
          if (fs.existsSync(projectJsonPath)) {
            const project = this.loadProject(fullPath)
            if (project) {
              projects.push(project)
            }
          } else {
            // Recursively scan subdirectories
            this.scanDirectory(fullPath, projects)
          }
        }
      }
    } catch (error) {
      console.error(`Error scanning directory ${dir}:`, error)
    }
  }

  private loadProject(projectDir: string): Project | null {
    try {
      const projectJsonPath = path.join(projectDir, 'project.json')
      const metadata = JSON.parse(fs.readFileSync(projectJsonPath, 'utf-8'))

      const relativePath = path.relative(this.projectsDir, projectDir)
      const projectDirPath = path.dirname(projectDir)

      return {
        id: relativePath.replace(/\\/g, '/'), // Normalize path separators
        path: projectDir,
        name: metadata.name || path.basename(projectDir),
        description: metadata.description || '',
        category: metadata.category || 'uncategorized',
        language: metadata.language || '',
        visibility: metadata.visibility || 'public',
        featured: metadata.featured || false,
        tags: metadata.tags || [],
        live_url: metadata.live_url,
        github_url: metadata.github_url,
        thumbnail: metadata.thumbnail,
        created: metadata.created,
        updated: metadata.updated,
        readme: this.findReadme(projectDir),
        structure: this.getStructure(projectDir)
      }
    } catch (error) {
      console.error(`Error loading project from ${projectDir}:`, error)
      return null
    }
  }

  private findReadme(projectDir: string): string | undefined {
    const readmeNames = ['README.md', 'readme.md', 'Readme.md']
    for (const readmeName of readmeNames) {
      const readmePath = path.join(projectDir, readmeName)
      if (fs.existsSync(readmePath)) {
        try {
          return fs.readFileSync(readmePath, 'utf-8')
        } catch {
          return undefined
        }
      }
    }
    return undefined
  }

  private getStructure(projectDir: string): string[] {
    try {
      const entries = fs.readdirSync(projectDir)
      return entries
        .filter(entry => !entry.startsWith('.'))
        .map(entry => {
          const fullPath = path.join(projectDir, entry)
          const stat = fs.statSync(fullPath)
          return stat.isDirectory() ? `${entry}/` : entry
        })
        .sort()
    } catch {
      return []
    }
  }

  getByCategory(projects: Project[], category: string): Project[] {
    return projects.filter(p => p.category === category)
  }

  getByVisibility(projects: Project[], visibility: string): Project[] {
    return projects.filter(p => p.visibility === visibility)
  }

  getFeatured(projects: Project[]): Project[] {
    return projects.filter(p => p.featured === true)
  }

  search(projects: Project[], query: string): Project[] {
    const queryLower = query.toLowerCase()
    return projects.filter(project => 
      project.name.toLowerCase().includes(queryLower) ||
      project.description.toLowerCase().includes(queryLower) ||
      project.tags.some(tag => tag.toLowerCase().includes(queryLower))
    )
  }
}

