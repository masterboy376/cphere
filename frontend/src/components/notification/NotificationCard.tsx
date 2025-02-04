// NotificationCard.tsx
import { UserIcon, VideoCameraIcon } from '@heroicons/react/24/outline'

type NotificationBase = {
  id: string
  timestamp: Date
  read: boolean
}

type FriendRequestNotification = NotificationBase & {
  type: 'friend-request'
  senderName: string
  senderId: string
  message?: string
}

type VideoCallNotification = NotificationBase & {
  type: 'video-call'
  callerName: string
  callerId: string
  callType: 'audio' | 'video'
}

export type Notification = FriendRequestNotification | VideoCallNotification

export const NotificationCard = ({
  notification,
  onAccept,
  onDecline
}: {
  notification: Notification
  onAccept: () => void
  onDecline: () => void
}) => {
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
            {notification.type === 'friend-request' ? (
              <>
                <span className="font-medium">{notification.senderName}</span> sent you a friend request
                {notification.message && (
                  <p className="mt-1 text-text-secondary">{notification.message}</p>
                )}
              </>
            ) : (
              <>
                <span className="font-medium">{notification.callerName}</span> is requesting a{' '}
                {notification.callType} call
              </>
            )}
          </div>
          
          <div className="flex items-center gap-2 mt-3">
            <button
              onClick={onAccept}
              className="px-3 py-1.5 bg-primary/10 text-primary rounded-lg hover:bg-primary/20 transition-colors"
            >
              {notification.type === 'friend-request' ? 'Accept' : 'Join'}
            </button>
            <button
              onClick={onDecline}
              className="px-3 py-1.5 bg-red-500/10 text-red-500 rounded-lg hover:bg-red-500/20 transition-colors"
            >
              {notification.type === 'friend-request' ? 'Decline' : 'Ignore'}
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