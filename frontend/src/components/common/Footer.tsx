import { FC } from 'react'

export const Footer: FC = () => {
  return (
    <footer className="bg-background py-4 px-6">
      <div className="flex items-center justify-between text-sm text-text-secondary">
        <span>© {new Date().getFullYear()} Cphere</span>
        <div className="flex gap-4">
          <a href="#" className="hover:text-primary transition-colors">Privacy</a>
          <a href="#" className="hover:text-primary transition-colors">Terms</a>
          <a href="#" className="hover:text-primary transition-colors">Help</a>
        </div>
      </div>
    </footer>
  )
}