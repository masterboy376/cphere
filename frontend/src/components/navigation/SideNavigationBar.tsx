import { FC } from 'react'
import { Link } from 'react-router-dom'
import { PlusIcon, ChatBubbleOvalLeftIcon } from '@heroicons/react/20/solid'

import { useNavigation } from '../../contexts/NavigationContext'

export const SideNavigationBar: FC = () => {
  const { isSidebarOpen, toggleSidebar } = useNavigation()

  return (
    <>
      {/* Sidebar */}
      <aside className={`fixed top-0 h-screen bg-background-paper
        transition-all duration-300 ease-in-out z-40 w-64 
        ${isSidebarOpen ? 'left-0' : '-left-full'}
        `}>
        <div className="flex flex-col h-full pt-18 p-4">

          {/* Navigation Buttons */}
          <nav className="space-y-4">
            <Link
              to="/search"
              onClick={toggleSidebar}
              className="flex items-center gap-3 p-3 hover:bg-background-lite rounded-lg transition-colors"
            >
              <PlusIcon className="h-6 w-6" />
              <span className="text-lg">New Chat</span>
            </Link>
            
            <Link
              to="/"
              onClick={toggleSidebar}
              className="flex items-center gap-3 p-3 hover:bg-background-lite rounded-lg transition-colors"
            >
              <ChatBubbleOvalLeftIcon className="h-6 w-6" />
              <span className="text-lg">Chats</span>
            </Link>
          </nav>
        </div>
      </aside>
    </>
  )
}