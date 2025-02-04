import { FC, useState } from 'react'
import { Bars3Icon, BellIcon } from '@heroicons/react/24/outline'
import { useNavigation } from '../../contexts/NavigationContext'

import { NotificationDrawer } from '../notification/NotificationDrawer'

import { Notification } from '../notification/NotificationCard'

export const TopNavigationBar: FC = () => {
  const { toggleSidebar } = useNavigation()
  const [isNotificationsOpen, setIsNotificationsOpen] = useState(false)
  const [notifications, setNotifications] = useState<Notification[]>([
    {
      id: '1',
      type: 'friend-request',
      senderName: 'John Doe',
      senderId: '123',
      timestamp: new Date(),
      read: false,
      message: 'Hey, let\'s connect!'
    },
    {
      id: '2',
      type: 'video-call',
      callerName: 'Jane Smith',
      callerId: '456',
      callType: 'video',
      timestamp: new Date(),
      read: false
    }
  ])

  const handleAccept = (notification: Notification) => {
    // Handle accept logic
  }

  const handleDecline = (notification: Notification) => {
    // Handle decline logic
  }

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
          <div className="w-8 h-8 bg-primary rounded-full" />
          <span className="text-xl font-bold text-primary">Cphere</span>
        </div>

        {/* Notifications */}
        <button
          onClick={() => setIsNotificationsOpen(true)}
          className="hover:bg-background-lite transition-color rounded-full p-2"
        >
          <BellIcon className="h-6 w-6" />
        </button>

        <NotificationDrawer
          isOpen={isNotificationsOpen}
          onClose={() => setIsNotificationsOpen(false)}
          notifications={notifications}
        />
      </nav>
    </header>
  )
}