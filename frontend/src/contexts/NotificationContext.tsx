import { createContext, useContext, useState } from 'react'

type NotificationSummaryType = {
    id: string,
    type: string,
    senderUserame: string,
    senderId: string,
    timestamp: Date,
    message: string
}

type NotificationSummaryBackendType = {
    id: string,
    type: string,
    sender_userame: string,
    sender_id: string,
    timestamp: Date,
    message: string
}

type NotificationContextType = {
    notifications: NotificationSummaryType[]
    setNotifications: (notifications: NotificationSummaryType[]) => void
    addNotification: (notification: NotificationSummaryType) => void
    removeNotification: (notificationId: string) => void
    toFrontendNotification: (notification: NotificationSummaryBackendType) => NotificationSummaryType
}

const NotificationContext = createContext<NotificationContextType>({
    notifications: [],
    addNotification: () => { },
    setNotifications: () => { },
    removeNotification: () => { },
    toFrontendNotification: () => ({
        id: '',
        type: '',
        senderUserame: '',
        senderId: '',
        timestamp: new Date(),
        message: ''
    })
})

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [notifications, setNotifications] = useState<NotificationSummaryType[]>([])

    const addNotification = (Notification: NotificationSummaryType) => {
        setNotifications(prevNotifications => [Notification, ...prevNotifications]);
    }

    const removeNotification = (notificationId: string) => {
        setNotifications(notifications => notifications.filter(
            notification => notification.id !== notificationId));
    }

    const toFrontendNotification = (notification: NotificationSummaryBackendType): NotificationSummaryType => ({
        id: notification.id,
        type: notification.type,
        senderUserame: notification.sender_userame,
        senderId: notification.sender_id,
        timestamp: notification.timestamp,
        message: notification.message
    })

    return (
        <NotificationContext.Provider value={{ notifications, addNotification, setNotifications, removeNotification, toFrontendNotification }}>
            {children}
        </NotificationContext.Provider>
    )
}

export const useNotification = () => useContext(NotificationContext)
export type { NotificationSummaryType, NotificationSummaryBackendType }