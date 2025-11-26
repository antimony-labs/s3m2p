import { GetServerSideProps } from 'next'
import Head from 'next/head'
import Link from 'next/link'
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
}

type CategoryPageProps = {
  projects: Project[]
  category: string
}

export default function CategoryPage({ projects, category }: CategoryPageProps) {
  return (
    <div className="min-h-screen bg-white text-gray-900">
      <Head>
        <title>{category} - too.foo</title>
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
        <Link href="/" className="text-gray-600 hover:text-gray-900 mb-8 inline-block">
          ← Back
        </Link>

        <h1 className="text-4xl font-bold mb-2 capitalize">{category}</h1>
        <p className="text-gray-600 mb-8">{projects.length} projects</p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {projects.map((project) => (
            <Link key={project.id} href={`/project/${project.id}`}>
              <div className="border border-gray-200 rounded p-4 hover:border-gray-400 transition-colors">
                <h3 className="font-semibold mb-2">{project.name}</h3>
                <p className="text-sm text-gray-600 mb-3">{project.description}</p>
                <div className="flex items-center justify-between text-xs text-gray-500">
                  <span>{project.language || 'N/A'}</span>
                </div>
              </div>
            </Link>
          ))}
        </div>

        {projects.length === 0 && (
          <div className="text-center py-12 text-gray-500">
            No projects found in this category.
          </div>
        )}
      </main>
    </div>
  )
}

export const getServerSideProps: GetServerSideProps = async ({ params }) => {
  try {
    const category = params?.category as string
    
    if (!category) {
      return { notFound: true }
    }
    
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    const categoryProjects = scanner.getByCategory(allProjects, category)
    const publicProjects = scanner.getByVisibility(categoryProjects, 'public')
    
    return {
      props: {
        projects: publicProjects,
        category
      }
    }
  } catch (error) {
    console.error('Error:', error)
    return { notFound: true }
  }
}
