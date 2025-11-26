import { GetServerSideProps } from 'next'
import Head from 'next/head'
import Link from 'next/link'
import { ProjectScanner } from '../../lib/project_scanner'
import path from 'path'
import ReactMarkdown from 'react-markdown'

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
  readme?: string
  structure?: string[]
}

type ProjectPageProps = {
  project: Project
}

export default function ProjectPage({ project }: ProjectPageProps) {
  return (
    <div className="min-h-screen bg-white text-gray-900">
      <Head>
        <title>{project.name} - too.foo</title>
        <meta name="description" content={project.description} />
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

      <main className="container mx-auto px-6 py-12 max-w-4xl">
        <Link href="/" className="text-gray-600 hover:text-gray-900 mb-8 inline-block">
          ← Back
        </Link>

        <div className="mb-8">
          <h1 className="text-4xl font-bold mb-4">{project.name}</h1>
          <p className="text-xl text-gray-600 mb-6">{project.description}</p>
          
          <div className="flex flex-wrap gap-2 mb-6">
            <span className="px-3 py-1 bg-gray-100 rounded text-sm capitalize">
              {project.category}
            </span>
            <span className="px-3 py-1 bg-gray-100 rounded text-sm">
              {project.language || 'N/A'}
            </span>
            {project.tags.map((tag) => (
              <span
                key={tag}
                className="px-3 py-1 bg-gray-100 rounded text-sm"
              >
                {tag}
              </span>
            ))}
          </div>

          <div className="flex flex-wrap gap-4">
            {project.live_url && (
              <a
                href={project.live_url}
                target="_blank"
                rel="noopener noreferrer"
                className="px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800"
              >
                Live Demo
              </a>
            )}
            {project.github_url && (
              <a
                href={project.github_url}
                target="_blank"
                rel="noopener noreferrer"
                className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
              >
                GitHub
              </a>
            )}
          </div>
        </div>

        {project.structure && project.structure.length > 0 && (
          <div className="mb-8">
            <h2 className="text-2xl font-bold mb-4">Project Structure</h2>
            <div className="bg-gray-50 rounded p-4 font-mono text-sm border border-gray-200">
              {project.structure.map((item, i) => (
                <div key={i} className="text-gray-700 py-1">
                  {item}
                </div>
              ))}
            </div>
          </div>
        )}

        {project.readme && (
          <div>
            <h2 className="text-2xl font-bold mb-4">Documentation</h2>
            <div className="prose max-w-none border border-gray-200 rounded p-6">
              <ReactMarkdown>{project.readme}</ReactMarkdown>
            </div>
          </div>
        )}
      </main>
    </div>
  )
}

export const getServerSideProps: GetServerSideProps = async ({ params }) => {
  try {
    const id = params?.id as string
    
    if (!id) {
      return { notFound: true }
    }
    
    const projectsDir = path.join(process.cwd(), '..', '..', 'projects')
    const scanner = new ProjectScanner(projectsDir)
    const allProjects = scanner.scan()
    
    const project = allProjects.find(p => p.id === id)
    
    if (!project) {
      return { notFound: true }
    }
    
    return {
      props: {
        project
      }
    }
  } catch (error) {
    console.error('Error:', error)
    return { notFound: true }
  }
}
