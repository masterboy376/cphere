// NotificationCard.tsx
import { UserIcon, VideoCameraIcon } from '@heroicons/react/24/outline'
import { NotificationSummaryType } from '../../contexts/NotificationContext'

interface NotificationCardProps {
  notification: NotificationSummaryType
  onAccept: (notification: NotificationSummaryType) => void
  onDecline: (notification: NotificationSummaryType) => void
}

export const NotificationCard = ({
  notification,
  onAccept,
  onDecline
}: NotificationCardProps) => {
  return (
    <div className="p-4 border-b border-background-lite">
      <div className="flex items-start gap-4">
        <div className="flex-shrink-0">
          {notification.type === 'friend-request' ? (
            <UserIcon className="h-6 w-6" />
          ) : (
            <VideoCameraIcon className="h-6 w-6" />
          )}
        </div>

        <div className="flex-1">
          <div className="text-sm text-text-primary">
            {notification.type === 'video_call' && 
              <>
                <span className="font-medium">{notification.senderUserame}</span>
                {notification.message && (
                  <p className="mt-1 text-text-secondary">{notification.message}</p>
                )}
              </>
            }
          </div>
          
          <div className="flex items-center gap-2 mt-3">
            <button
              onClick={() => onAccept(notification)}
              className="px-3 py-1.5 bg-primary/10 text-primary rounded-lg hover:bg-primary/20 transition-colors"
            >
              Accept
            </button>
            <button
              onClick={() => onDecline(notification)}
              className="px-3 py-1.5 bg-red-500/10 text-red-500 rounded-lg hover:bg-red-500/20 transition-colors"
            >
              Decline
            </button>
          </div>
          
          <div className="mt-2 text-xs text-text-secondary">
            {new Date(notification.timestamp).toLocaleTimeString([], {
              hour: '2-digit',
              minute: '2-digit'
            })}
          </div>
        </div>
      </div>
    </div>
  )
}