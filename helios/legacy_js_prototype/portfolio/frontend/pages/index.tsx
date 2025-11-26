import { GetServerSideProps } from 'next'
import Head from 'next/head'
import Link from 'next/link'
import { ProjectScanner } from '../lib/project_scanner'
import path from 'path'
import { useState } from 'react'

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

type HomeProps = {
  projects: Project[]
  categories: string[]
  featured: Project[]
}

export default function Home({ projects, categories, featured }: HomeProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)
  
  const filteredProjects = projects.filter(project => {
    const matchesSearch = !searchQuery || 
      project.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      project.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      project.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
    
    const matchesCategory = !selectedCategory || project.category === selectedCategory
    
    return matchesSearch && matchesCategory
  })

  return (
    <div className="min-h-screen bg-white text-gray-900">
      <Head>
        <title>too.foo</title>
        <meta name="description" content="Portfolio" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>

      <nav className="border-b border-gray-200">
        <div className="container mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <Link href="/" className="text-xl font-semibold">
              too.foo
            </Link>
            <div className="flex gap-6">
              <Link href="/" className="hover:text-gray-600">Home</Link>
              <Link href="/me" className="hover:text-gray-600">About</Link>
            </div>
          </div>
        </div>
      </nav>

      <main className="container mx-auto px-6 py-12 max-w-6xl">
        {/* Search */}
        <div className="mb-8">
          <input
            type="text"
            placeholder="Search projects..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:border-gray-500"
          />
        </div>

        {/* Categories */}
        <div className="mb-8 flex flex-wrap gap-2">
          <button
            onClick={() => setSelectedCategory(null)}
            className={`px-4 py-2 border rounded ${
              selectedCategory === null
                ? 'bg-gray-900 text-white border-gray-900'
                : 'bg-white border-gray-300 hover:bg-gray-50'
            }`}
          >
            All
          </button>
          {categories.map((category) => (
            <button
              key={category}
              onClick={() => setSelectedCategory(category)}
              className={`px-4 py-2 border rounded capitalize ${
                selectedCategory === category
                  ? 'bg-gray-900 text-white border-gray-900'
                  : 'bg-white border-gray-300 hover:bg-gray-50'
              }`}
            >
              {category}
            </button>
          ))}
        </div>

        {/* Projects */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredProjects.map((project) => (
            <Link key={project.id} href={`/project/${project.id}`}>
              <div className="border border-gray-200 rounded p-4 hover:border-gray-400 transition-colors">
                <h3 className="font-semibold mb-2">{project.name}</h3>
                <p className="text-sm text-gray-600 mb-3">{project.description}</p>
                <div className="flex items-center justify-between text-xs text-gray-500">
                  <span className="capitalize">{project.category}</span>
                  {project.live_url && (
                    <a
                      href={project.live_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-blue-600 hover:underline"
                      onClick={(e) => e.stopPropagation()}
                    >
                      Live
                    </a>
                  )}
                </div>
              </div>
            </Link>
          ))}
        </div>

        {filteredProjects.length === 0 && (
          <div className="text-center py-12 text-gray-500">
            No projects found
          </div>
        )}
      </main>
    </div>
  )
}

export const getServerSideProps: GetServerSideProps = async () => {
  try {
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    const publicProjects = scanner.getByVisibility(allProjects, 'public')
    const categories = [...new Set(allProjects.map(p => p.category))].sort()
    const featured = scanner.getFeatured(publicProjects)
    
    return {
      props: {
        projects: publicProjects,
        categories,
        featured
      }
    }
  } catch (error) {
    console.error('Error:', error)
    return {
      props: {
        projects: [],
        categories: [],
        featured: []
      }
    }
  }
}
