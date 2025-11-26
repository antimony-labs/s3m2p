import Head from 'next/head'
import Link from 'next/link'

export default function About() {
  return (
    <div className="min-h-screen bg-white text-gray-900">
      <Head>
        <title>About Me - too.foo</title>
        <meta name="description" content="About me" />
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
              <Link href="/me" className="text-gray-900 font-medium">About</Link>
            </div>
          </div>
        </div>
      </nav>

      <main className="container mx-auto px-6 py-16 max-w-2xl">
        <h1 className="text-4xl font-bold mb-8">About Me</h1>
        
        <div className="prose prose-lg">
          <p className="text-gray-600 leading-relaxed mb-6">
            Write your about me content here.
          </p>
        </div>
      </main>
    </div>
  )
}

