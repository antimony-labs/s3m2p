#!/usr/bin/env python3
"""
Project Scanner - Scans the /projects directory and builds project index
"""

import os
import json
from pathlib import Path
from typing import List, Dict, Optional
from datetime import datetime

class ProjectScanner:
    def __init__(self, projects_dir: str = "projects"):
        self.projects_dir = Path(projects_dir)
        self.projects = []
    
    def scan(self) -> List[Dict]:
        """Scan projects directory and return all projects"""
        if not self.projects_dir.exists():
            return []
        
        projects = []
        for project_path in self.projects_dir.rglob("project.json"):
            project_dir = project_path.parent
            metadata = self._load_metadata(project_path)
            
            if metadata:
                project = {
                    "id": str(project_dir.relative_to(self.projects_dir)),
                    "path": str(project_dir),
                    "name": metadata.get("name", project_dir.name),
                    "description": metadata.get("description", ""),
                    "category": metadata.get("category", "uncategorized"),
                    "language": metadata.get("language", ""),
                    "visibility": metadata.get("visibility", "public"),
                    "featured": metadata.get("featured", False),
                    "tags": metadata.get("tags", []),
                    "live_url": metadata.get("live_url"),
                    "github_url": metadata.get("github_url"),
                    "thumbnail": metadata.get("thumbnail"),
                    "created": metadata.get("created"),
                    "updated": metadata.get("updated"),
                    "readme": self._find_readme(project_dir),
                    "structure": self._get_structure(project_dir)
                }
                projects.append(project)
        
        return projects
    
    def _load_metadata(self, json_path: Path) -> Optional[Dict]:
        """Load project.json metadata"""
        try:
            with open(json_path, 'r') as f:
                return json.load(f)
        except Exception as e:
            print(f"Error loading {json_path}: {e}")
            return None
    
    def _find_readme(self, project_dir: Path) -> Optional[str]:
        """Find README file in project directory"""
        for readme_name in ["README.md", "readme.md", "Readme.md"]:
            readme_path = project_dir / readme_name
            if readme_path.exists():
                return readme_path.read_text(encoding='utf-8', errors='ignore')
        return None
    
    def _get_structure(self, project_dir: Path) -> List[str]:
        """Get project file structure (first level only)"""
        structure = []
        try:
            for item in project_dir.iterdir():
                if item.is_dir():
                    structure.append(f"{item.name}/")
                elif item.is_file() and not item.name.startswith('.'):
                    structure.append(item.name)
        except Exception:
            pass
        return sorted(structure)
    
    def get_by_category(self, projects: List[Dict], category: str) -> List[Dict]:
        """Filter projects by category"""
        return [p for p in projects if p.get("category") == category]
    
    def get_by_visibility(self, projects: List[Dict], visibility: str) -> List[Dict]:
        """Filter projects by visibility"""
        return [p for p in projects if p.get("visibility") == visibility]
    
    def get_featured(self, projects: List[Dict]) -> List[Dict]:
        """Get featured projects"""
        return [p for p in projects if p.get("featured", False)]
    
    def search(self, projects: List[Dict], query: str) -> List[Dict]:
        """Search projects by name, description, or tags"""
        query_lower = query.lower()
        results = []
        for project in projects:
            if (query_lower in project.get("name", "").lower() or
                query_lower in project.get("description", "").lower() or
                any(query_lower in tag.lower() for tag in project.get("tags", []))):
                results.append(project)
        return results

def main():
    """Example usage"""
    scanner = ProjectScanner("projects")
    
    print("Scanning projects...")
    all_projects = scanner.scan()
    
    print(f"\nFound {len(all_projects)} projects")
    
    # Categories
    categories = set(p.get("category") for p in all_projects)
    print(f"\nCategories: {', '.join(categories)}")
    
    # Public projects
    public = scanner.get_by_visibility(all_projects, "public")
    print(f"\nPublic projects: {len(public)}")
    
    # Featured
    featured = scanner.get_featured(all_projects)
    print(f"Featured projects: {len(featured)}")
    
    # Example: Show first project
    if all_projects:
        print(f"\nExample project:")
        project = all_projects[0]
        print(f"  Name: {project['name']}")
        print(f"  Category: {project['category']}")
        print(f"  Visibility: {project['visibility']}")

if __name__ == "__main__":
    main()

