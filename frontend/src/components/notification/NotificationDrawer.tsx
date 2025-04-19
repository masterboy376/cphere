// NotificationDrawer.tsx
import { XMarkIcon } from '@heroicons/react/24/outline'
import { NotificationCard } from './NotificationCard'
import { NotificationSummaryType, useNotification } from '../../contexts/NotificationContext'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import userBackendApiService from '../../services/user/UserBackendApiService'
import { VideoCallRequest } from '../../types/WsMessageTypes'
import wsService from '../../services/ws/WsService'
import { Loader } from '../common/Loader'
import videoBackendApiService, { VideoRespondPayload } from '../../services/video/VideoBackendApiService'

interface NotificationDrawerProps {
  isOpen: boolean
  onClose: () => void
}
export const NotificationDrawer = ({
  isOpen,
  onClose
}: NotificationDrawerProps) => {
  const { notifications, setNotifications, addNotification, removeNotification, toFrontendNotification } = useNotification()
  const [loading, setLoading] = useState(false)
  const navigate = useNavigate()

  const onAccept = (notification: NotificationSummaryType) => {
    const payload: VideoRespondPayload = {
      notification_id: notification.id,
      accepted: true
    }
    videoBackendApiService.respond(payload);
    removeNotification(notification.id)
    navigate(`/video-call/${notification.senderId}`) 
  }

  const onDecline = (notification: NotificationSummaryType) => {
    const payload: VideoRespondPayload = {
      notification_id: notification.id,
      accepted: false
    }
    videoBackendApiService.respond(payload);
    removeNotification(notification.id)
  }

  useEffect(() => {
    const fetchNotifications = async () => {
      try {
        setLoading(true)
        setNotifications([])
        const response = await userBackendApiService.getNotifications()

        if (response && Array.isArray(response)) {
          // Transform backend data to frontend format and update context
          const frontendNotifications = response.map(chat => toFrontendNotification(chat))
          setNotifications(frontendNotifications)
        }
      } catch (error) {
        console.error('Failed to fetch notifications:', error)
      } finally {
        setLoading(false)
      }
    }

    const videoCallRequestListener = async (message: VideoCallRequest) => {
      const newNotification: NotificationSummaryType = toFrontendNotification(message.notification)
      addNotification(newNotification)
    }
    fetchNotifications()
    wsService.addEventListener('video_call_request', videoCallRequestListener)
    
    return () => {
      wsService.removeEventListener('video_call_request', videoCallRequestListener)
    }
  }, [])

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

      {loading ? (
        <div className="flex-1 overflow-y-auto min-h-0">
          <Loader message="Loading chats..." />
        </div>
      ) : (
        <div className="overflow-y-auto h-[calc(100vh-4rem)]">
          {notifications.length === 0 ? (
            <div className="p-4 text-center text-text-secondary">No notifications</div>
          ) : (
            notifications.map(notification => (
              <NotificationCard
                key={notification.id}
                notification={notification}
                onAccept={onAccept}
                onDecline={onDecline}
              />
            ))
          )}
        </div>)}
    </div>
  )
}
