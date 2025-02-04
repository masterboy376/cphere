// NotificationDrawer.tsx
import { XMarkIcon } from '@heroicons/react/24/outline'
import { NotificationCard } from './NotificationCard'
import { Notification } from './NotificationCard'

export const NotificationDrawer = ({
  isOpen,
  onClose,
  notifications
}: {
  isOpen: boolean
  onClose: () => void
  notifications: Notification[]
}) => {
  return (
    <div
      className={`fixed inset-y-0 right-0 w-full max-w-md bg-background-paper border-l
         border-background-lite transform transition-transform duration-300 ease-in-out z-50 
         ${isOpen ? 'translate-x-0' : 'translate-x-full'}`}
    >
      <div className="flex items-center justify-between p-4 border-b border-background-lite">
        <h2 className="text-lg font-medium text-text-primary">Notifications</h2>
        <button
          onClick={onClose}
          className="text-text-secondary hover:bg-background-lite p-2 transition-all duration-300 ease-in-out 
          rounded-full"
        >
          <XMarkIcon className="h-6 w-6" />
        </button>
      </div>

      <div className="overflow-y-auto h-[calc(100vh-4rem)]">
        {notifications.length === 0 ? (
          <div className="p-4 text-center text-text-secondary">No notifications</div>
        ) : (
          notifications.map(notification => (
            <NotificationCard
              key={notification.id}
              notification={notification}
            //   onAccept={() => handleAccept(notification)}
              onAccept={() => console.log('accept')}
              onDecline={() => console.log('decline')}
            //   onDecline={() => handleDecline(notification)}
            />
          ))
        )}
      </div>
    </div>
  )
}
