import { createContext, useContext, useState } from 'react'

type NavigationContextType = {
  isSidebarOpen: boolean
  toggleSidebar: () => void
}

const NavigationContext = createContext<NavigationContextType>({
  isSidebarOpen: false,
  toggleSidebar: () => {}
})

export const NavigationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true)

  const toggleSidebar = () => {
    setIsSidebarOpen(prev => !prev)
  }

  return (
    <NavigationContext.Provider value={{ isSidebarOpen, toggleSidebar }}>
      {children}
    </NavigationContext.Provider>
  )
}

export const useNavigation = () => useContext(NavigationContext)