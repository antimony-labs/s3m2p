# Project Template

Create a new project by copying this structure:

```
projects/
└── your-project-name/
    ├── project.json       # Project metadata (REQUIRED)
    ├── README.md          # Project documentation
    └── ...                 # Your project files
```

## project.json Template

```json
{
  "name": "Your Project Name",
  "description": "Brief description of your project",
  "category": "web|python|ai|mobile|game|tool|other",
  "language": "JavaScript|Python|TypeScript|Java|etc",
  "visibility": "public|private|invite_only",
  "invite_code": null,
  "featured": false,
  "tags": ["tag1", "tag2"],
  "live_url": "https://yourproject.com",
  "github_url": null,
  "thumbnail": "thumbnail.png",
  "created": "2024-01-01",
  "updated": "2024-11-06"
}
```

## Categories

- `web` - Web applications
- `python` - Python projects
- `ai` - AI/ML projects
- `mobile` - Mobile apps
- `game` - Games
- `tool` - Utility tools
- `other` - Other projects

## Visibility

- `public` - Visible to everyone
- `private` - Visible only to authenticated users
- `invite_only` - Requires invite code

## Example Project Structure

```
projects/
└── my-awesome-app/
    ├── project.json
    ├── README.md
    ├── src/
    │   ├── index.js
    │   └── components/
    ├── package.json
    └── public/
```

## Quick Start

1. Create project directory:
   ```bash
   mkdir -p projects/my-project
   cd projects/my-project
   ```

2. Create project.json:
   ```bash
   cp ../../project.json.example project.json
   # Edit project.json with your project details
   ```

3. Add README:
   ```bash
   echo "# My Project" > README.md
   ```

4. Add your code files

5. The portfolio website will automatically discover it!

