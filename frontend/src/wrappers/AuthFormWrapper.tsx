import { ReactNode } from 'react'

export const AuthFormWrapper = ({ 
  title, 
  subtitle,
  children 
}: { 
  title: string
  subtitle: string
  children: ReactNode
}) => {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="w-full max-w-md bg-background-paper p-8 rounded-xl border border-secondary/50">
        <div className="flex justify-center mb-6">
          <img src="/logo.svg" alt="cphere" className='h-12 w-12' />
        </div>
        
        <h1 className="text-2xl font-bold text-center mb-2 text-primary">{title}</h1>
        <p className="text-center text-text-secondary mb-8">{subtitle}</p>

        {children}

      </div>
    </div>
  )
}