import { FC, useState } from 'react'
import { Bars3Icon, BellIcon } from '@heroicons/react/24/outline'
import { useNavigation } from '../../contexts/NavigationContext'

import { NotificationDrawer } from '../notification/NotificationDrawer'
import LogoutButton from '../button/LogoutButton'

export const TopNavigationBar: FC = () => {
  const { toggleSidebar } = useNavigation()
  const [isNotificationsOpen, setIsNotificationsOpen] = useState(false)

  return (
    <header className="sticky top-0 bg-background-paper z-50">
      <nav className="p-4 h-16 flex items-center justify-between">
        {/* Toggle Button */}
        <button
          onClick={toggleSidebar}
          className="hover:bg-background-lite transition-color rounded-full p-2"
        >
          <Bars3Icon className="h-6 w-6" />
        </button>

        {/* App Logo */}
        <div className="flex items-center gap-2">
          <img src="/logo.svg" alt="cphere" className='h-12 w-12' />
          <span className="text-xl font-bold text-primary">Cphere</span>
        </div>

        {/* Notifications and logout */}
        <div className="flex items-center gap-2">
          <LogoutButton />
          <button
            onClick={() => setIsNotificationsOpen(true)}
            className="hover:bg-background-lite transition-color rounded-full p-2"
          >
            <BellIcon className="h-6 w-6" />
          </button>

          <NotificationDrawer
            isOpen={isNotificationsOpen}
            onClose={() => setIsNotificationsOpen(false)}
          />
        </div>
      </nav>
    </header>
  )
}